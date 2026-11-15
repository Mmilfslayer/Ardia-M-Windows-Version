use std::sync::Arc;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Runtime};

use ardia_core::ipc::{IpcManager, methods};

pub fn init_events<R: Runtime>(app: AppHandle<R>, ipc: Arc<IpcManager>) {
    // Register notification handlers for core -> UI events
    let app_handle = app.clone();
    let ipc_clone = ipc.clone();

    tauri::async_runtime::spawn(async move {
        // Window events
        ipc_clone.register_notification_handler(methods::NOTIFY_WINDOW_CREATED, move |params| {
            let _ = app_handle.emit("window-created", params);
        }).await;

        ipc_clone.register_notification_handler(methods::NOTIFY_WINDOW_CLOSED, move |params| {
            let _ = app_handle.emit("window-closed", params);
        }).await;

        ipc_clone.register_notification_handler(methods::NOTIFY_WINDOW_FOCUSED, move |params| {
            let _ = app_handle.emit("window-focused", params);
        }).await;

        // Tab events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_TAB_CREATED, move |params| {
            let _ = app_handle.emit("tab-created", params);
        }).await;

        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_TAB_CLOSED, move |params| {
            let _ = app_handle.emit("tab-closed", params);
        }).await;

        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_TAB_NAVIGATED, move |params| {
            let _ = app_handle.emit("tab-navigated", params);
        }).await;

        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_TAB_TITLE_CHANGED, move |params| {
            let _ = app_handle.emit("tab-title-changed", params);
        }).await;

        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_TAB_FAVICON_CHANGED, move |params| {
            let _ = app_handle.emit("tab-favicon-changed", params);
        }).await;

        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_TAB_LOADING_CHANGED, move |params| {
            let _ = app_handle.emit("tab-loading-changed", params);
        }).await;

        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_TAB_CRASHED, move |params| {
            let _ = app_handle.emit("tab-crashed", params);
        }).await;

        // Download events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_DOWNLOAD_STARTED, move |params| {
            let _ = app_handle.emit("download-started", params);
        }).await;

        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_DOWNLOAD_PROGRESS, move |params| {
            let _ = app_handle.emit("download-progress", params);
        }).await;

        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_DOWNLOAD_COMPLETED, move |params| {
            let _ = app_handle.emit("download-completed", params);
        }).await;

        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_DOWNLOAD_FAILED, move |params| {
            let _ = app_handle.emit("download-failed", params);
        }).await;

        // Bookmark events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_BOOKMARK_CHANGED, move |params| {
            let _ = app_handle.emit("bookmark-changed", params);
        }).await;

        // History events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_HISTORY_CHANGED, move |params| {
            let _ = app_handle.emit("history-changed", params);
        }).await;

        // Password events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_PASSWORD_SAVED, move |params| {
            let _ = app_handle.emit("password-saved", params);
        }).await;

        // Extension events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_EXTENSION_CHANGED, move |params| {
            let _ = app_handle.emit("extension-changed", params);
        }).await;

        // Settings events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_SETTINGS_CHANGED, move |params| {
            let _ = app_handle.emit("settings-changed", params);
        }).await;

        // Theme events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_THEME_CHANGED, move |params| {
            let _ = app_handle.emit("theme-changed", params);
        }).await;

        // Profile events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_PROFILE_SWITCHED, move |params| {
            let _ = app_handle.emit("profile-switched", params);
        }).await;

        // PiP events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_PIP_ENTERED, move |params| {
            let _ = app_handle.emit("pip-entered", params);
        }).await;

        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_PIP_EXITED, move |params| {
            let _ = app_handle.emit("pip-exited", params);
        }).await;

        // Content blocker events
        let app_handle = app_handle.clone();
        ipc_clone.register_notification_handler(methods::NOTIFY_CONTENT_BLOCKED, move |params| {
            let _ = app_handle.emit("content-blocked", params);
        }).await;
    });
}

// Helper functions to emit events from core
pub fn emit_window_created<R: Runtime>(app: &AppHandle<R>, window_id: &str, profile_id: &str) {
    let _ = app.emit("window-created", serde_json::json!({
        "window_id": window_id,
        "profile_id": profile_id,
    }));
}

pub fn emit_window_closed<R: Runtime>(app: &AppHandle<R>, window_id: &str) {
    let _ = app.emit("window-closed", serde_json::json!({
        "window_id": window_id,
    }));
}

pub fn emit_tab_created<R: Runtime>(app: &AppHandle<R>, window_id: &str, tab_id: &str, url: &str) {
    let _ = app.emit("tab-created", serde_json::json!({
        "window_id": window_id,
        "tab_id": tab_id,
        "url": url,
    }));
}

