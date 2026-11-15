use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::error::CoreResult;
use crate::bookmarks::BookmarkManager;
use crate::history::HistoryManager;
use crate::extensions::ExtensionManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SidebarPanel {
    VerticalTabs,
    Bookmarks,
    History,
    Extensions,
    Downloads,
    Settings,
}

impl Default for SidebarPanel {
    fn default() -> Self {
        SidebarPanel::VerticalTabs
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarState {
    pub is_open: bool,
    pub active_panel: SidebarPanel,
    pub width: u32,
    pub is_collapsed: bool,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            is_open: true,
            active_panel: SidebarPanel::VerticalTabs,
            width: 280,
            is_collapsed: false,
        }
    }
}

pub struct SidebarManager {
    state: Arc<RwLock<SidebarState>>,
    bookmark_manager: Option<Arc<BookmarkManager>>,
    history_manager: Option<Arc<HistoryManager>>,
    extension_manager: Option<Arc<ExtensionManager>>,
    panel_states: Arc<RwLock<HashMap<SidebarPanel, PanelState>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelState {
    pub scroll_position: f64,
    pub search_query: String,
    pub filter: Option<String>,
    pub expanded_folders: Vec<String>,
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            scroll_position: 0.0,
            search_query: String::new(),
            filter: None,
            expanded_folders: Vec::new(),
        }
    }
}

impl SidebarManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(SidebarState::default())),
            bookmark_manager: None,
            history_manager: None,
            extension_manager: None,
            panel_states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn with_bookmarks(mut self, manager: Arc<BookmarkManager>) -> Self {
        self.bookmark_manager = Some(manager);
        self
    }

    pub fn with_history(mut self, manager: Arc<HistoryManager>) -> Self {
        self.history_manager = Some(manager);
        self
    }

    pub fn with_extensions(mut self, manager: Arc<ExtensionManager>) -> Self {
        self.extension_manager = Some(manager);
        self
    }

    pub async fn get_state(&self) -> SidebarState {
        self.state.read().await.clone()
    }

    pub async fn set_open(&self, open: bool) {
        let mut state = self.state.write().await;
        state.is_open = open;
    }

    pub async fn toggle(&self) -> bool {
        let mut state = self.state.write().await;
        state.is_open = !state.is_open;
        state.is_open
    }

    pub async fn set_active_panel(&self, panel: SidebarPanel) {
        let mut state = self.state.write().await;
        state.active_panel = panel;
    }

    pub async fn set_width(&self, width: u32) {
        let mut state = self.state.write().await;
        state.width = width.clamp(200, 600);
    }

    pub async fn set_collapsed(&self, collapsed: bool) {
        let mut state = self.state.write().await;
        state.is_collapsed = collapsed;
    }

    pub async fn get_panel_state(&self, panel: SidebarPanel) -> PanelState {
        let states = self.panel_states.read().await;
        states.get(&panel).cloned().unwrap_or_default()
    }

    pub async fn set_panel_state(&self, panel: SidebarPanel, state: PanelState) {
        let mut states = self.panel_states.write().await;
        states.insert(panel, state);
    }

    pub async fn update_scroll_position(&self, panel: SidebarPanel, position: f64) {
        let mut states = self.panel_states.write().await;
        if let Some(state) = states.get_mut(&panel) {
            state.scroll_position = position;
        }
    }

    pub async fn update_search_query(&self, panel: SidebarPanel, query: String) {
        let mut states = self.panel_states.write().await;
        if let Some(state) = states.get_mut(&panel) {
            state.search_query = query;
        }
    }

    pub async fn set_filter(&self, panel: SidebarPanel, filter: Option<String>) {
        let mut states = self.panel_states.write().await;
        if let Some(state) = states.get_mut(&panel) {
            state.filter = filter;
        }
    }

    pub async fn toggle_folder_expanded(&self, panel: SidebarPanel, folder_id: String) {
        let mut states = self.panel_states.write().await;
        if let Some(state) = states.get_mut(&panel) {
            if let Some(pos) = state.expanded_folders.iter().position(|f| f == &folder_id) {
                state.expanded_folders.remove(pos);
            } else {
                state.expanded_folders.push(folder_id);
            }
        }
    }

    pub async fn get_bookmark_tree(&self, profile_id: &str) -> CoreResult<Vec<crate::bookmarks::Bookmark>> {
        if let Some(mgr) = &self.bookmark_manager {
            mgr.get_tree(profile_id).await
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn get_history(&self, profile_id: &str, limit: usize) -> CoreResult<Vec<crate::history::HistoryEntry>> {
        if let Some(mgr) = &self.history_manager {
            mgr.get_history(profile_id, limit, 0).await
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn search_history(&self, profile_id: &str, query: &str, limit: usize) -> CoreResult<Vec<crate::history::HistoryEntry>> {
        if let Some(mgr) = &self.history_manager {
            mgr.search_history(profile_id, query, limit).await
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn search_bookmarks(&self, profile_id: &str, query: &str) -> CoreResult<Vec<crate::bookmarks::Bookmark>> {
        if let Some(mgr) = &self.bookmark_manager {
            mgr.search_bookmarks(profile_id, query).await
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn get_extensions(&self) -> CoreResult<Vec<crate::extensions::Extension>> {
        if let Some(mgr) = &self.extension_manager {
            mgr.get_all_extensions().await
        } else {
            Ok(Vec::new())
        }
    }
}

impl Default for SidebarManager {
    fn default() -> Self {
        Self::new()
    }
}