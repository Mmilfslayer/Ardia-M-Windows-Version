use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{State, Window, AppHandle, Emitter};

use ardia_core::{
    window::{WindowManager, WindowId, WindowState, OpenWindowOptions},
    tab::{TabManager, TabId, TabState, TabNavigation},
    profile::{ProfileManager, ProfileId, Profile},
    settings::SettingsManager,
    extensions::{ExtensionManager, ExtensionId},
    downloads::{DownloadManager, DownloadId, DownloadState},
    history::{HistoryManager, HistoryEntry, HistoryId},
    bookmarks::{BookmarkManager, Bookmark, BookmarkId},
    passwords::{PasswordManager, PasswordEntry, PasswordId},
    autocomplete::{AutocompleteManager, AutocompleteResult, SearchEngine},
    content_blocker::{ContentBlocker, FilterRule, FilterAction, ResourceType, BlockerStats, FilterList},
    sidebar::{SidebarManager, SidebarPanel, SidebarState, PanelState},
    pip::{PipManager, PipId, PipWindow, PipBounds, PipState},
    telemetry::{TelemetryManager, TelemetryEventType, TelemetryEvent, PerformanceMetric, SessionInfo},
    ipc::{IpcManager, IpcState},
    ArdiaCore,
};

use crate::state::AppState;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateWindowRequest {
    pub profile_id: Option<String>,
    pub url: Option<String>,
    pub bounds: Option<WindowBounds>,
    pub fullscreen: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CreateWindowResponse {
    pub window_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
pub struct NavigateRequest {
    pub window_id: String,
    pub tab_id: Option<String>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTabRequest {
    pub window_id: String,
    pub url: Option<String>,
    pub background: Option<bool>,
    pub pinned: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CreateTabResponse {
    pub tab_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CloseTabRequest {
    pub window_id: String,
    pub tab_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SetActiveTabRequest {
    pub window_id: String,
    pub tab_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ReorderTabsRequest {
    pub window_id: String,
    pub tab_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PinTabRequest {
    pub window_id: String,
    pub tab_id: String,
    pub pinned: bool,
}

#[derive(Debug, Deserialize)]
pub struct MuteTabRequest {
    pub window_id: String,
    pub tab_id: String,
    pub muted: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProfileCreateRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ProfileCreateResponse {
    pub profile_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SettingsGetRequest {
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct SettingsSetRequest {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkAddRequest {
    pub parent_id: String,
    pub index: i32,
    pub title: String,
    pub url: Option<String>,
    pub profile_id: String,
}

#[derive(Debug, Serialize)]
pub struct BookmarkAddResponse {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkMoveRequest {
    pub id: String,
    pub new_parent_id: String,
    pub new_index: i32,
}

#[derive(Debug, Deserialize)]
pub struct BookmarkSearchRequest {
    pub profile_id: String,
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct HistorySearchRequest {
    pub profile_id: String,
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct PasswordGetRequest {
    pub origin_url: String,
    pub profile_id: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordSaveRequest {
    pub origin_url: String,
    pub action_url: String,
    pub username: String,
    pub password: String,
    pub username_element: Option<String>,
    pub password_element: Option<String>,
    pub profile_id: String,
    pub is_federated: Option<bool>,
    pub federation_origin: Option<String>,
    pub display_name: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PasswordSaveResponse {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct DownloadStartRequest {
    pub url: String,
    pub filename: Option<String>,
    pub referrer: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DownloadStartResponse {
    pub download_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ExtensionInstallRequest {
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct ExtensionInstallResponse {
    pub extension_id: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockerAddRuleRequest {
    pub rule: String,
    pub action: String,
    pub domains: Option<Vec<String>>,
    pub exclude_domains: Option<Vec<String>>,
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct BlockerAddRuleResponse {
    pub rule_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct AutocompleteSuggestRequest {
    pub input: String,
    pub profile_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AutocompleteAddEngineRequest {
    pub name: String,
    pub keyword: String,
    pub search_url: String,
    pub suggest_url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SidebarSetPanelRequest {
    pub panel: String,
}

#[derive(Debug, Deserialize)]
pub struct PipEnterRequest {
    pub tab_id: String,
    pub window_id: String,
}

#[derive(Debug, Serialize)]
pub struct PipEnterResponse {
    pub pip_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TelemetryEventRequest {
    pub event_type: String,
    pub profile_id: Option<String>,
    pub properties: Option<Value>,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct IpcRequestRequest {
    pub method: String,
    pub params: Value,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct IpcNotifyRequest {
    pub method: String,
    pub params: Value,
}

// ============================================================================
// Window Commands
// ============================================================================

#[tauri::command]
pub async fn window_create(
    state: State<'_, AppState>,
    request: CreateWindowRequest,
) -> Result<CreateWindowResponse, String> {
    let core = state.core.read().await;

    let options = OpenWindowOptions {
        profile_id: request.profile_id.unwrap_or_else(|| "default".to_string()),
        initial_url: request.url,
        bounds: request.bounds.map(|b| ardia_core::window::Rect {
            x: b.x as i32,
            y: b.y as i32,
            width: b.width as u32,
            height: b.height as u32,
        }),
        fullscreen: request.fullscreen.unwrap_or(false),
    };

    let window_id = core.window_manager.write().await
        .create_window(options)
        .map_err(|e| e.to_string())?;

    Ok(CreateWindowResponse {
        window_id: window_id.to_string(),
    })
}

#[tauri::command]
pub async fn window_close(
    state: State<'_, AppState>,
    window_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    core.window_manager.write().await
        .close_window(window_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn window_focus(
    state: State<'_, AppState>,
    window_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    core.window_manager.write().await
        .focus_window(window_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn window_get_bounds(
    state: State<'_, AppState>,
    window_id: String,
) -> Result<WindowBounds, String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    let bounds = core.window_manager.read().await
        .get_window_frame(window_id)
        .map_err(|e| e.to_string())?;

    Ok(WindowBounds {
        x: bounds.x as f64,
        y: bounds.y as f64,
        width: bounds.width as f64,
        height: bounds.height as f64,
    })
}

#[tauri::command]
pub async fn window_set_bounds(
    state: State<'_, AppState>,
    window_id: String,
    bounds: WindowBounds,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    core.window_manager.write().await
        .set_window_bounds(window_id, ardia_core::window::Rect {
            x: bounds.x as i32,
            y: bounds.y as i32,
            width: bounds.width as u32,
            height: bounds.height as u32,
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn window_maximize(
    state: State<'_, AppState>,
    window_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    core.window_manager.write().await
        .maximize_window(window_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn window_minimize(
    state: State<'_, AppState>,
    window_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    core.window_manager.write().await
        .minimize_window(window_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn window_fullscreen(
    state: State<'_, AppState>,
    window_id: String,
    fullscreen: bool,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    core.window_manager.write().await
        .set_fullscreen(window_id, fullscreen)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn window_list(
    state: State<'_, AppState>,
) -> Result<Vec<WindowState>, String> {
    let core = state.core.read().await;
    let windows = core.window_manager.read().await.list_windows();
    Ok(windows)
}

// ============================================================================
// Tab Commands
// ============================================================================

#[tauri::command]
pub async fn tab_create(
    state: State<'_, AppState>,
    request: CreateTabRequest,
) -> Result<CreateTabResponse, String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&request.window_id).map_err(|e| e.to_string())?);

    let tab_id = core.tab_manager.write().await
        .create_tab(window_id, request.url, request.background.unwrap_or(false), request.pinned.unwrap_or(false))
        .map_err(|e| e.to_string())?;

    Ok(CreateTabResponse {
        tab_id: tab_id.to_string(),
    })
}

#[tauri::command]
pub async fn tab_close(
    state: State<'_, AppState>,
    request: CloseTabRequest,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&request.window_id).map_err(|e| e.to_string())?);
    let tab_id = TabId(uuid::Uuid::parse_str(&request.tab_id).map_err(|e| e.to_string())?);

    core.tab_manager.write().await
        .close_tab(window_id, tab_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn tab_navigate(
    state: State<'_, AppState>,
    request: NavigateRequest,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&request.window_id).map_err(|e| e.to_string())?);

    let tab_id = if let Some(tid) = request.tab_id {
        Some(TabId(uuid::Uuid::parse_str(&tid).map_err(|e| e.to_string())?))
    } else {
        None
    };

    core.tab_manager.write().await
        .navigate(window_id, tab_id, &request.url)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn tab_reload(
    state: State<'_, AppState>,
    window_id: String,
    tab_id: Option<String>,
    bypass_cache: Option<bool>,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    let tab_id = tab_id.map(|tid| TabId(uuid::Uuid::parse_str(&tid).map_err(|e| e.to_string())))
        .transpose()?;

    core.tab_manager.write().await
        .reload(window_id, tab_id, bypass_cache.unwrap_or(false))
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn tab_go_back(
    state: State<'_, AppState>,
    window_id: String,
    tab_id: Option<String>,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    let tab_id = tab_id.map(|tid| TabId(uuid::Uuid::parse_str(&tid).map_err(|e| e.to_string())))
        .transpose()?;

    core.tab_manager.write().await
        .go_back(window_id, tab_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn tab_go_forward(
    state: State<'_, AppState>,
    window_id: String,
    tab_id: Option<String>,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    let tab_id = tab_id.map(|tid| TabId(uuid::Uuid::parse_str(&tid).map_err(|e| e.to_string())))
        .transpose()?;

    core.tab_manager.write().await
        .go_forward(window_id, tab_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn tab_get_state(
    state: State<'_, AppState>,
    window_id: String,
    tab_id: String,
) -> Result<TabState, String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);
    let tab_id = TabId(uuid::Uuid::parse_str(&tab_id).map_err(|e| e.to_string())?);

    let tab_state = core.tab_manager.read().await
        .get_tab(window_id, tab_id)
        .map_err(|e| e.to_string())?;

    Ok(tab_state)
}

#[tauri::command]
pub async fn tab_set_active(
    state: State<'_, AppState>,
    request: SetActiveTabRequest,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&request.window_id).map_err(|e| e.to_string())?);
    let tab_id = TabId(uuid::Uuid::parse_str(&request.tab_id).map_err(|e| e.to_string())?);

    core.tab_manager.write().await
        .set_active_tab(window_id, tab_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn tab_reorder(
    state: State<'_, AppState>,
    request: ReorderTabsRequest,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&request.window_id).map_err(|e| e.to_string())?);

    let tab_ids: Result<Vec<TabId>, _> = request.tab_ids.iter()
        .map(|tid| TabId(uuid::Uuid::parse_str(tid).map_err(|e| e.to_string())))
        .collect();

    core.tab_manager.write().await
        .reorder_tabs(window_id, &tab_ids?)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn tab_pin(
    state: State<'_, AppState>,
    request: PinTabRequest,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&request.window_id).map_err(|e| e.to_string())?);
    let tab_id = TabId(uuid::Uuid::parse_str(&request.tab_id).map_err(|e| e.to_string())?);

    core.tab_manager.write().await
        .pin_tab(window_id, tab_id, request.pinned)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn tab_mute(
    state: State<'_, AppState>,
    request: MuteTabRequest,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&request.window_id).map_err(|e| e.to_string())?);
    let tab_id = TabId(uuid::Uuid::parse_str(&request.tab_id).map_err(|e| e.to_string())?);

    core.tab_manager.write().await
        .mute_tab(window_id, tab_id, request.muted)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn tab_list(
    state: State<'_, AppState>,
    window_id: String,
) -> Result<Vec<TabState>, String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    let tabs = core.tab_manager.read().await
        .list_tabs(window_id);

    Ok(tabs)
}

// ============================================================================
// Navigation
// ============================================================================

#[tauri::command]
pub async fn navigate(
    state: State<'_, AppState>,
    window_id: String,
    tab_id: Option<String>,
    url: String,
) -> Result<(), String> {
    tab_navigate(state, NavigateRequest { window_id, tab_id, url }).await
}

#[tauri::command]
pub async fn stop_loading(
    state: State<'_, AppState>,
    window_id: String,
    tab_id: Option<String>,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);

    let tab_id = tab_id.map(|tid| TabId(uuid::Uuid::parse_str(&tid).map_err(|e| e.to_string())))
        .transpose()?;

    core.tab_manager.write().await
        .stop_loading(window_id, tab_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// Profile Commands
// ============================================================================

#[tauri::command]
pub async fn profile_create(
    state: State<'_, AppState>,
    request: ProfileCreateRequest,
) -> Result<ProfileCreateResponse, String> {
    let core = state.core.read().await;

    let profile_id = core.profile_manager.write().await
        .create_profile(request.name)
        .map_err(|e| e.to_string())?;

    Ok(ProfileCreateResponse {
        profile_id: profile_id.to_string(),
    })
}

#[tauri::command]
pub async fn profile_switch(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let profile_id = ProfileId(uuid::Uuid::parse_str(&profile_id).map_err(|e| e.to_string())?);

    core.profile_manager.write().await
        .switch_profile(profile_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn profile_delete(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let profile_id = ProfileId(uuid::Uuid::parse_str(&profile_id).map_err(|e| e.to_string())?);

    core.profile_manager.write().await
        .delete_profile(profile_id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn profile_list(
    state: State<'_, AppState>,
) -> Result<Vec<Profile>, String> {
    let core = state.core.read().await;

    let profiles = core.profile_manager.read().await
        .list_profiles();

    Ok(profiles)
}

#[tauri::command]
pub async fn profile_get_current(
    state: State<'_, AppState>,
) -> Result<Option<Profile>, String> {
    let core = state.core.read().await;

    let profile = core.profile_manager.read().await
        .get_current_profile();

    Ok(profile)
}

// ============================================================================
// Settings Commands
// ============================================================================

#[tauri::command]
pub async fn settings_get(
    state: State<'_, AppState>,
    request: SettingsGetRequest,
) -> Result<Option<Value>, String> {
    let core = state.core.read().await;

    let value = core.settings_manager.read().await
        .get(&request.key)
        .map_err(|e| e.to_string())?;

    Ok(value)
}

#[tauri::command]
pub async fn settings_set(
    state: State<'_, AppState>,
    request: SettingsSetRequest,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.settings_manager.write().await
        .set(&request.key, request.value)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn settings_reset(
    state: State<'_, AppState>,
    key: String,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.settings_manager.write().await
        .reset(&key)
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// Bookmark Commands
// ============================================================================

#[tauri::command]
pub async fn bookmark_add(
    state: State<'_, AppState>,
    request: BookmarkAddRequest,
) -> Result<BookmarkAddResponse, String> {
    let core = state.core.read().await;
    let parent_id = BookmarkId(uuid::Uuid::parse_str(&request.parent_id).map_err(|e| e.to_string())?);

    let bookmark_id = core.bookmark_manager.write().await
        .add_bookmark(
            parent_id,
            request.index,
            request.title,
            request.url,
            request.profile_id,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(BookmarkAddResponse {
        id: bookmark_id.to_string(),
    })
}

#[tauri::command]
pub async fn bookmark_remove(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let bookmark_id = BookmarkId(uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?);

    core.bookmark_manager.write().await
        .delete_bookmark(bookmark_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn bookmark_update(
    state: State<'_, AppState>,
    id: String,
    title: Option<String>,
    url: Option<String>,
) -> Result<(), String> {
    let core = state.core.read().await;
    let bookmark_id = BookmarkId(uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?);

    core.bookmark_manager.write().await
        .update_bookmark(bookmark_id, title, url)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn bookmark_move(
    state: State<'_, AppState>,
    request: BookmarkMoveRequest,
) -> Result<(), String> {
    let core = state.core.read().await;
    let id = BookmarkId(uuid::Uuid::parse_str(&request.id).map_err(|e| e.to_string())?);
    let new_parent_id = BookmarkId(uuid::Uuid::parse_str(&request.new_parent_id).map_err(|e| e.to_string())?);

    core.bookmark_manager.write().await
        .move_bookmark(id, new_parent_id, request.new_index)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn bookmark_get_tree(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<Vec<Bookmark>, String> {
    let core = state.core.read().await;

    let tree = core.bookmark_manager.read().await
        .get_tree(&profile_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(tree)
}

#[tauri::command]
pub async fn bookmark_search(
    state: State<'_, AppState>,
    request: BookmarkSearchRequest,
) -> Result<Vec<Bookmark>, String> {
    let core = state.core.read().await;

    let results = core.bookmark_manager.read().await
        .search_bookmarks(&request.profile_id, &request.query)
        .await
        .map_err(|e| e.to_string())?;

    Ok(results)
}

#[tauri::command]
pub async fn bookmark_export(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<String, String> {
    let core = state.core.read().await;

    let json = core.bookmark_manager.read().await
        .export_bookmarks(&profile_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(json)
}

#[tauri::command]
pub async fn bookmark_import(
    state: State<'_, AppState>,
    profile_id: String,
    json: String,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.bookmark_manager.write().await
        .import_bookmarks(&profile_id, &json)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// History Commands
// ============================================================================

#[tauri::command]
pub async fn history_get(
    state: State<'_, AppState>,
    profile_id: String,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<HistoryEntry>, String> {
    let core = state.core.read().await;

    let entries = core.history_manager.read().await
        .get_history(&profile_id, limit.unwrap_or(100), offset.unwrap_or(0))
        .await
        .map_err(|e| e.to_string())?;

    Ok(entries)
}

#[tauri::command]
pub async fn history_search(
    state: State<'_, AppState>,
    request: HistorySearchRequest,
) -> Result<Vec<HistoryEntry>, String> {
    let core = state.core.read().await;

    let entries = core.history_manager.read().await
        .search_history(&request.profile_id, &request.query, request.limit.unwrap_or(50))
        .await
        .map_err(|e| e.to_string())?;

    Ok(entries)
}

#[tauri::command]
pub async fn history_clear(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.history_manager.write().await
        .clear_history(&profile_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn history_most_visited(
    state: State<'_, AppState>,
    profile_id: String,
    limit: Option<usize>,
) -> Result<Vec<HistoryEntry>, String> {
    let core = state.core.read().await;

    let entries = core.history_manager.read().await
        .get_most_visited(&profile_id, limit.unwrap_or(10))
        .await
        .map_err(|e| e.to_string())?;

    Ok(entries)
}

#[tauri::command]
pub async fn history_recently_closed(
    state: State<'_, AppState>,
    profile_id: String,
    limit: Option<usize>,
) -> Result<Vec<HistoryEntry>, String> {
    let core = state.core.read().await;

    let entries = core.history_manager.read().await
        .get_recently_closed(&profile_id, limit.unwrap_or(10))
        .await
        .map_err(|e| e.to_string())?;

    Ok(entries)
}

// ============================================================================
// Password Commands
// ============================================================================

#[tauri::command]
pub async fn password_get(
    state: State<'_, AppState>,
    request: PasswordGetRequest,
) -> Result<Vec<PasswordEntry>, String> {
    let core = state.core.read().await;

    let passwords = core.password_manager.read().await
        .find_passwords(&request.origin_url, &request.profile_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(passwords)
}

#[tauri::command]
pub async fn password_save(
    state: State<'_, AppState>,
    request: PasswordSaveRequest,
) -> Result<PasswordSaveResponse, String> {
    let core = state.core.read().await;

    let password_id = core.password_manager.write().await
        .add_password(
            request.origin_url,
            request.action_url,
            request.username,
            request.password,
            request.username_element,
            request.password_element,
            request.profile_id,
            request.is_federated.unwrap_or(false),
            request.federation_origin,
            request.display_name,
            request.icon_url,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(PasswordSaveResponse {
        id: password_id.to_string(),
    })
}

#[tauri::command]
pub async fn password_delete(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let password_id = PasswordId(uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?);

    core.password_manager.write().await
        .delete_password(password_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn password_list(
    state: State<'_, AppState>,
    profile_id: String,
) -> Result<Vec<PasswordEntry>, String> {
    let core = state.core.read().await;

    let passwords = core.password_manager.read().await
        .find_all_passwords(&profile_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(passwords)
}

#[tauri::command]
pub async fn password_fill(
    state: State<'_, AppState>,
    window_id: String,
    tab_id: String,
    password_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let window_id = WindowId(uuid::Uuid::parse_str(&window_id).map_err(|e| e.to_string())?);
    let tab_id = TabId(uuid::Uuid::parse_str(&tab_id).map_err(|e| e.to_string())?);
    let password_id = PasswordId(uuid::Uuid::parse_str(&password_id).map_err(|e| e.to_string())?);

    // Get the password entry
    let password = core.password_manager.read().await
        .get_password(password_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Password not found".to_string())?;

    // Inject autofill script
    core.tab_manager.write().await
        .autofill_password(window_id, tab_id, &password.username, &password.password)
        .map_err(|e| e.to_string())?;

    // Record use
    core.password_manager.write().await
        .record_password_use(password_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// Download Commands
// ============================================================================

#[tauri::command]
pub async fn download_start(
    state: State<'_, AppState>,
    request: DownloadStartRequest,
) -> Result<DownloadStartResponse, String> {
    let core = state.core.read().await;

    let download_id = core.download_manager.write().await
        .start_download(request.url, request.filename, request.referrer)
        .await
        .map_err(|e| e.to_string())?;

    Ok(DownloadStartResponse {
        download_id: download_id.to_string(),
    })
}

#[tauri::command]
pub async fn download_cancel(
    state: State<'_, AppState>,
    download_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let download_id = DownloadId(uuid::Uuid::parse_str(&download_id).map_err(|e| e.to_string())?);

    core.download_manager.write().await
        .cancel_download(download_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn download_pause(
    state: State<'_, AppState>,
    download_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let download_id = DownloadId(uuid::Uuid::parse_str(&download_id).map_err(|e| e.to_string())?);

    core.download_manager.write().await
        .pause_download(download_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn download_resume(
    state: State<'_, AppState>,
    download_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let download_id = DownloadId(uuid::Uuid::parse_str(&download_id).map_err(|e| e.to_string())?);

    core.download_manager.write().await
        .resume_download(download_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn download_get_list(
    state: State<'_, AppState>,
) -> Result<Vec<DownloadState>, String> {
    let core = state.core.read().await;

    let downloads = core.download_manager.read().await
        .get_all_downloads();

    Ok(downloads)
}

#[tauri::command]
pub async fn download_clear(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.download_manager.write().await
        .clear_completed()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// Extension Commands
// ============================================================================

#[tauri::command]
pub async fn extension_install(
    state: State<'_, AppState>,
    request: ExtensionInstallRequest,
) -> Result<ExtensionInstallResponse, String> {
    let core = state.core.read().await;

    let extension_id = core.extension_manager.write().await
        .install_extension(&request.path)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ExtensionInstallResponse {
        extension_id: extension_id.to_string(),
    })
}

#[tauri::command]
pub async fn extension_uninstall(
    state: State<'_, AppState>,
    extension_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let extension_id = ExtensionId(uuid::Uuid::parse_str(&extension_id).map_err(|e| e.to_string())?);

    core.extension_manager.write().await
        .uninstall_extension(extension_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn extension_enable(
    state: State<'_, AppState>,
    extension_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let extension_id = ExtensionId(uuid::Uuid::parse_str(&extension_id).map_err(|e| e.to_string())?);

    core.extension_manager.write().await
        .enable_extension(extension_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn extension_disable(
    state: State<'_, AppState>,
    extension_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let extension_id = ExtensionId(uuid::Uuid::parse_str(&extension_id).map_err(|e| e.to_string())?);

    core.extension_manager.write().await
        .disable_extension(extension_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn extension_get_all(
    state: State<'_, AppState>,
) -> Result<Vec<ardia_core::extensions::Extension>, String> {
    let core = state.core.read().await;

    let extensions = core.extension_manager.read().await
        .get_all_extensions()
        .await
        .map_err(|e| e.to_string())?;

    Ok(extensions)
}

#[tauri::command]
pub async fn extension_get(
    state: State<'_, AppState>,
    extension_id: String,
) -> Result<Option<ardia_core::extensions::Extension>, String> {
    let core = state.core.read().await;
    let extension_id = ExtensionId(uuid::Uuid::parse_str(&extension_id).map_err(|e| e.to_string())?);

    let extension = core.extension_manager.read().await
        .get_extension(extension_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(extension)
}

// ============================================================================
// Content Blocker Commands
// ============================================================================

#[tauri::command]
pub async fn blocker_get_rules(
    state: State<'_, AppState>,
) -> Result<Vec<FilterRule>, String> {
    let core = state.core.read().await;

    let rules = core.content_blocker.read().await
        .get_all_rules();

    Ok(rules)
}

#[tauri::command]
pub async fn blocker_add_rule(
    state: State<'_, AppState>,
    request: BlockerAddRuleRequest,
) -> Result<BlockerAddRuleResponse, String> {
    let core = state.core.read().await;

    let action = match request.action.as_str() {
        "block" => FilterAction::Block,
        "allow" => FilterAction::Allow,
        "redirect" => FilterAction::Redirect("".to_string()),
        "upgrade" => FilterAction::UpgradeScheme,
        _ => FilterAction::Noop,
    };

    let resource_types: Result<Vec<ResourceType>, _> = request.resource_types
        .unwrap_or_default()
        .iter()
        .map(|s| s.parse())
        .collect();

    let rule_id = core.content_blocker.write().await
        .add_custom_rule(
            request.rule,
            action,
            request.domains.unwrap_or_default(),
            request.exclude_domains.unwrap_or_default(),
            resource_types?,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(BlockerAddRuleResponse { rule_id })
}

#[tauri::command]
pub async fn blocker_remove_rule(
    state: State<'_, AppState>,
    rule_id: u64,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.content_blocker.write().await
        .remove_rule(rule_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn blocker_toggle(
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.content_blocker.write().await
        .set_enabled(enabled);

    Ok(())
}

#[tauri::command]
pub async fn blocker_get_stats(
    state: State<'_, AppState>,
) -> Result<BlockerStats, String> {
    let core = state.core.read().await;

    let stats = core.content_blocker.read().await
        .get_stats();

    Ok(stats)
}

#[tauri::command]
pub async fn blocker_update_lists(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let core = state.core.read().await;

    let lists = core.content_blocker.read().await
        .get_filter_lists()
        .to_vec();

    for list in lists {
        if list.enabled {
            core.content_blocker.write().await
                .update_filter_list(&list.name)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

// ============================================================================
// Autocomplete Commands
// ============================================================================

#[tauri::command]
pub async fn autocomplete_suggest(
    state: State<'_, AppState>,
    request: AutocompleteSuggestRequest,
) -> Result<Vec<AutocompleteResult>, String> {
    let core = state.core.read().await;

    let suggestions = core.autocomplete_manager.read().await
        .get_suggestions(
            &request.input,
            &core.history_manager.read().await,
            &core.bookmark_manager.read().await,
            &request.profile_id,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(suggestions)
}

#[tauri::command]
pub async fn autocomplete_add_engine(
    state: State<'_, AppState>,
    request: AutocompleteAddEngineRequest,
) -> Result<(), String> {
    let core = state.core.read().await;

    let engine = SearchEngine {
        name: request.name,
        keyword: request.keyword,
        search_url: request.search_url,
        suggest_url: request.suggest_url,
        icon_url: request.icon_url,
        is_default: false,
    };

    core.autocomplete_manager.write().await
        .add_custom_search_engine(engine)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn autocomplete_remove_engine(
    state: State<'_, AppState>,
    keyword: String,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.autocomplete_manager.write().await
        .remove_search_engine(&keyword)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn autocomplete_set_default(
    state: State<'_, AppState>,
    keyword: String,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.autocomplete_manager.write().await
        .set_default_engine(&keyword)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// Sidebar Commands
// ============================================================================

#[tauri::command]
pub async fn sidebar_get_state(
    state: State<'_, AppState>,
) -> Result<SidebarState, String> {
    let core = state.core.read().await;

    let sidebar_state = core.sidebar_manager.read().await
        .get_state()
        .await;

    Ok(sidebar_state)
}

#[tauri::command]
pub async fn sidebar_set_open(
    state: State<'_, AppState>,
    open: bool,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.sidebar_manager.write().await
        .set_open(open)
        .await;

    Ok(())
}

#[tauri::command]
pub async fn sidebar_set_panel(
    state: State<'_, AppState>,
    request: SidebarSetPanelRequest,
) -> Result<(), String> {
    let core = state.core.read().await;

    let panel = match request.panel.as_str() {
        "vertical_tabs" => SidebarPanel::VerticalTabs,
        "bookmarks" => SidebarPanel::Bookmarks,
        "history" => SidebarPanel::History,
        "extensions" => SidebarPanel::Extensions,
        "downloads" => SidebarPanel::Downloads,
        "settings" => SidebarPanel::Settings,
        _ => SidebarPanel::VerticalTabs,
    };

    core.sidebar_manager.write().await
        .set_active_panel(panel)
        .await;

    Ok(())
}

#[tauri::command]
pub async fn sidebar_set_width(
    state: State<'_, AppState>,
    width: u32,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.sidebar_manager.write().await
        .set_width(width)
        .await;

    Ok(())
}

// ============================================================================
// PiP Commands
// ============================================================================

#[tauri::command]
pub async fn pip_enter(
    state: State<'_, AppState>,
    request: PipEnterRequest,
) -> Result<PipEnterResponse, String> {
    let core = state.core.read().await;
    let tab_id = TabId(uuid::Uuid::parse_str(&request.tab_id).map_err(|e| e.to_string())?);

    let pip_id = core.pip_manager.write().await
        .enter_pip(tab_id, request.window_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(PipEnterResponse {
        pip_id: pip_id.to_string(),
    })
}

#[tauri::command]
pub async fn pip_exit(
    state: State<'_, AppState>,
    pip_id: String,
) -> Result<(), String> {
    let core = state.core.read().await;
    let pip_id = PipId(uuid::Uuid::parse_str(&pip_id).map_err(|e| e.to_string())?);

    core.pip_manager.write().await
        .exit_pip(pip_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn pip_get_state(
    state: State<'_, AppState>,
    pip_id: Option<String>,
) -> Result<Option<PipWindow>, String> {
    let core = state.core.read().await;

    let window = if let Some(pip_id) = pip_id {
        let pip_id = PipId(uuid::Uuid::parse_str(&pip_id).map_err(|e| e.to_string())?);
        core.pip_manager.read().await.get_pip(pip_id).await
    } else {
        core.pip_manager.read().await.get_active_pip().await
    };

    Ok(window)
}

#[tauri::command]
pub async fn pip_set_muted(
    state: State<'_, AppState>,
    pip_id: String,
    muted: bool,
) -> Result<(), String> {
    let core = state.core.read().await;
    let pip_id = PipId(uuid::Uuid::parse_str(&pip_id).map_err(|e| e.to_string())?);

    core.pip_manager.write().await
        .set_muted(pip_id, muted)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn pip_set_rate(
    state: State<'_, AppState>,
    pip_id: String,
    rate: f64,
) -> Result<(), String> {
    let core = state.core.read().await;
    let pip_id = PipId(uuid::Uuid::parse_str(&pip_id).map_err(|e| e.to_string())?);

    core.pip_manager.write().await
        .set_playback_rate(pip_id, rate)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// Telemetry Commands
// ============================================================================

#[tauri::command]
pub async fn telemetry_event(
    state: State<'_, AppState>,
    request: TelemetryEventRequest,
) -> Result<(), String> {
    let core = state.core.read().await;

    let event_type = match request.event_type.as_str() {
        "AppStart" => TelemetryEventType::AppStart,
        "AppExit" => TelemetryEventType::AppExit,
        "WindowOpen" => TelemetryEventType::WindowOpen,
        "WindowClose" => TelemetryEventType::WindowClose,
        "TabOpen" => TelemetryEventType::TabOpen,
        "TabClose" => TelemetryEventType::TabClose,
        "TabNavigate" => TelemetryEventType::TabNavigate,
        "TabReload" => TelemetryEventType::TabReload,
        "Search" => TelemetryEventType::Search,
        "BookmarkAdd" => TelemetryEventType::BookmarkAdd,
        "BookmarkRemove" => TelemetryEventType::BookmarkRemove,
        "HistoryClear" => TelemetryEventType::HistoryClear,
        "DownloadStart" => TelemetryEventType::DownloadStart,
        "DownloadComplete" => TelemetryEventType::DownloadComplete,
        "DownloadCancel" => TelemetryEventType::DownloadCancel,
        "ExtensionInstall" => TelemetryEventType::ExtensionInstall,
        "ExtensionEnable" => TelemetryEventType::ExtensionEnable,
        "ExtensionDisable" => TelemetryEventType::ExtensionDisable,
        "SettingsChange" => TelemetryEventType::SettingsChange,
        "ThemeChange" => TelemetryEventType::ThemeChange,
        "ProfileSwitch" => TelemetryEventType::ProfileSwitch,
        "PasswordSave" => TelemetryEventType::PasswordSave,
        "PasswordFill" => TelemetryEventType::PasswordFill,
        "ContentBlock" => TelemetryEventType::ContentBlock,
        "ContentAllow" => TelemetryEventType::ContentAllow,
        "PipEnter" => TelemetryEventType::PipEnter,
        "PipExit" => TelemetryEventType::PipExit,
        "Crash" => TelemetryEventType::Crash,
        "Error" => TelemetryEventType::Error,
        "Performance" => TelemetryEventType::Performance,
        _ => TelemetryEventType::Error,
    };

    core.telemetry_manager.write().await
        .record_event(event_type, request.profile_id, request.properties.unwrap_or_default(), request.duration_ms)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn telemetry_flush(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let core = state.core.read().await;

    core.telemetry_manager.write().await
        .flush()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================================
// IPC Commands
// ============================================================================

#[tauri::command]
pub async fn ipc_request(
    state: State<'_, IpcState>,
    request: IpcRequestRequest,
) -> Result<Value, String> {
    state.0.send_request(&request.method, request.params, request.timeout_ms)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ipc_notify(
    state: State<'_, IpcState>,
    request: IpcNotifyRequest,
) -> Result<(), String> {
    state.0.send_notification(&request.method, request.params)
        .await
        .map_err(|e| e.to_string())
}