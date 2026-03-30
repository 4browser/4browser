mod app;
mod browser;
mod database;
mod extensions;
mod features;
mod permissions;
mod settings;
mod ui;
mod utils;

use anyhow::Result;
use log::info;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    info!("🌐 4 Browser Starting...");

    if let Err(e) = run().await {
        eprintln!("Fatal error: {}", e);
        log::error!("Fatal error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let app_data_dir = get_app_data_dir()?;
    info!("App data directory: {:?}", app_data_dir);

    // Initialize database
    database::init(&app_data_dir).await?;

    // Initialize settings
    let settings = settings::Settings::load(&app_data_dir).await?;
    info!("Loaded settings: browser_name={}", settings.browser_name);

    // Initialize permission manager
    let permission_manager = permissions::PermissionManager::new(&app_data_dir).await?;

    // Initialize extension manager
    let extension_manager = extensions::ExtensionManager::new(&app_data_dir).await?;

    // Launch browser
    let browser = app::BrowserApp::new(
        app_data_dir,
        settings,
        permission_manager,
        extension_manager,
    )
    .await?;

    browser.run().await?;

    Ok(())
}

pub fn get_app_data_dir() -> Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("com", "4browser", "4Browser")
        .ok_or_else(|| anyhow::anyhow!("Failed to determine project directories"))?;
    let data_dir = dirs.data_dir().to_path_buf();

    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir)?;
    }

    Ok(data_dir)
}
