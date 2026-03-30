use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme: String,           // "light", "dark", "system"
    pub accent_color: String,    // hex color
    pub font_size: u16,
    pub font_family: String,
    pub compact_mode: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            accent_color: "#FF6B35".to_string(),
            font_size: 12,
            font_family: "Segoe UI, -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
            compact_mode: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub block_trackers: bool,
    pub block_ads: bool,
    pub do_not_track: bool,
    pub block_third_party_cookies: bool,
    pub clear_history_on_exit: bool,
    pub auto_delete_cookies_after_days: u16,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            block_trackers: true,
            block_ads: false,
            do_not_track: true,
            block_third_party_cookies: true,
            clear_history_on_exit: false,
            auto_delete_cookies_after_days: 365,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettings {
    pub default_search_engine: String,
    pub home_page: String,
    pub startup_behavior: String,
    pub download_location: PathBuf,
    pub auto_update: bool,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            default_search_engine: "https://www.google.com/search?q=".to_string(),
            home_page: "about:home".to_string(),
            startup_behavior: "open_home".to_string(),
            download_location: dirs::download_dir().unwrap_or_else(|| PathBuf::from("~/Downloads")),
            auto_update: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub browser_name: String,
    pub device_name: String,
    pub custom_user_agent: Option<String>,
    pub appearance: AppearanceSettings,
    pub privacy: PrivacySettings,
    pub browser: BrowserSettings,
    pub extensions_enabled: bool,
    pub developer_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            browser_name: "4 Browser".to_string(),
            device_name: "Custom Device".to_string(),
            custom_user_agent: None,
            appearance: AppearanceSettings::default(),
            privacy: PrivacySettings::default(),
            browser: BrowserSettings::default(),
            extensions_enabled: true,
            developer_mode: false,
        }
    }
}

impl Settings {
    pub async fn load(app_data_dir: &PathBuf) -> Result<Self> {
        let settings_file = app_data_dir.join("settings.json");

        if settings_file.exists() {
            let content = tokio::fs::read_to_string(&settings_file).await?;
            let settings: Settings = serde_json::from_str(&content)?;
            info!("Loaded settings from file");
            return Ok(settings);
        }

        info!("Using default settings");
        Ok(Settings::default())
    }

    pub async fn save(&self, app_data_dir: &PathBuf) -> Result<()> {
        let settings_file = app_data_dir.join("settings.json");
        let content = serde_json::to_string_pretty(&self)?;
        tokio::fs::write(&settings_file, content).await?;
        info!("Settings saved to file");

        Ok(())
    }

    pub fn update_theme(&mut self, theme: String) {
        self.appearance.theme = theme;
    }

    pub fn update_accent_color(&mut self, color: String) {
        self.appearance.accent_color = color;
    }

    pub fn update_font_size(&mut self, size: u16) {
        self.appearance.font_size = size;
    }

    pub fn update_browser_name(&mut self, name: String) {
        self.browser_name = name;
    }

    pub fn update_device_name(&mut self, name: String) {
        self.device_name = name;
    }

    pub fn toggle_developer_mode(&mut self) {
        self.developer_mode = !self.developer_mode;
    }

    pub fn get_user_agent(&self) -> String {
        if let Some(custom_ua) = &self.custom_user_agent {
            return custom_ua.clone();
        }

        // Default user agent with custom browser name
        format!(
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) {}/1.0 Chrome/120.0.0.0 Safari/537.36",
            self.browser_name
        )
    }
}
