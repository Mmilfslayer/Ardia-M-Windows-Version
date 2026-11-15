use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::{error::CoreResult, settings::SettingsManager};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProfileId(pub Uuid);

impl ProfileId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ProfileId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ProfileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: ProfileId,
    pub name: String,
    pub path: PathBuf,
    pub settings: ProfileSettings,
    pub extensions: Vec<String>, // Extension IDs
    pub theme: ThemeConfig,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used: chrono::DateTime<chrono::Utc>,
    pub is_default: bool,
}

impl Profile {
    pub fn new(name: String, path: PathBuf, is_default: bool) -> Self {
        let id = ProfileId::new();
        Self {
            id,
            name,
            path,
            settings: ProfileSettings::default(),
            extensions: Vec::new(),
            theme: ThemeConfig::default(),
            created_at: chrono::Utc::now(),
            last_used: chrono::Utc::now(),
            is_default,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSettings {
    pub start_page: StartPage,
    pub search_engine: String,
    pub download_path: PathBuf,
    pub ask_download_location: bool,
    pub enable_autofill: bool,
    pub enable_passwords: bool,
    pub enable_cookies: bool,
    pub enable_javascript: bool,
    pub enable_content_blocker: bool,
    pub privacy_level: PrivacyLevel,
    pub custom_user_agent: Option<String>,
    pub homepage: Option<String>,
}

impl Default for ProfileSettings {
    fn default() -> Self {
        Self {
            start_page: StartPage::NewTab,
            search_engine: "https://www.google.com/search?q={}".to_string(),
            download_path: dirs::download_dir().unwrap_or_else(|| PathBuf::from(".")),
            ask_download_location: false,
            enable_autofill: true,
            enable_passwords: true,
            enable_cookies: true,
            enable_javascript: true,
            enable_content_blocker: true,
            privacy_level: PrivacyLevel::Standard,
            custom_user_agent: None,
            homepage: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StartPage {
    NewTab,
    Homepage,
    PreviousSession,
    Blank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivacyLevel {
    Standard,
    Strict,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub mode: ThemeMode,
    pub accent_color: Option<String>,
    pub custom_css: Option<String>,
    pub use_system_theme: bool,
    pub backdrop_opacity: f64,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            mode: ThemeMode::System,
            accent_color: None,
            custom_css: None,
            use_system_theme: true,
            backdrop_opacity: 0.8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

pub struct ProfileManager {
    profiles: HashMap<ProfileId, Arc<RwLock<Profile>>>,
    default_profile: Option<ProfileId>,
    settings_manager: Arc<RwLock<SettingsManager>>,
    profiles_dir: PathBuf,
}

impl ProfileManager {
    pub async fn new(settings_manager: Arc<RwLock<SettingsManager>>) -> CoreResult<Self> {
        let profiles_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Ardia")
            .join("Profiles");

        std::fs::create_dir_all(&profiles_dir)?;

        let mut manager = Self {
            profiles: HashMap::new(),
            default_profile: None,
            settings_manager,
            profiles_dir,
        };

        // Load existing profiles
        manager.load_profiles().await?;

        // Create default profile if none exists
        if manager.profiles.is_empty() {
            manager.create_default_profile().await?;
        }

        Ok(manager)
    }

    async fn load_profiles(&mut self) -> CoreResult<()> {
        let entries = std::fs::read_dir(&self.profiles_dir)?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let profile_json = path.join("profile.json");
                if profile_json.exists() {
                    let content = std::fs::read_to_string(&profile_json)?;
                    if let Ok(profile) = serde_json::from_str::<Profile>(&content) {
                        let id = profile.id;
                        let is_default = profile.is_default;
                        self.profiles.insert(id, Arc::new(RwLock::new(profile)));
                        if is_default {
                            self.default_profile = Some(id);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn create_default_profile(&mut self) -> CoreResult<()> {
        let profile_path = self.profiles_dir.join("default");
        std::fs::create_dir_all(&profile_path)?;

        let mut profile = Profile::new(
            "Default".to_string(),
            profile_path,
            true,
        );

        profile.settings.download_path = dirs::download_dir().unwrap_or_else(|| PathBuf::from("."));

        let id = profile.id;
        self.profiles.insert(id, Arc::new(RwLock::new(profile)));
        self.default_profile = Some(id);

        self.save_profile(id).await?;
        Ok(())
    }

    pub async fn create_profile(&mut self, name: String) -> CoreResult<ProfileId> {
        let profile_name = name.trim().to_string();
        if profile_name.is_empty() {
            return Err(crate::error::CoreError::InvalidState("Profile name cannot be empty".to_string()));
        }

        let sanitized_name = profile_name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' { c } else { '_' })
            .collect::<String>()
            .replace(' ', "_");

        let profile_path = self.profiles_dir.join(&sanitized_name);
        std::fs::create_dir_all(&profile_path)?;

        let profile = Profile::new(profile_name, profile_path, false);
        let id = profile.id;

        self.profiles.insert(id, Arc::new(RwLock::new(profile)));
        self.save_profile(id).await?;

        Ok(id)
    }

    pub async fn delete_profile(&mut self, id: ProfileId) -> CoreResult<()> {
        if self.profiles.len() <= 1 {
            return Err(crate::error::CoreError::InvalidState("Cannot delete the last profile".to_string()));
        }

        if let Some(profile) = self.profiles.remove(&id) {
            let profile = profile.read().await;
            if profile.is_default {
                return Err(crate::error::CoreError::InvalidState("Cannot delete default profile".to_string()));
            }

            // Remove profile directory
            if profile.path.exists() {
                std::fs::remove_dir_all(&profile.path)?;
            }
        }

        // Update default if needed
        if self.default_profile == Some(id) {
            if let Some((&new_default, _)) = self.profiles.iter().next() {
                self.default_profile = Some(new_default);
                if let Some(profile) = self.profiles.get(&new_default) {
                    let mut profile = profile.write().await;
                    profile.is_default = true;
                    drop(profile);
                    self.save_profile(new_default).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn get_profile(&self, id: ProfileId) -> Option<Arc<RwLock<Profile>>> {
        self.profiles.get(&id).cloned()
    }

    pub async fn get_default_profile(&self) -> Option<ProfileId> {
        self.default_profile
    }

    pub async fn set_default_profile(&mut self, id: ProfileId) -> CoreResult<()> {
        if !self.profiles.contains_key(&id) {
            return Err(crate::error::CoreError::NotFound("Profile not found".to_string()));
        }

        // Unset current default
        if let Some(current_default) = self.default_profile {
            if let Some(profile) = self.profiles.get(&current_default) {
                let mut profile = profile.write().await;
                profile.is_default = false;
            }
        }

        // Set new default
        if let Some(profile) = self.profiles.get(&id) {
            let mut profile = profile.write().await;
            profile.is_default = true;
        }

        self.default_profile = Some(id);
        self.save_profile(id).await?;
        Ok(())
    }

    pub async fn list_profiles(&self) -> Vec<ProfileId> {
        self.profiles.keys().copied().collect()
    }

    pub async fn update_profile_name(&self, id: ProfileId, name: String) -> CoreResult<()> {
        if let Some(profile) = self.profiles.get(&id) {
            let mut profile = profile.write().await;
            profile.name = name;
            profile.last_used = chrono::Utc::now();
        }
        self.save_profile(id).await?;
        Ok(())
    }

    pub async fn update_profile_settings(&self, id: ProfileId, settings: ProfileSettings) -> CoreResult<()> {
        if let Some(profile) = self.profiles.get(&id) {
            let mut profile = profile.write().await;
            profile.settings = settings;
            profile.last_used = chrono::Utc::now();
        }
        self.save_profile(id).await?;
        Ok(())
    }

    pub async fn update_profile_theme(&self, id: ProfileId, theme: ThemeConfig) -> CoreResult<()> {
        if let Some(profile) = self.profiles.get(&id) {
            let mut profile = profile.write().await;
            profile.theme = theme;
            profile.last_used = chrono::Utc::now();
        }
        self.save_profile(id).await?;
        Ok(())
    }

    pub async fn add_extension(&self, id: ProfileId, extension_id: String) -> CoreResult<()> {
        if let Some(profile) = self.profiles.get(&id) {
            let mut profile = profile.write().await;
            if !profile.extensions.contains(&extension_id) {
                profile.extensions.push(extension_id);
                profile.last_used = chrono::Utc::now();
            }
        }
        self.save_profile(id).await?;
        Ok(())
    }

    pub async fn remove_extension(&self, id: ProfileId, extension_id: &str) -> CoreResult<()> {
        if let Some(profile) = self.profiles.get(&id) {
            let mut profile = profile.write().await;
            profile.extensions.retain(|e| e != extension_id);
            profile.last_used = chrono::Utc::now();
        }
        self.save_profile(id).await?;
        Ok(())
    }

    async fn save_profile(&self, id: ProfileId) -> CoreResult<()> {
        if let Some(profile) = self.profiles.get(&id) {
            let profile = profile.read().await;
            let profile_json = profile.path.join("profile.json");
            let content = serde_json::to_string_pretty(&*profile)?;
            std::fs::write(&profile_json, content)?;
        }
        Ok(())
    }

    pub async fn save(&self) -> CoreResult<()> {
        for id in self.profiles.keys() {
            self.save_profile(*id).await?;
        }
        Ok(())
    }

    pub fn get_profile_path(&self, id: ProfileId) -> Option<PathBuf> {
        self.profiles.get(&id).map(|p| p.blocking_read().path.clone())
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        // This is just for compilation; actual initialization is async
        Self {
            profiles: HashMap::new(),
            default_profile: None,
            settings_manager: Arc::new(RwLock::new(SettingsManager::default())),
            profiles_dir: PathBuf::from("."),
        }
    }
}