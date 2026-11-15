use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc, oneshot};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::CoreResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: String,
    pub method: String,
    pub params: Value,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: String,
    pub result: Option<Value>,
    pub error: Option<IpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcNotification {
    pub method: String,
    pub params: Value,
}

pub type RequestHandler = Box<dyn Fn(Value) -> Result<Value, IpcError> + Send + Sync>;
pub type NotificationHandler = Box<dyn Fn(Value) + Send + Sync>;

pub struct IpcManager {
    request_handlers: Arc<RwLock<HashMap<String, RequestHandler>>>,
    notification_handlers: Arc<RwLock<HashMap<String, Vec<NotificationHandler>>>>,
    pending_requests: Arc<RwLock<HashMap<String, oneshot::Sender<IpcResponse>>>>,
    tx: mpsc::UnboundedSender<IpcMessage>,
    rx: Arc<RwLock<Option<mpsc::UnboundedReceiver<IpcMessage>>>>,
}

#[derive(Debug)]
enum IpcMessage {
    Request(IpcRequest, oneshot::Sender<IpcResponse>),
    Notification(IpcNotification),
    Response(IpcResponse),
    Shutdown,
}

impl IpcManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Self {
            request_handlers: Arc::new(RwLock::new(HashMap::new())),
            notification_handlers: Arc::new(RwLock::new(HashMap::new())),
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            tx,
            rx: Arc::new(RwLock::new(Some(rx))),
        }
    }

    pub fn sender(&self) -> IpcSender {
        IpcSender {
            tx: self.tx.clone(),
        }
    }

    pub async fn register_request_handler<F>(&self, method: &str, handler: F)
    where
        F: Fn(Value) -> Result<Value, IpcError> + Send + Sync + 'static,
    {
        let mut handlers = self.request_handlers.write().await;
        handlers.insert(method.to_string(), Box::new(handler));
    }

    pub async fn register_notification_handler<F>(&self, method: &str, handler: F)
    where
        F: Fn(Value) + Send + Sync + 'static,
    {
        let mut handlers = self.notification_handlers.write().await;
        handlers.entry(method.to_string()).or_default().push(Box::new(handler));
    }

    pub async fn start_processing(&self) {
        let mut rx = self.rx.write().await;
        if let Some(mut rx) = rx.take() {
            let request_handlers = self.request_handlers.clone();
            let notification_handlers = self.notification_handlers.clone();
            let pending_requests = self.pending_requests.clone();

            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    match msg {
                        IpcMessage::Request(request, responder) => {
                            let handlers = request_handlers.read().await;
                            if let Some(handler) = handlers.get(&request.method) {
                                let result = handler(request.params);
                                let response = match result {
                                    Ok(value) => IpcResponse {
                                        id: request.id,
                                        result: Some(value),
                                        error: None,
                                    },
                                    Err(error) => IpcResponse {
                                        id: request.id,
                                        result: None,
                                        error: Some(error),
                                    },
                                };
                                let _ = responder.send(response);
                            } else {
                                let response = IpcResponse {
                                    id: request.id,
                                    result: None,
                                    error: Some(IpcError {
                                        code: -32601,
                                        message: format!("Method not found: {}", request.method),
                                        data: None,
                                    }),
                                };
                                let _ = responder.send(response);
                            }
                        }
                        IpcMessage::Notification(notification) => {
                            let handlers = notification_handlers.read().await;
                            if let Some(handlers) = handlers.get(&notification.method) {
                                for handler in handlers {
                                    handler(notification.params.clone());
                                }
                            }
                        }
                        IpcMessage::Response(response) => {
                            let mut pending = pending_requests.write().await;
                            if let Some(responder) = pending.remove(&response.id) {
                                let _ = responder.send(response);
                            }
                        }
                        IpcMessage::Shutdown => break,
                    }
                }
            });
        }
    }

    pub async fn send_request(&self, method: &str, params: Value, timeout_ms: Option<u64>) -> CoreResult<Value> {
        let id = Uuid::new_v4().to_string();
        let (tx, rx) = oneshot::channel();

        {
            let mut pending = self.pending_requests.write().await;
            pending.insert(id.clone(), tx);
        }

        let request = IpcRequest {
            id: id.clone(),
            method: method.to_string(),
            params,
            timeout_ms,
        };

        self.tx.send(IpcMessage::Request(request, tx)).map_err(|_| {
            crate::error::CoreError::IpcError("IPC channel closed".to_string())
        })?;

        let response = tokio::time::timeout(
            std::time::Duration::from_millis(timeout_ms.unwrap_or(30000)),
            rx,
        ).await
        .map_err(|_| crate::error::CoreError::IpcError("Request timeout".to_string()))?
        .map_err(|_| crate::error::CoreError::IpcError("Request cancelled".to_string()))?;

        if let Some(error) = response.error {
            return Err(crate::error::CoreError::IpcError(format!("{}: {}", error.code, error.message)));
        }

        response.result.ok_or_else(|| crate::error::CoreError::IpcError("Empty response".to_string()))
    }

    pub async fn send_notification(&self, method: &str, params: Value) -> CoreResult<()> {
        let notification = IpcNotification {
            method: method.to_string(),
            params,
        };

        self.tx.send(IpcMessage::Notification(notification)).map_err(|_| {
            crate::error::CoreError::IpcError("IPC channel closed".to_string())
        })?;

        Ok(())
    }

    pub async fn handle_response(&self, response: IpcResponse) {
        self.tx.send(IpcMessage::Response(response)).ok();
    }

    pub async fn shutdown(&self) {
        self.tx.send(IpcMessage::Shutdown).ok();
    }
}

