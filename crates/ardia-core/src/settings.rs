use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};
use dirs;

use crate::error::CoreResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub appearance: AppearanceSettings,
    pub tabs: TabSettings,
    pub privacy: PrivacySettings,
    pub advanced: AdvancedSettings,
    pub shortcuts: ShortcutSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            appearance: AppearanceSettings::default(),
            tabs: TabSettings::default(),
            privacy: PrivacySettings::default(),
            advanced: AdvancedSettings::default(),
            shortcuts: ShortcutSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub language: String,
    pub default_profile: Option<String>,
    pub startup_behavior: StartupBehavior,
    pub check_updates: bool,
    pub send_telemetry: bool,
    pub open_devtools_on_startup: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            language: "en-US".to_string(),
            default_profile: None,
            startup_behavior: StartupBehavior::RestoreSession,
            check_updates: true,
            send_telemetry: false,
            open_devtools_on_startup: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StartupBehavior {
    NewTab,
    Homepage,
    RestoreSession,
    Blank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme_mode: ThemeMode,
    pub accent_color: Option<String>,
    pub use_system_accent: bool,
    pub show_bookmarks_bar: bool,
    pub show_home_button: bool,
    pub compact_mode: bool,
    pub sidebar_width: f64,
    pub font_size: f64,
    pub font_family: String,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::System,
            accent_color: None,
            use_system_accent: true,
            show_bookmarks_bar: true,
            show_home_button: true,
            compact_mode: false,
            sidebar_width: 280.0,
            font_size: 14.0,
            font_family: "system-ui".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabSettings {
    pub tab_position: TabPosition,
    pub show_tab_close_button: bool,
    pub double_click_to_close: bool,
    pub new_tab_position: NewTabPosition,
    pub switch_to_new_tab: bool,
    pub tab_wrap: bool,
    pub min_tab_width: f64,
    pub max_tab_width: f64,
    pub show_favicon: bool,
    pub show_mute_button: bool,
}

impl Default for TabSettings {
    fn default() -> Self {
        Self {
            tab_position: TabPosition::Top,
            show_tab_close_button: true,
            double_click_to_close: true,
            new_tab_position: NewTabPosition::AfterCurrent,
            switch_to_new_tab: true,
            tab_wrap: true,
            min_tab_width: 120.0,
            max_tab_width: 280.0,
            show_favicon: true,
            show_mute_button: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NewTabPosition {
    AfterCurrent,
    AtEnd,
    AtBeginning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub block_third_party_cookies: bool,
    pub block_all_cookies: bool,
    pub clear_on_exit: bool,
    pub send_do_not_track: bool,
    pub safe_browsing: SafeBrowsingLevel,
    pub search_suggestions: bool,
    pub url_suggestions: bool,
    pub dns_over_https: DnsOverHttpsMode,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            block_third_party_cookies: true,
            block_all_cookies: false,
            clear_on_exit: false,
            send_do_not_track: true,
            safe_browsing: SafeBrowsingLevel::Standard,
            search_suggestions: true,
            url_suggestions: true,
            dns_over_https: DnsOverHttpsMode::Off,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafeBrowsingLevel {
    Off,
    Standard,
    Enhanced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsOverHttpsMode {
    Off,
    Automatic,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub hardware_acceleration: bool,
    pub enable_webgl: bool,
    pub enable_wasm: bool,
    pub proxy_mode: ProxyMode,
    pub custom_proxy: Option<String>,
    pub certificate_revocation_check: bool,
    pub experimental_features: bool,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        Self {
            hardware_acceleration: true,
            enable_webgl: true,
            enable_wasm: true,
            proxy_mode: ProxyMode::System,
            custom_proxy: None,
            certificate_revocation_check: true,
            experimental_features: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyMode {
    System,
    None,
    Manual,
    AutoDetect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutSettings {
    pub new_tab: String,
    pub new_window: String,
    pub new_incognito: String,
    pub close_tab: String,
    pub reopen_closed_tab: String,
    pub next_tab: String,
    pub prev_tab: String,
    pub focus_address_bar: String,
    pub open_downloads: String,
    pub open_history: String,
    pub open_bookmarks: String,
    pub open_extensions: String,
    pub open_settings: String,
    pub toggle_devtools: String,
    pub zoom_in: String,
    pub zoom_out: String,
    pub zoom_reset: String,
    pub find_in_page: String,
    pub print_page: String,
    pub fullscreen: String,
}

impl Default for ShortcutSettings {
    fn default() -> Self {
        Self {
            new_tab: "Ctrl+T".to_string(),
            new_window: "Ctrl+N".to_string(),
            new_incognito: "Ctrl+Shift+N".to_string(),
            close_tab: "Ctrl+W".to_string(),
            reopen_closed_tab: "Ctrl+Shift+T".to_string(),
            next_tab: "Ctrl+Tab".to_string(),
            prev_tab: "Ctrl+Shift+Tab".to_string(),
            focus_address_bar: "Ctrl+L".to_string(),
            open_downloads: "Ctrl+J".to_string(),
            open_history: "Ctrl+H".to_string(),
            open_bookmarks: "Ctrl+Shift+B".to_string(),
            open_extensions: "Ctrl+Shift+E".to_string(),
            open_settings: "Ctrl+Shift+P".to_string(),
            toggle_devtools: "F12".to_string(),
            zoom_in: "Ctrl+=".to_string(),
            zoom_out: "Ctrl+-".to_string(),
            zoom_reset: "Ctrl+0".to_string(),
            find_in_page: "Ctrl+F".to_string(),
            print_page: "Ctrl+P".to_string(),
            fullscreen: "F11".to_string(),
        }
    }
}

pub struct SettingsManager {
    config: Arc<RwLock<Config>>,
    settings_path: PathBuf,
    settings: Arc<RwLock<AppSettings>>,
}

impl SettingsManager {
    pub async fn new() -> CoreResult<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Ardia");

        std::fs::create_dir_all(&config_dir)?;

        let settings_path = config_dir.join("settings.toml");

        let config = Config::builder()
            .add_source(File::new(settings_path.to_str().unwrap(), FileFormat::Toml).required(false))
            .add_source(config::Environment::with_prefix("ARDIA"))
            .build()?;

        let settings = config.clone().try_deserialize::<AppSettings>().unwrap_or_default();

        let manager = Self {
            config: Arc::new(RwLock::new(config)),
            settings_path,
            settings: Arc::new(RwLock::new(settings)),
        };

        // Save defaults if file doesn't exist
        if !manager.settings_path.exists() {
            manager.save().await?;
        }

        Ok(manager)
    }

    pub async fn get_settings(&self) -> AppSettings {
        self.settings.read().await.clone()
    }

    pub async fn get_general(&self) -> GeneralSettings {
        self.settings.read().await.general.clone()
    }

    pub async fn get_appearance(&self) -> AppearanceSettings {
        self.settings.read().await.appearance.clone()
    }

    pub async fn get_tabs(&self) -> TabSettings {
        self.settings.read().await.tabs.clone()
    }

    pub async fn get_privacy(&self) -> PrivacySettings {
        self.settings.read().await.privacy.clone()
    }

    pub async fn get_advanced(&self) -> AdvancedSettings {
        self.settings.read().await.advanced.clone()
    }

    pub async fn get_shortcuts(&self) -> ShortcutSettings {
        self.settings.read().await.shortcuts.clone()
    }

    pub async fn update_general(&self, settings: GeneralSettings) -> CoreResult<()> {
        let mut s = self.settings.write().await;
        s.general = settings;
        drop(s);
        self.save().await?;
        Ok(())
    }

    pub async fn update_appearance(&self, settings: AppearanceSettings) -> CoreResult<()> {
        let mut s = self.settings.write().await;
        s.appearance = settings;
        drop(s);
        self.save().await?;
        Ok(())
    }

    pub async fn update_tabs(&self, settings: TabSettings) -> CoreResult<()> {
        let mut s = self.settings.write().await;
        s.tabs = settings;
        drop(s);
        self.save().await?;
        Ok(())
    }

    pub async fn update_privacy(&self, settings: PrivacySettings) -> CoreResult<()> {
        let mut s = self.settings.write().await;
        s.privacy = settings;
        drop(s);
        self.save().await?;
        Ok(())
    }

    pub async fn update_advanced(&self, settings: AdvancedSettings) -> CoreResult<()> {
        let mut s = self.settings.write().await;
        s.advanced = settings;
        drop(s);
        self.save().await?;
        Ok(())
    }

    pub async fn update_shortcuts(&self, settings: ShortcutSettings) -> CoreResult<()> {
        let mut s = self.settings.write().await;
        s.shortcuts = settings;
        drop(s);
        self.save().await?;
        Ok(())
    }

    pub async fn set_value(&self, key: &str, value: serde_json::Value) -> CoreResult<()> {
        // For dynamic settings not in the structured config
        // Could use a separate key-value store
        let mut config = self.config.write().await;
        config.set(key, value)?;
        drop(config);
        self.save().await?;
        Ok(())
    }

    pub async fn get_value(&self, key: &str) -> CoreResult<Option<serde_json::Value>> {
        let config = self.config.read().await;
        match config.get(key) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        }
    }

    async fn save(&self) -> CoreResult<()> {
        let settings = self.settings.read().await;
        let toml = toml::to_string_pretty(&*settings)?;
        std::fs::write(&self.settings_path, toml)?;
        Ok(())
    }

    pub async fn reset_to_defaults(&self) -> CoreResult<()> {
        let defaults = AppSettings::default();
        let mut settings = self.settings.write().await;
        *settings = defaults;
        drop(settings);
        self.save().await?;
        Ok(())
    }

    pub fn get_settings_path(&self) -> &PathBuf {
        &self.settings_path
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        // This is just for compilation; actual initialization is async
        Self {
            config: Arc::new(RwLock::new(Config::default())),
            settings_path: PathBuf::from("."),
            settings: Arc::new(RwLock::new(AppSettings::default())),
        }
    }
}