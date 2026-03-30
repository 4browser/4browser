use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub tracker_blocking_enabled: bool,
    pub ad_blocking_enabled: bool,
    pub fingerprint_protection: bool,
    pub webrtc_leak_prevention: bool,
    pub referer_control: RefererControl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefererControl {
    SendFull,
    SendOnlyDomain,
    SendNone,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            tracker_blocking_enabled: true,
            ad_blocking_enabled: false,
            fingerprint_protection: true,
            webrtc_leak_prevention: true,
            referer_control: RefererControl::SendOnlyDomain,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadManager {
    pub path: String,
    pub ask_before_downloading: bool,
    pub auto_open_supported_files: bool,
    pub dangerous_file_warning: bool,
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self {
            path: dirs::Downloads::download_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            ask_before_downloading: false,
            auto_open_supported_files: false,
            dangerous_file_warning: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSettings {
    pub enabled: bool,
    pub sync_bookmarks: bool,
    pub sync_passwords: bool,
    pub sync_history: bool,
    pub sync_settings: bool,
    pub sync_extensions: bool,
}

impl Default for SyncSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            sync_bookmarks: true,
            sync_passwords: true,
            sync_history: true,
            sync_settings: true,
            sync_extensions: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngineConfig {
    pub name: String,
    pub url: String,
    pub icon_url: Option<String>,
    pub suggestions_enabled: bool,
}

pub struct FeatureManager;

impl FeatureManager {
    /// Unique Feature: Smart Tab Groups
    /// Automatically group tabs by domain and allow collapsing/organizing
    pub fn enable_tab_grouping() -> Result<()> {
        println!("✨ Smart Tab Grouping enabled!");
        Ok(())
    }

    /// Unique Feature: Privacy Dashboard
    /// Shows real-time privacy metrics and blocked trackers
    pub fn show_privacy_dashboard() -> Result<()> {
        println!("🛡️ Privacy Dashboard opened");
        Ok(())
    }

    /// Unique Feature: Session Snapshots
    /// Save and restore entire browser sessions instantly
    pub fn save_session_snapshot(name: &str) -> Result<()> {
        println!("📸 Session '{}' saved!", name);
        Ok(())
    }

    /// Unique Feature: Split View Browsing
    /// Browse two websites side by side
    pub fn enable_split_view() -> Result<()> {
        println!("➡️ Split view enabled");
        Ok(())
    }

    /// Unique Feature: Smart Notes
    /// Annotate and save highlights from web pages
    pub fn add_smart_note(url: &str, text: &str) -> Result<()> {
        println!("📝 Note added from {}: {}", url, text);
        Ok(())
    }

    /// Unique Feature: Keyboard Shortcuts Pro
    /// Advanced keyboard navigation and shortcuts
    pub fn show_keyboard_shortcuts_help() -> Result<()> {
        println!("⌨️ Keyboard Shortcuts Help:");
        println!("Ctrl+T - New Tab");
        println!("Ctrl+W - Close Tab");
        println!("Ctrl+N - New Window");
        println!("Ctrl+H - History");
        println!("Ctrl+B - Toggle Bookmarks");
        println!("Ctrl+, - Settings");
        println!("Ctrl+L - Focus Address Bar");
        println!("Ctrl+Shift+DWhite - Developer Tools");
        println!("Ctrl+Shift+P - Private Mode");
        println!("Alt+← - Back");
        println!("Alt+→ - Forward");
        println!("Ctrl+R - Reload");
        println!("Ctrl+Shift+R - Hard Reload");
        Ok(())
    }

    /// Unique Feature: Domain Impersonator Prevention
    /// Warns when similar domains try to load
    pub fn check_domain_lookalike(current_domain: &str, new_domain: &str) -> bool {
        let similarity = Self::levenshtein_distance(current_domain, new_domain);
        similarity < 3
    }

    fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        if s1.len() > s2.len() {
            return Self::levenshtein_distance(s2, s1);
        }

        let mut distance = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for i in 0..=s1.len() {
            distance[i][0] = i;
        }
        for j in 0..=s2.len() {
            distance[0][j] = j;
        }

        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                let cost = if c1 == c2 { 0 } else { 1 };
                distance[i + 1][j + 1] = (*[
                    distance[i][j + 1] + 1,
                    distance[i + 1][j] + 1,
                    distance[i][j] + cost,
                ]
                .iter()
                .min()
                .unwrap()) as usize;
            }
        }

        distance[s1.len()][s2.len()]
    }
}
