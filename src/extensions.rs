use anyhow::Result;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub manifest_version: u32,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub icons: Option<HashMap<String, String>>,
    pub background: Option<ExtensionBackground>,
    pub content_scripts: Option<Vec<ContentScript>>,
    pub action: Option<ExtensionAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionBackground {
    pub service_worker: Option<String>,
    pub scripts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentScript {
    pub matches: Vec<String>,
    pub js: Vec<String>,
    pub run_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionAction {
    pub default_title: Option<String>,
    pub default_popup: Option<String>,
    pub default_icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub id: String,
    pub manifest: ExtensionManifest,
    pub enabled: bool,
    pub install_path: PathBuf,
    pub installed_at: chrono::DateTime<chrono::Utc>,
}

pub struct ExtensionManager {
    extensions: Arc<RwLock<HashMap<String, Extension>>>,
    app_data_dir: PathBuf,
    extensions_dir: PathBuf,
}

impl ExtensionManager {
    pub async fn new(app_data_dir: &PathBuf) -> Result<Self> {
        info!("Initializing ExtensionManager");

        let extensions_dir = app_data_dir.join("extensions");
        if !extensions_dir.exists() {
            tokio::fs::create_dir_all(&extensions_dir).await?;
        }

        let manager = Self {
            extensions: Arc::new(RwLock::new(HashMap::new())),
            app_data_dir: app_data_dir.clone(),
            extensions_dir,
        };

        manager.load_extensions().await?;

        Ok(manager)
    }

    pub async fn load_extensions(&self) -> Result<()> {
        let mut extensions = self.extensions.write().await;

        if !self.extensions_dir.exists() {
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(&self.extensions_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(ext) = self.load_extension_from_dir(&path).await {
                    info!("Loaded extension: {} ({})", ext.manifest.name, ext.id);
                    extensions.insert(ext.id.clone(), ext);
                }
            }
        }

        info!("Loaded {} extensions", extensions.len());

        Ok(())
    }

    async fn load_extension_from_dir(&self, dir: &Path) -> Result<Extension> {
        let manifest_path = dir.join("manifest.json");

        if !manifest_path.exists() {
            return Err(anyhow::anyhow!(
                "No manifest.json found in extension directory"
            ));
        }

        let content = tokio::fs::read_to_string(&manifest_path).await?;
        let manifest: ExtensionManifest = serde_json::from_str(&content)?;

        let ext = Extension {
            id: Uuid::new_v4().to_string(),
            manifest,
            enabled: true,
            install_path: dir.to_path_buf(),
            installed_at: chrono::Utc::now(),
        };

        Ok(ext)
    }

    pub async fn install_extension(&self, file_path: &Path) -> Result<String> {
        info!("Installing extension from: {:?}", file_path);

        // Extract and validate extension
        let extension_id = Uuid::new_v4().to_string();
        let install_dir = self.extensions_dir.join(&extension_id);

        // Handle zip extraction if it's a zipped extension
        if file_path.extension().map_or(false, |ext| ext == "zip") {
            self.extract_zip_extension(file_path, &install_dir).await?;
        } else {
            // Copy directory as is
            copy_dir_recursive(file_path, &install_dir).await?;
        }

        // Load and validate the extension
        let extension = self.load_extension_from_dir(&install_dir).await?;

        let mut extensions = self.extensions.write().await;
        extensions.insert(extension_id.clone(), extension);

        info!("Extension installed successfully: {}", extension_id);

        Ok(extension_id)
    }

    async fn extract_zip_extension(&self, _zip_file: &Path, dest: &Path) -> Result<()> {
        // This would require a zip extraction library
        // For now, we'll just return an error unless implemented
        tokio::fs::create_dir_all(dest).await?;

        warn!("Zip extraction not yet implemented, treating as directory");

        Ok(())
    }

    pub async fn uninstall_extension(&self, extension_id: &str) -> Result<()> {
        let mut extensions = self.extensions.write().await;

        if let Some(ext) = extensions.remove(extension_id) {
            if ext.install_path.exists() {
                tokio::fs::remove_dir_all(&ext.install_path).await?;
            }
            info!("Extension uninstalled: {}", extension_id);
        }

        Ok(())
    }

    pub async fn enable_extension(&self, extension_id: &str) -> Result<()> {
        let mut extensions = self.extensions.write().await;

        if let Some(ext) = extensions.get_mut(extension_id) {
            ext.enabled = true;
            info!("Extension enabled: {}", extension_id);
        }

        Ok(())
    }

    pub async fn disable_extension(&self, extension_id: &str) -> Result<()> {
        let mut extensions = self.extensions.write().await;

        if let Some(ext) = extensions.get_mut(extension_id) {
            ext.enabled = false;
            info!("Extension disabled: {}", extension_id);
        }

        Ok(())
    }

    pub async fn get_extensions(&self) -> Vec<Extension> {
        let extensions = self.extensions.read().await;
        extensions.values().map(|e| e.clone()).collect()
    }

    pub async fn get_enabled_extensions(&self) -> Vec<Extension> {
        let extensions = self.extensions.read().await;
        extensions
            .values()
            .filter(|e| e.enabled)
            .map(|e| e.clone())
            .collect()
    }

    pub async fn get_extension(&self, extension_id: &str) -> Option<Extension> {
        self.extensions
            .read()
            .await
            .get(extension_id)
            .map(|e| e.clone())
    }
}

async fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    tokio::fs::create_dir_all(dst).await?;

    let mut entries = tokio::fs::read_dir(src).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let file_name = entry.file_name();
        let dest_path = dst.join(&file_name);

        if path.is_dir() {
            Box::pin(copy_dir_recursive(&path, &dest_path)).await?;
        } else {
            tokio::fs::copy(&path, &dest_path).await?;
        }
    }

    Ok(())
}
