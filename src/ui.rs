use anyhow::Result;
use log::info;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct UI {
    title: String,
}

impl UI {
    pub async fn new(title: &str) -> Result<Self> {
        info!("Initializing UI: {}", title);

        Ok(Self {
            title: title.to_string(),
        })
    }

    pub async fn start(&self, app: &crate::app::BrowserApp) -> Result<()> {
        info!("🌐 {} Starting...", self.title);
        info!("📡 Backend initialized and ready");
        info!("🔗 Open http://localhost:8080 in your browser");

        // Create initial window
        let window_id = app.browser_engine.create_window().await?;
        info!("✓ Main window created: {}", window_id);

        // Create initial tab
        let tab = app
            .browser_engine
            .create_tab(&window_id, "about:home")
            .await?;
        info!("✓ Initial tab created: {}", tab.id);

        // Start the web server with the existing app
        let app_clone = Arc::new(RwLock::new(crate::app::BrowserApp::new(
            app.app_data_dir.clone(),
            app.settings.read().await.clone(),
            app.permission_manager.clone(),
            app.extension_manager.clone(),
        ).await?));
        
        crate::web::start_server(app_clone).await?;

        Ok(())
    }

    pub fn render_settings_panel(&self) {}

    pub fn render_permissions_panel(&self) {}

    pub fn render_extensions_panel(&self) {}
}