#[derive(Clone)]
pub struct IpcSender {
    tx: mpsc::UnboundedSender<IpcMessage>,
}

impl IpcSender {
    pub async fn send_request(&self, method: &str, params: Value, timeout_ms: Option<u64>) -> CoreResult<Value> {
        let id = Uuid::new_v4().to_string();
        let (tx, rx) = oneshot::channel();

        let request = IpcRequest {
            id: id.clone(),
            method: method.to_string(),
            params,
            timeout_ms,
        };

        self.tx.send(IpcMessage::Request(request, tx)).map_err(|_| {
            crate::error::CoreError::IpcError("IPC channel closed".to_string())
        })?;

        let response = tokio::time::timeout(
            std::time::Duration::from_millis(timeout_ms.unwrap_or(30000)),
            rx,
        ).await
        .map_err(|_| crate::error::CoreError::IpcError("Request timeout".to_string()))?
        .map_err(|_| crate::error::CoreError::IpcError("Request cancelled".to_string()))?;

        if let Some(error) = response.error {
            return Err(crate::error::CoreError::IpcError(format!("{}: {}", error.code, error.message)));
        }

        response.result.ok_or_else(|| crate::error::CoreError::IpcError("Empty response".to_string()))
    }

    pub async fn send_notification(&self, method: &str, params: Value) -> CoreResult<()> {
        let notification = IpcNotification {
            method: method.to_string(),
            params,
        };

        self.tx.send(IpcMessage::Notification(notification)).map_err(|_| {
            crate::error::CoreError::IpcError("IPC channel closed".to_string())
        })?;

        Ok(())
    }
}

// Standard method names
pub mod methods {
    // Window management
    pub const WINDOW_CREATE: &str = "window.create";
    pub const WINDOW_CLOSE: &str = "window.close";
    pub const WINDOW_FOCUS: &str = "window.focus";
    pub const WINDOW_GET_BOUNDS: &str = "window.getBounds";
    pub const WINDOW_SET_BOUNDS: &str = "window.setBounds";
    pub const WINDOW_MAXIMIZE: &str = "window.maximize";
    pub const WINDOW_MINIMIZE: &str = "window.minimize";
    pub const WINDOW_FULLSCREEN: &str = "window.fullscreen";

