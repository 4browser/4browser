use crate::browser::BrowserEngine;
use crate::extensions::ExtensionManager;
use crate::permissions::PermissionManager;
use crate::settings::Settings;
use anyhow::Result;
use log::info;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct BrowserApp {
    pub name: String,
    pub version: String,
    pub app_data_dir: PathBuf,
    pub browser_engine: BrowserEngine,
    pub permission_manager: PermissionManager,
    pub extension_manager: ExtensionManager,
    pub settings: Arc<RwLock<Settings>>,
}

impl BrowserApp {
    pub async fn new(
        app_data_dir: PathBuf,
        settings: Settings,
        permission_manager: PermissionManager,
        extension_manager: ExtensionManager,
    ) -> Result<Self> {
        info!("Creating BrowserApp instance");

        let browser_engine = BrowserEngine::new(&app_data_dir).await?;

        let app = Self {
            name: settings.browser_name.clone(),
            version: "0.1.0".to_string(),
            app_data_dir,
            browser_engine,
            permission_manager,
            extension_manager,
            settings: Arc::new(RwLock::new(settings)),
        };

        info!("BrowserApp initialized: {} v{}", app.name, app.version);

        Ok(app)
    }

    pub async fn run(&self) -> Result<()> {
        info!("🚀 Starting {} application...", self.name);

        // Initialize UI
        let ui = crate::ui::UI::new(&self.name).await?;

        // Start the UI
        ui.start(self).await?;

        Ok(())
    }

    pub async fn get_user_agent(&self) -> String {
        self.settings.read().await.get_user_agent()
    }

    pub async fn update_browser_name(&self, name: String) -> Result<()> {
        self.settings.write().await.update_browser_name(name);
        self.settings
            .read()
            .await
            .save(&self.app_data_dir)
            .await?;

        Ok(())
    }

    pub async fn update_device_name(&self, name: String) -> Result<()> {
        self.settings.write().await.update_device_name(name);
        self.settings
            .read()
            .await
            .save(&self.app_data_dir)
            .await?;

        Ok(())
    }

    pub async fn get_stats(&self) -> BrowserStats {
        let history = self.browser_engine.get_history(1000).await;
        let bookmarks = self.browser_engine.get_bookmarks(None).await;
        let extensions = self.extension_manager.get_extensions().await;

        BrowserStats {
            total_tabs: 0,
            total_windows: 0,
            history_count: history.len(),
            bookmarks_count: bookmarks.len(),
            extensions_count: extensions.len(),
            memory_usage_mb: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BrowserStats {
    pub total_tabs: usize,
    pub total_windows: usize,
    pub history_count: usize,
    pub bookmarks_count: usize,
    pub extensions_count: usize,
    pub memory_usage_mb: f64,
}