pub fn emit_tab_closed<R: Runtime>(app: &AppHandle<R>, window_id: &str, tab_id: &str) {
    let _ = app.emit("tab-closed", serde_json::json!({
        "window_id": window_id,
        "tab_id": tab_id,
    }));
}

pub fn emit_tab_navigated<R: Runtime>(app: &AppHandle<R>, window_id: &str, tab_id: &str, url: &str, title: &str) {
    let _ = app.emit("tab-navigated", serde_json::json!({
        "window_id": window_id,
        "tab_id": tab_id,
        "url": url,
        "title": title,
    }));
}

pub fn emit_tab_title_changed<R: Runtime>(app: &AppHandle<R>, window_id: &str, tab_id: &str, title: &str) {
    let _ = app.emit("tab-title-changed", serde_json::json!({
        "window_id": window_id,
        "tab_id": tab_id,
        "title": title,
    }));
}

pub fn emit_tab_favicon_changed<R: Runtime>(app: &AppHandle<R>, window_id: &str, tab_id: &str, favicon_url: &str) {
    let _ = app.emit("tab-favicon-changed", serde_json::json!({
        "window_id": window_id,
        "tab_id": tab_id,
        "favicon_url": favicon_url,
    }));
}

pub fn emit_tab_loading_changed<R: Runtime>(app: &AppHandle<R>, window_id: &str, tab_id: &str, loading: bool, progress: f64) {
    let _ = app.emit("tab-loading-changed", serde_json::json!({
        "window_id": window_id,
        "tab_id": tab_id,
        "loading": loading,
        "progress": progress,
    }));
}

pub fn emit_download_started<R: Runtime>(app: &AppHandle<R>, download_id: &str, url: &str, filename: &str) {
    let _ = app.emit("download-started", serde_json::json!({
        "download_id": download_id,
        "url": url,
        "filename": filename,
    }));
}

pub fn emit_download_progress<R: Runtime>(app: &AppHandle<R>, download_id: &str, progress: f64, received: u64, total: u64) {
    let _ = app.emit("download-progress", serde_json::json!({
        "download_id": download_id,
        "progress": progress,
        "received_bytes": received,
        "total_bytes": total,
    }));
}

pub fn emit_download_completed<R: Runtime>(app: &AppHandle<R>, download_id: &str, path: &str) {
    let _ = app.emit("download-completed", serde_json::json!({
        "download_id": download_id,
        "path": path,
    }));
}

pub fn emit_download_failed<R: Runtime>(app: &AppHandle<R>, download_id: &str, error: &str) {
    let _ = app.emit("download-failed", serde_json::json!({
        "download_id": download_id,
        "error": error,
    }));
}

pub fn emit_bookmark_changed<R: Runtime>(app: &AppHandle<R>, action: &str, bookmark_id: &str, profile_id: &str) {
    let _ = app.emit("bookmark-changed", serde_json::json!({
        "action": action,
        "bookmark_id": bookmark_id,
        "profile_id": profile_id,
    }));
}

pub fn emit_history_changed<R: Runtime>(app: &AppHandle<R>, action: &str, profile_id: &str) {
    let _ = app.emit("history-changed", serde_json::json!({
        "action": action,
        "profile_id": profile_id,
    }));
}

pub fn emit_extension_changed<R: Runtime>(app: &AppHandle<R>, action: &str, extension_id: &str) {
    let _ = app.emit("extension-changed", serde_json::json!({
        "action": action,
        "extension_id": extension_id,
    }));
}

pub fn emit_settings_changed<R: Runtime>(app: &AppHandle<R>, key: &str, value: Value) {
    let _ = app.emit("settings-changed", serde_json::json!({
        "key": key,
        "value": value,
    }));
}

pub fn emit_theme_changed<R: Runtime>(app: &AppHandle<R>, theme: &str) {
    let _ = app.emit("theme-changed", serde_json::json!({
        "theme": theme,
    }));
}

pub fn emit_profile_switched<R: Runtime>(app: &AppHandle<R>, profile_id: &str) {
    let _ = app.emit("profile-switched", serde_json::json!({
        "profile_id": profile_id,
    }));
}

pub fn emit_pip_entered<R: Runtime>(app: &AppHandle<R>, pip_id: &str, tab_id: &str) {
    let _ = app.emit("pip-entered", serde_json::json!({
        "pip_id": pip_id,
        "tab_id": tab_id,
    }));
}

pub fn emit_pip_exited<R: Runtime>(app: &AppHandle<R>, pip_id: &str) {
    let _ = app.emit("pip-exited", serde_json::json!({
        "pip_id": pip_id,
    }));
}

pub fn emit_content_blocked<R: Runtime>(app: &AppHandle<R>, url: &str, rule: &str) {
    let _ = app.emit("content-blocked", serde_json::json!({
        "url": url,
        "rule": rule,
    }));
}