use std::sync::Arc;
use tokio::sync::RwLock;

use ardia_core::{
    window::WindowManager,
    tab::TabManager,
    profile::ProfileManager,
    settings::SettingsManager,
    extensions::ExtensionManager,
    downloads::DownloadManager,
    history::HistoryManager,
    bookmarks::BookmarkManager,
    passwords::PasswordManager,
    autocomplete::AutocompleteManager,
    content_blocker::ContentBlocker,
    sidebar::SidebarManager,
    pip::PipManager,
    telemetry::TelemetryManager,
    ipc::{IpcManager, methods, IpcState},
    ArdiaCore,
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};
use tauri_plugin_log::{Target, TargetKind};

mod commands;
mod events;
mod state;

use commands::*;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let core = tauri::async_runtime::block_on(async {
        ArdiaCore::new().await.expect("Failed to initialize ArdiaCore")
    });

    let ipc = Arc::new(IpcManager::new());
    let app_state = AppState::new(core, ipc.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { file_name: Some("ardia.log".to_string()) }),
            ])
            .build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(app_state)
        .manage(IpcState(ipc.clone()))
        .invoke_handler(tauri::generate_handler![
            // Window commands
            window_create,
            window_close,
            window_focus,
            window_get_bounds,
            window_set_bounds,
            window_maximize,
            window_minimize,
            window_fullscreen,
            window_list,
            // Tab commands
            tab_create,
            tab_close,
            tab_navigate,
            tab_reload,
            tab_go_back,
            tab_go_forward,
            tab_get_state,
            tab_set_active,
            tab_reorder,
            tab_pin,
            tab_mute,
            tab_list,
            // Navigation
            navigate,
            stop_loading,
            // Profile commands
            profile_create,
            profile_switch,
            profile_delete,
            profile_list,
            profile_get_current,
            // Settings commands
            settings_get,
            settings_set,
            settings_reset,
            // Bookmark commands
            bookmark_add,
            bookmark_remove,
            bookmark_update,
            bookmark_move,
            bookmark_get_tree,
            bookmark_search,
            bookmark_export,
            bookmark_import,
            // History commands
            history_get,
            history_search,
            history_clear,
            history_most_visited,
            history_recently_closed,
            // Password commands
            password_get,
            password_save,
            password_delete,
            password_list,
            password_fill,
            // Download commands
            download_start,
            download_cancel,
            download_pause,
            download_resume,
            download_get_list,
            download_clear,
            // Extension commands
            extension_install,
            extension_uninstall,
            extension_enable,
            extension_disable,
            extension_get_all,
            extension_get,
            // Content blocker commands
            blocker_get_rules,
            blocker_add_rule,
            blocker_remove_rule,
            blocker_toggle,
            blocker_get_stats,
            blocker_update_lists,
            // Autocomplete commands
            autocomplete_suggest,
            autocomplete_add_engine,
            autocomplete_remove_engine,
            autocomplete_set_default,
            // Sidebar commands
            sidebar_get_state,
            sidebar_set_open,
            sidebar_set_panel,
            sidebar_set_width,
            // PiP commands
            pip_enter,
            pip_exit,
            pip_get_state,
            pip_set_muted,
            pip_set_rate,
            // Telemetry commands
            telemetry_event,
            telemetry_flush,
            // IPC commands
            ipc_request,
            ipc_notify,
        ])
        .setup(move |app| {
            // Initialize IPC
            let ipc = ipc.clone();
            tauri::async_runtime::spawn(async move {
                ipc.start_processing().await;
            });

            // Set up system tray
            setup_tray(app)?;

            // Set up custom protocol handlers
            setup_protocols(app)?;

            // Initialize event listeners
            events::init_events(app.handle().clone(), ipc);

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::Focused(focused) => {
                    if *focused {
                        let _ = window.emit("window-focused", ());
                    }
                }
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // Handle window close
                    window.hide().ok();
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let show_item = MenuItem::with_id(app, "show", "Show Ardia", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

    TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().cloned().unwrap())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    window.show().ok();
                    window.set_focus().ok();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|_tray, event| {
            if let TrayIconEvent::Click { .. } = event {
                // Could show main window on click
            }
        })
        .build(app)?;

    Ok(())
}

fn setup_protocols<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    // Register custom protocols
    // ardia:// - for internal pages (settings, history, bookmarks, etc.)
    // ardia-extension:// - for extension pages

    Ok(())
}

fn main() {
    run();
}