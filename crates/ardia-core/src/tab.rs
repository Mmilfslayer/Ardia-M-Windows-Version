use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use url::Url;
use serde::{Serialize, Deserialize};

use crate::error::CoreResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TabId(pub Uuid);

impl TabId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TabId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TabId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabState {
    pub id: TabId,
    pub window_id: String, // WindowId as string for serialization
    pub url: Option<Url>,
    pub title: String,
    pub favicon: Option<Vec<u8>>,
    pub loading: bool,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub is_pinned: bool,
    pub is_incognito: bool,
    pub is_muted: bool,
    pub index: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    // Page state for session restore
    pub scroll_position: Option<[f64; 2]>,
    pub zoom_level: f64,
    pub form_data: Option<serde_json::Value>,
}

impl TabState {
    pub fn new(window_id: String, url: Option<Url>, is_incognito: bool) -> Self {
        Self {
            id: TabId::new(),
            window_id,
            url,
            title: "New Tab".to_string(),
            favicon: None,
            loading: false,
            can_go_back: false,
            can_go_forward: false,
            is_pinned: false,
            is_incognito,
            is_muted: false,
            index: 0,
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
            scroll_position: None,
            zoom_level: 1.0,
            form_data: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageState {
    pub url: Url,
    pub title: String,
    pub scroll_position: [f64; 2],
    pub zoom_level: f64,
    pub form_data: serde_json::Value,
    pub history_entries: Vec<HistoryEntry>,
    pub current_history_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: Url,
    pub title: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct TabManager {
    tabs: HashMap<TabId, Arc<RwLock<TabState>>>,
    tab_order: Vec<TabId>, // Ordered by visual position
    active_tab: Option<TabId>,
    window_tabs: HashMap<String, Vec<TabId>>, // WindowId -> TabIds
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            tab_order: Vec::new(),
            active_tab: None,
            window_tabs: HashMap::new(),
        }
    }

    pub fn create_tab(&mut self, window_id: String, url: Option<Url>, is_incognito: bool) -> TabId {
        let tab = TabState::new(window_id.clone(), url, is_incognito);
        let id = tab.id;

        // Add to window's tab list
        let window_tabs = self.window_tabs.entry(window_id.clone()).or_default();
        tab.index = window_tabs.len();
        window_tabs.push(id);

        // Add to global order
        self.tab_order.push(id);

        // Set as active if it's the first tab
        if self.active_tab.is_none() {
            self.active_tab = Some(id);
        }

        let tab_arc = Arc::new(RwLock::new(tab));
        self.tabs.insert(id, tab_arc);

        id
    }

    pub fn close_tab(&mut self, tab_id: TabId) -> CoreResult<()> {
        if let Some(tab) = self.tabs.remove(&tab_id) {
            // Remove from window tabs
            if let Some(window_tabs) = self.window_tabs.get_mut(&tab.read().await.window_id) {
                window_tabs.retain(|&id| id != tab_id);
                // Reindex
                for (index, &id) in window_tabs.iter().enumerate() {
                    if let Some(t) = self.tabs.get(&id) {
                        let mut t = t.write().await;
                        t.index = index;
                    }
                }
            }

            // Remove from global order
            self.tab_order.retain(|&id| id != tab_id);

            // Update active tab if needed
            if self.active_tab == Some(tab_id) {
                self.active_tab = self.tab_order.last().copied();
            }
        }
        Ok(())
    }

    pub fn get_tab(&self, tab_id: TabId) -> Option<Arc<RwLock<TabState>>> {
        self.tabs.get(&tab_id).cloned()
    }

    pub fn get_active_tab(&self) -> Option<TabId> {
        self.active_tab
    }

    pub fn set_active_tab(&mut self, tab_id: TabId) -> CoreResult<()> {
        if self.tabs.contains_key(&tab_id) {
            self.active_tab = Some(tab_id);
            // Update last accessed
            if let Some(tab) = self.tabs.get(&tab_id) {
                let mut tab = tab.write().await;
                tab.last_accessed = chrono::Utc::now();
            }
        }
        Ok(())
    }

    pub fn get_window_tabs(&self, window_id: &str) -> Vec<TabId> {
        self.window_tabs.get(window_id).cloned().unwrap_or_default()
    }

    pub fn reorder_tabs(&mut self, window_id: &str, tab_ids: Vec<TabId>) -> CoreResult<()> {
        if let Some(window_tabs) = self.window_tabs.get_mut(window_id) {
            *window_tabs = tab_ids;
            for (index, &id) in window_tabs.iter().enumerate() {
                if let Some(tab) = self.tabs.get(&id) {
                    let mut tab = tab.write().await;
                    tab.index = index;
                }
            }
        }
        Ok(())
    }

    pub fn move_tab(&mut self, tab_id: TabId, target_window_id: String, target_index: usize) -> CoreResult<()> {
        // Remove from current window
        if let Some(tab) = self.tabs.get(&tab_id) {
            let mut tab = tab.write().await;
            let old_window_id = tab.window_id.clone();
            if let Some(old_window_tabs) = self.window_tabs.get_mut(&old_window_id) {
                old_window_tabs.retain(|&id| id != tab_id);
                for (index, &id) in old_window_tabs.iter().enumerate() {
                    if let Some(t) = self.tabs.get(&id) {
                        let mut t = t.write().await;
                        t.index = index;
                    }
                }
            }

            // Add to new window
            tab.window_id = target_window_id.clone();
            let new_window_tabs = self.window_tabs.entry(target_window_id).or_default();
            let index = target_index.min(new_window_tabs.len());
            new_window_tabs.insert(index, tab_id);
            for (index, &id) in new_window_tabs.iter().enumerate() {
                if let Some(t) = self.tabs.get(&id) {
                    let mut t = t.write().await;
                    t.index = index;
                }
            }
        }
        Ok(())
    }

    pub fn update_tab_url(&mut self, tab_id: TabId, url: Url, title: String) -> CoreResult<()> {
        if let Some(tab) = self.tabs.get(&tab_id) {
            let mut tab = tab.write().await;
            tab.url = Some(url);
            tab.title = title;
            tab.last_accessed = chrono::Utc::now();
        }
        Ok(())
    }

    pub fn update_tab_loading(&mut self, tab_id: TabId, loading: bool) -> CoreResult<()> {
        if let Some(tab) = self.tabs.get(&tab_id) {
            let mut tab = tab.write().await;
            tab.loading = loading;
        }
        Ok(())
    }

    pub fn update_tab_navigation(&mut self, tab_id: TabId, can_go_back: bool, can_go_forward: bool) -> CoreResult<()> {
        if let Some(tab) = self.tabs.get(&tab_id) {
            let mut tab = tab.write().await;
            tab.can_go_back = can_go_back;
            tab.can_go_forward = can_go_forward;
        }
        Ok(())
    }

    pub fn set_tab_favicon(&mut self, tab_id: TabId, favicon: Vec<u8>) -> CoreResult<()> {
        if let Some(tab) = self.tabs.get(&tab_id) {
            let mut tab = tab.write().await;
            tab.favicon = Some(favicon);
        }
        Ok(())
    }

    pub fn pin_tab(&mut self, tab_id: TabId, pinned: bool) -> CoreResult<()> {
        if let Some(tab) = self.tabs.get(&tab_id) {
            let mut tab = tab.write().await;
            tab.is_pinned = pinned;
        }
        Ok(())
    }

    pub fn mute_tab(&mut self, tab_id: TabId, muted: bool) -> CoreResult<()> {
        if let Some(tab) = self.tabs.get(&tab_id) {
            let mut tab = tab.write().await;
            tab.is_muted = muted;
        }
        Ok(())
    }

    pub fn get_all_tabs(&self) -> Vec<TabId> {
        self.tab_order.clone()
    }

    pub fn get_tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn get_window_tab_count(&self, window_id: &str) -> usize {
        self.window_tabs.get(window_id).map(|v| v.len()).unwrap_or(0)
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}