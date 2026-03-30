use anyhow::Result;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionStatus {
    Granted,
    Denied,
    NotResponded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionState {
    pub camera: PermissionStatus,
    pub microphone: PermissionStatus,
    pub notifications: PermissionStatus,
    pub geolocation: PermissionStatus,
    pub clipboard_read: PermissionStatus,
    pub clipboard_write: PermissionStatus,
    pub storage_access: PermissionStatus,
    pub payment: PermissionStatus,
}

impl Default for PermissionState {
    fn default() -> Self {
        Self {
            camera: PermissionStatus::NotResponded,
            microphone: PermissionStatus::NotResponded,
            notifications: PermissionStatus::NotResponded,
            geolocation: PermissionStatus::NotResponded,
            clipboard_read: PermissionStatus::NotResponded,
            clipboard_write: PermissionStatus::NotResponded,
            storage_access: PermissionStatus::NotResponded,
            payment: PermissionStatus::NotResponded,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PermissionRequest {
    pub domain: String,
    pub permission_type: String,
    pub requested_at: chrono::DateTime<chrono::Utc>,
}

pub struct PermissionManager {
    permissions: Arc<RwLock<HashMap<String, PermissionState>>>,
    pending_requests: Arc<RwLock<Vec<PermissionRequest>>>,
    app_data_dir: PathBuf,
}

impl PermissionManager {
    pub async fn new(app_data_dir: &PathBuf) -> Result<Self> {
        info!("Initializing PermissionManager");

        let manager = Self {
            permissions: Arc::new(RwLock::new(HashMap::new())),
            pending_requests: Arc::new(RwLock::new(Vec::new())),
            app_data_dir: app_data_dir.clone(),
        };

        manager.load_permissions().await?;

        Ok(manager)
    }

    pub async fn load_permissions(&self) -> Result<()> {
        let perms_file = self.app_data_dir.join("permissions.json");

        if perms_file.exists() {
            let content = tokio::fs::read_to_string(&perms_file).await?;
            let perms: HashMap<String, PermissionState> = serde_json::from_str(&content)?;
            *self.permissions.write().await = perms;
            info!("Loaded permissions for {} domains", self.permissions.read().await.len());
        }

        Ok(())
    }

    pub async fn save_permissions(&self) -> Result<()> {
        let perms_file = self.app_data_dir.join("permissions.json");
        let perms = self.permissions.read().await;
        let content = serde_json::to_string_pretty(&*perms)?;
        tokio::fs::write(&perms_file, content).await?;

        Ok(())
    }

    pub async fn request_permission(
        &self,
        domain: &str,
        permission_type: &str,
    ) -> Result<PermissionStatus> {
        info!("Permission request from {}: {}", domain, permission_type);

        let mut perms = self.permissions.write().await;
        let perm_state = perms
            .entry(domain.to_string())
            .or_insert_with(PermissionState::default);

        let status = match permission_type {
            "camera" => &perm_state.camera,
            "microphone" => &perm_state.microphone,
            "notifications" => &perm_state.notifications,
            "geolocation" => &perm_state.geolocation,
            "clipboard_read" => &perm_state.clipboard_read,
            "clipboard_write" => &perm_state.clipboard_write,
            "storage_access" => &perm_state.storage_access,
            "payment" => &perm_state.payment,
            _ => {
                warn!("Unknown permission type: {}", permission_type);
                return Ok(PermissionStatus::Denied);
            }
        };

        Ok(status.clone())
    }

    pub async fn set_permission(
        &self,
        domain: &str,
        permission_type: &str,
        status: PermissionStatus,
    ) -> Result<()> {
        info!(
            "Setting {} for {}: {:?}",
            permission_type, domain, status
        );

        let mut perms = self.permissions.write().await;
        let perm_state = perms
            .entry(domain.to_string())
            .or_insert_with(PermissionState::default);

        match permission_type {
            "camera" => perm_state.camera = status,
            "microphone" => perm_state.microphone = status,
            "notifications" => perm_state.notifications = status,
            "geolocation" => perm_state.geolocation = status,
            "clipboard_read" => perm_state.clipboard_read = status,
            "clipboard_write" => perm_state.clipboard_write = status,
            "storage_access" => perm_state.storage_access = status,
            "payment" => perm_state.payment = status,
            _ => return Err(anyhow::anyhow!("Unknown permission type: {}", permission_type)),
        };

        self.save_permissions().await?;

        Ok(())
    }

    pub async fn get_permissions(&self, domain: &str) -> Option<PermissionState> {
        self.permissions
            .read()
            .await
            .get(domain)
            .map(|p| p.clone())
    }

    pub async fn clear_permissions(&self, domain: &str) -> Result<()> {
        self.permissions.write().await.remove(domain);
        self.save_permissions().await?;
        info!("Cleared permissions for domain: {}", domain);

        Ok(())
    }

    pub async fn clear_all_permissions(&self) -> Result<()> {
        self.permissions.write().await.clear();
        self.save_permissions().await?;
        info!("Cleared all permissions");

        Ok(())
    }

    pub async fn add_pending_request(
        &self,
        domain: String,
        permission_type: String,
    ) -> Result<()> {
        let request = PermissionRequest {
            domain,
            permission_type,
            requested_at: chrono::Utc::now(),
        };

        self.pending_requests.write().await.push(request);

        Ok(())
    }

    pub async fn get_pending_requests(&self) -> Result<Vec<PermissionRequest>> {
        Ok(self.pending_requests.read().await.clone())
    }
}