    // Tab management
    pub const TAB_CREATE: &str = "tab.create";
    pub const TAB_CLOSE: &str = "tab.close";
    pub const TAB_NAVIGATE: &str = "tab.navigate";
    pub const TAB_RELOAD: &str = "tab.reload";
    pub const TAB_GO_BACK: &str = "tab.goBack";
    pub const TAB_GO_FORWARD: &str = "tab.goForward";
    pub const TAB_GET_STATE: &str = "tab.getState";
    pub const TAB_SET_ACTIVE: &str = "tab.setActive";
    pub const TAB_REORDER: &str = "tab.reorder";
    pub const TAB_PIN: &str = "tab.pin";
    pub const TAB_MUTE: &str = "tab.mute";

    // Navigation
    pub const NAVIGATE: &str = "navigate";
    pub const STOP_LOADING: &str = "stopLoading";

    // Profile management
    pub const PROFILE_CREATE: &str = "profile.create";
    pub const PROFILE_SWITCH: &str = "profile.switch";
    pub const PROFILE_DELETE: &str = "profile.delete";
    pub const PROFILE_LIST: &str = "profile.list";
    pub const PROFILE_GET_CURRENT: &str = "profile.getCurrent";

    // Settings
    pub const SETTINGS_GET: &str = "settings.get";
    pub const SETTINGS_SET: &str = "settings.set";
    pub const SETTINGS_RESET: &str = "settings.reset";

    // Bookmarks
    pub const BOOKMARK_ADD: &str = "bookmark.add";
    pub const BOOKMARK_REMOVE: &str = "bookmark.remove";
    pub const BOOKMARK_UPDATE: &str = "bookmark.update";
    pub const BOOKMARK_MOVE: &str = "bookmark.move";
    pub const BOOKMARK_GET_TREE: &str = "bookmark.getTree";
    pub const BOOKMARK_SEARCH: &str = "bookmark.search";
    pub const BOOKMARK_EXPORT: &str = "bookmark.export";
    pub const BOOKMARK_IMPORT: &str = "bookmark.import";

    // History
    pub const HISTORY_GET: &str = "history.get";
    pub const HISTORY_SEARCH: &str = "history.search";
    pub const HISTORY_CLEAR: &str = "history.clear";
    pub const HISTORY_MOST_VISITED: &str = "history.mostVisited";
    pub const HISTORY_RECENTLY_CLOSED: &str = "history.recentlyClosed";

    // Passwords
    pub const PASSWORD_GET: &str = "password.get";
    pub const PASSWORD_SAVE: &str = "password.save";
    pub const PASSWORD_DELETE: &str = "password.delete";
    pub const PASSWORD_LIST: &str = "password.list";
    pub const PASSWORD_FILL: &str = "password.fill";

    // Downloads
    pub const DOWNLOAD_START: &str = "download.start";
    pub const DOWNLOAD_CANCEL: &str = "download.cancel";
    pub const DOWNLOAD_PAUSE: &str = "download.pause";
    pub const DOWNLOAD_RESUME: &str = "download.resume";
    pub const DOWNLOAD_GET_LIST: &str = "download.getList";
    pub const DOWNLOAD_CLEAR: &str = "download.clear";

    // Extensions
    pub const EXTENSION_INSTALL: &str = "extension.install";
    pub const EXTENSION_UNINSTALL: &str = "extension.uninstall";
    pub const EXTENSION_ENABLE: &str = "extension.enable";
    pub const EXTENSION_DISABLE: &str = "extension.disable";
    pub const EXTENSION_GET_ALL: &str = "extension.getAll";
    pub const EXTENSION_GET: &str = "extension.get";

    // Content blocker
    pub const BLOCKER_GET_RULES: &str = "blocker.getRules";
    pub const BLOCKER_ADD_RULE: &str = "blocker.addRule";
    pub const BLOCKER_REMOVE_RULE: &str = "blocker.removeRule";
    pub const BLOCKER_TOGGLE: &str = "blocker.toggle";
    pub const BLOCKER_GET_STATS: &str = "blocker.getStats";
    pub const BLOCKER_UPDATE_LISTS: &str = "blocker.updateLists";

    // Autocomplete
    pub const AUTOCOMPLETE_SUGGEST: &str = "autocomplete.suggest";
    pub const AUTOCOMPLETE_ADD_ENGINE: &str = "autocomplete.addEngine";
    pub const AUTOCOMPLETE_REMOVE_ENGINE: &str = "autocomplete.removeEngine";
    pub const AUTOCOMPLETE_SET_DEFAULT: &str = "autocomplete.setDefault";

