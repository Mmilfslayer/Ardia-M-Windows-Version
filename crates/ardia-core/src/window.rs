use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

use crate::{error::CoreResult, webview::WebView};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct WindowId(pub Uuid);

impl WindowId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for WindowId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for WindowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WindowKind {
    Browser,
    Small,
    Settings,
    Import,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenWindowOptions {
    pub frame: Option<[f64; 4]>,
    pub incognito: Option<bool>,
    pub title: Option<String>,
    pub focus: Option<bool>,
    pub kind: Option<WindowKind>,
    pub profile: Option<String>,
    pub size: Option<[f64; 2]>,
    pub behind: Option<String>,
    pub chrome_window: Option<String>,
    pub claim: Option<String>,
    pub pointer: Option<[f64; 2]>,
    pub animated: Option<bool>,
}

impl Default for OpenWindowOptions {
    fn default() -> Self {
        Self {
            frame: None,
            incognito: Some(false),
            title: None,
            focus: Some(true),
            kind: Some(WindowKind::Browser),
            profile: None,
            size: None,
            behind: None,
            chrome_window: None,
            claim: None,
            pointer: None,
            animated: Some(true),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowState {
    pub id: WindowId,
    pub hwnd: Option<HWND>,
    pub tabs: Vec<String>, // Tab IDs
    pub active_tab: Option<String>,
    pub profile_id: String,
    pub bounds: Option<[f64; 4]>,
    pub is_fullscreen: bool,
    pub is_maximized: bool,
    pub theme: String,
    pub sidebar_visible: bool,
    pub sidebar_width: f64,
    pub kind: WindowKind,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl WindowState {
    pub fn new(id: WindowId, profile_id: String, kind: WindowKind) -> Self {
        Self {
            id,
            hwnd: None,
            tabs: Vec::new(),
            active_tab: None,
            profile_id,
            bounds: None,
            is_fullscreen: false,
            is_maximized: false,
            theme: "system".to_string(),
            sidebar_visible: true,
            sidebar_width: 280.0,
            kind,
            created_at: chrono::Utc::now(),
        }
    }
}

pub struct WindowManager {
    windows: HashMap<WindowId, Arc<RwLock<WindowState>>>,
    webview_env: Option<Arc<crate::webview::WebViewEnvironment>>,
    default_profile: String,
}

impl WindowManager {
    pub async fn new() -> CoreResult<Self> {
        Ok(Self {
            windows: HashMap::new(),
            webview_env: None,
            default_profile: "default".to_string(),
        })
    }

    pub fn set_webview_environment(&mut self, env: Arc<crate::webview::WebViewEnvironment>) {
        self.webview_env = Some(env);
    }

    pub async fn create_window(&mut self, options: OpenWindowOptions) -> CoreResult<WindowId> {
        let profile_id = options.profile.unwrap_or_else(|| self.default_profile.clone());
        let kind = options.kind.unwrap_or(WindowKind::Browser);
        let id = WindowId::new();

        let mut window = WindowState::new(id, profile_id, kind);

        if let Some(frame) = options.frame {
            window.bounds = Some(frame);
        }

        if let Some(title) = options.title {
            // Store title in window state
        }

        let window_arc = Arc::new(RwLock::new(window));
        self.windows.insert(id, window_arc.clone());

        // If focus is requested, make this the active window
        if options.focus.unwrap_or(true) {
            // Focus logic will be handled by the UI layer
        }

        Ok(id)
    }

    pub async fn close_window(&mut self, id: WindowId) -> CoreResult<()> {
        if let Some(window) = self.windows.remove(&id) {
            let mut window = window.write().await;
            if let Some(hwnd) = window.hwnd {
                // Close the WebView2 controller
                // The actual WebView2 cleanup happens in the UI layer
                unsafe {
                    windows::Win32::UI::WindowsAndMessaging::DestroyWindow(hwnd)?;
                }
            }
        }
        Ok(())
    }

    pub async fn focus_window(&self, id: WindowId) -> CoreResult<()> {
        if let Some(window) = self.windows.get(&id) {
            let window = window.read().await;
            if let Some(hwnd) = window.hwnd {
                unsafe {
                    windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow(hwnd)?;
                    windows::Win32::UI::WindowsAndMessaging::BringWindowToTop(hwnd)?;
                }
            }
        }
        Ok(())
    }

    pub async fn get_window_frame(&self, id: WindowId) -> CoreResult<Option<[f64; 4]>> {
        if let Some(window) = self.windows.get(&id) {
            let window = window.read().await;
            if let Some(hwnd) = window.hwnd {
                let mut rect = RECT::default();
                unsafe {
                    GetClientRect(hwnd, &mut rect)?;
                }
                return Ok(Some([
                    rect.left as f64,
                    rect.top as f64,
                    rect.right as f64,
                    rect.bottom as f64,
                ]));
            }
            Ok(window.bounds)
        } else {
            Ok(None)
        }
    }

    pub async fn set_window_title(&self, id: WindowId, title: &str) -> CoreResult<()> {
        if let Some(window) = self.windows.get(&id) {
            // The actual title setting happens in the UI layer via WebView2
            // Here we just store it in state
            let mut window = window.write().await;
            // Store title if needed
        }
        Ok(())
    }

    pub async fn get_window(&self, id: WindowId) -> Option<Arc<RwLock<WindowState>>> {
        self.windows.get(&id).cloned()
    }

    pub async fn list_windows(&self) -> Vec<WindowId> {
        self.windows.keys().copied().collect()
    }

    pub async fn set_window_bounds(&self, id: WindowId, bounds: [f64; 4]) -> CoreResult<()> {
        if let Some(window) = self.windows.get(&id) {
            let mut window = window.write().await;
            window.bounds = Some(bounds);
            if let Some(hwnd) = window.hwnd {
                let rect = RECT {
                    left: bounds[0] as i32,
                    top: bounds[1] as i32,
                    right: bounds[2] as i32,
                    bottom: bounds[3] as i32,
                };
                unsafe {
                    windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
                        hwnd,
                        windows::Win32::UI::WindowsAndMessaging::HWND_TOP,
                        rect.left,
                        rect.top,
                        rect.right - rect.left,
                        rect.bottom - rect.top,
                        windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER | windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE,
                    )?;
                }
            }
        }
        Ok(())
    }

    pub async fn set_fullscreen(&self, id: WindowId, fullscreen: bool) -> CoreResult<()> {
        if let Some(window) = self.windows.get(&id) {
            let mut window = window.write().await;
            window.is_fullscreen = fullscreen;
            if let Some(hwnd) = window.hwnd {
                let style = if fullscreen {
                    windows::Win32::UI::WindowsAndMessaging::WS_POPUP | windows::Win32::UI::WindowsAndMessaging::WS_VISIBLE
                } else {
                    windows::Win32::UI::WindowsAndMessaging::WS_OVERLAPPEDWINDOW | windows::Win32::UI::WindowsAndMessaging::WS_VISIBLE
                };
                unsafe {
                    windows::Win32::UI::WindowsAndMessaging::SetWindowLongW(
                        hwnd,
                        windows::Win32::UI::WindowsAndMessaging::GWL_STYLE,
                        style.0 as i32,
                    );
                    windows::Win32::UI::WindowsAndMessaging::SetWindowPos(
                        hwnd,
                        windows::Win32::UI::WindowsAndMessaging::HWND_TOP,
                        0, 0, 0, 0,
                        windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE | windows::Win32::UI::WindowsAndMessaging::SWP_NOSIZE | windows::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED,
                    )?;
                }
            }
        }
        Ok(())
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        // This is just for compilation; actual initialization is async
        Self {
            windows: HashMap::new(),
            webview_env: None,
            default_profile: "default".to_string(),
        }
    }
}