    // Sidebar
    pub const SIDEBAR_GET_STATE: &str = "sidebar.getState";
    pub const SIDEBAR_SET_OPEN: &str = "sidebar.setOpen";
    pub const SIDEBAR_SET_PANEL: &str = "sidebar.setPanel";
    pub const SIDEBAR_SET_WIDTH: &str = "sidebar.setWidth";

    // PiP
    pub const PIP_ENTER: &str = "pip.enter";
    pub const PIP_EXIT: &str = "pip.exit";
    pub const PIP_GET_STATE: &str = "pip.getState";
    pub const PIP_SET_MUTED: &str = "pip.setMuted";
    pub const PIP_SET_RATE: &str = "pip.setRate";

    // Telemetry
    pub const TELEMETRY_EVENT: &str = "telemetry.event";
    pub const TELEMETRY_FLUSH: &str = "telemetry.flush";

    // Notifications (from core to UI)
    pub const NOTIFY_TAB_CREATED: &str = "notify.tabCreated";
    pub const NOTIFY_TAB_CLOSED: &str = "notify.tabClosed";
    pub const NOTIFY_TAB_NAVIGATED: &str = "notify.tabNavigated";
    pub const NOTIFY_TAB_TITLE_CHANGED: &str = "notify.tabTitleChanged";
    pub const NOTIFY_TAB_FAVICON_CHANGED: &str = "notify.tabFaviconChanged";
    pub const NOTIFY_TAB_LOADING_CHANGED: &str = "notify.tabLoadingChanged";
    pub const NOTIFY_TAB_CRASHED: &str = "notify.tabCrashed";
    pub const NOTIFY_WINDOW_CREATED: &str = "notify.windowCreated";
    pub const NOTIFY_WINDOW_CLOSED: &str = "notify.windowClosed";
    pub const NOTIFY_WINDOW_FOCUSED: &str = "notify.windowFocused";
    pub const NOTIFY_DOWNLOAD_STARTED: &str = "notify.downloadStarted";
    pub const NOTIFY_DOWNLOAD_PROGRESS: &str = "notify.downloadProgress";
    pub const NOTIFY_DOWNLOAD_COMPLETED: &str = "notify.downloadCompleted";
    pub const NOTIFY_DOWNLOAD_FAILED: &str = "notify.downloadFailed";
    pub const NOTIFY_BOOKMARK_CHANGED: &str = "notify.bookmarkChanged";
    pub const NOTIFY_HISTORY_CHANGED: &str = "notify.historyChanged";
    pub const NOTIFY_PASSWORD_SAVED: &str = "notify.passwordSaved";
    pub const NOTIFY_EXTENSION_CHANGED: &str = "notify.extensionChanged";
    pub const NOTIFY_SETTINGS_CHANGED: &str = "notify.settingsChanged";
    pub const NOTIFY_THEME_CHANGED: &str = "notify.themeChanged";
    pub const NOTIFY_PROFILE_SWITCHED: &str = "notify.profileSwitched";
    pub const NOTIFY_PIP_ENTERED: &str = "notify.pipEntered";
    pub const NOTIFY_PIP_EXITED: &str = "notify.pipExited";
    pub const NOTIFY_CONTENT_BLOCKED: &str = "notify.contentBlocked";
}

// Tauri command wrappers for TypeScript
pub mod tauri_commands {
    use super::*;
    use tauri::State;

    pub struct IpcState(pub Arc<IpcManager>);

    #[tauri::command]
    pub async fn ipc_request(
        state: State<'_, IpcState>,
        method: String,
        params: Value,
        timeout_ms: Option<u64>,
    ) -> Result<Value, String> {
        state.0.send_request(&method, params, timeout_ms).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub async fn ipc_notify(
        state: State<'_, IpcState>,
        method: String,
        params: Value,
    ) -> Result<(), String> {
        state.0.send_notification(&method, params).await.map_err(|e| e.to_string())
    }
}