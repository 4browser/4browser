use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: String,
    pub title: String,
    pub url: String,
    pub favicon_url: Option<String>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub id: String,
    pub tabs: Vec<Tab>,
    pub active_tab_idx: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserHistory {
    pub id: String,
    pub url: String,
    pub title: String,
    pub visited_at: chrono::DateTime<chrono::Utc>,
    pub time_spent_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub title: String,
    pub url: String,
    pub folder: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct BrowserEngine {
    windows: Arc<RwLock<HashMap<String, Window>>>,
    history: Arc<RwLock<Vec<BrowserHistory>>>,
    bookmarks: Arc<RwLock<Vec<Bookmark>>>,
    app_data_dir: PathBuf,
}

impl BrowserEngine {
    pub async fn new(app_data_dir: &PathBuf) -> Result<Self> {
        info!("Initializing BrowserEngine");

        let engine = Self {
            windows: Arc::new(RwLock::new(HashMap::new())),
            history: Arc::new(RwLock::new(Vec::new())),
            bookmarks: Arc::new(RwLock::new(Vec::new())),
            app_data_dir: app_data_dir.clone(),
        };

        engine.load_history().await?;
        engine.load_bookmarks().await?;

        Ok(engine)
    }

    pub async fn create_window(&self) -> Result<String> {
        let window_id = Uuid::new_v4().to_string();

        let mut windows = self.windows.write().await;
        windows.insert(
            window_id.clone(),
            Window {
                id: window_id.clone(),
                tabs: Vec::new(),
                active_tab_idx: 0,
                created_at: chrono::Utc::now(),
            },
        );

        info!("Created window: {}", window_id);

        Ok(window_id)
    }

    pub async fn create_tab(&self, window_id: &str, url: &str) -> Result<Tab> {
        let tab_id = Uuid::new_v4().to_string();
        let tab = Tab {
            id: tab_id,
            title: "New Tab".to_string(),
            url: url.to_string(),
            favicon_url: None,
            is_active: true,
            created_at: chrono::Utc::now(),
        };

        if let Some(window) = self.windows.write().await.get_mut(window_id) {
            window.tabs.push(tab.clone());
        }

        info!("Created tab: {} for window: {}", tab.id, window_id);

        Ok(tab)
    }

    pub async fn close_tab(&self, window_id: &str, tab_id: &str) -> Result<()> {
        if let Some(window) = self.windows.write().await.get_mut(window_id) {
            window.tabs.retain(|t| t.id != tab_id);
        }

        info!("Closed tab: {}", tab_id);

        Ok(())
    }

    pub async fn update_tab_title(&self, window_id: &str, tab_id: &str, title: &str) -> Result<()> {
        if let Some(window) = self.windows.write().await.get_mut(window_id) {
            if let Some(tab) = window.tabs.iter_mut().find(|t| t.id == tab_id) {
                tab.title = title.to_string();
            }
        }

        Ok(())
    }

    pub async fn navigate(&self, window_id: &str, tab_id: &str, url: &str) -> Result<()> {
        if let Some(window) = self.windows.write().await.get_mut(window_id) {
            if let Some(tab) = window.tabs.iter_mut().find(|t| t.id == tab_id) {
                tab.url = url.to_string();
            }
        }

        info!("Navigated tab {} to: {}", tab_id, url);

        Ok(())
    }

    pub async fn add_to_history(&self, url: &str, title: &str, time_spent_ms: u64) -> Result<()> {
        let entry = BrowserHistory {
            id: Uuid::new_v4().to_string(),
            url: url.to_string(),
            title: title.to_string(),
            visited_at: chrono::Utc::now(),
            time_spent_ms,
        };

        self.history.write().await.push(entry);

        Ok(())
    }

    pub async fn get_history(&self, limit: usize) -> Vec<BrowserHistory> {
        let history = self.history.read().await;
        history
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    pub async fn search_history(&self, query: &str) -> Vec<BrowserHistory> {
        let history = self.history.read().await;
        history
            .iter()
            .filter(|h| {
                h.url.contains(query)
                    || h.title.to_lowercase().contains(&query.to_lowercase())
            })
            .cloned()
            .collect()
    }

    pub async fn clear_history(&self) -> Result<()> {
        self.history.write().await.clear();
        self.save_history().await?;

        info!("History cleared");

        Ok(())
    }

    pub async fn load_history(&self) -> Result<()> {
        let history_file = self.app_data_dir.join("history.json");

        if history_file.exists() {
            let content = tokio::fs::read_to_string(&history_file).await?;
            let history: Vec<BrowserHistory> = serde_json::from_str(&content)?;
            *self.history.write().await = history;
            info!("Loaded {} history entries", self.history.read().await.len());
        }

        Ok(())
    }

    pub async fn save_history(&self) -> Result<()> {
        let history_file = self.app_data_dir.join("history.json");
        let history = self.history.read().await;
        let content = serde_json::to_string_pretty(&*history)?;
        tokio::fs::write(&history_file, content).await?;

        info!("History saved");

        Ok(())
    }

    pub async fn add_bookmark(&self, url: &str, title: &str, folder: &str) -> Result<Bookmark> {
        let bookmark = Bookmark {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            url: url.to_string(),
            folder: folder.to_string(),
            created_at: chrono::Utc::now(),
        };

        self.bookmarks.write().await.push(bookmark.clone());

        info!("Added bookmark: {} ({})", title, folder);

        Ok(bookmark)
    }

    pub async fn get_bookmarks(&self, folder: Option<&str>) -> Vec<Bookmark> {
        let bookmarks = self.bookmarks.read().await;

        if let Some(f) = folder {
            bookmarks.iter().filter(|b| b.folder == f).cloned().collect()
        } else {
            bookmarks.iter().cloned().collect()
        }
    }

    pub async fn remove_bookmark(&self, bookmark_id: &str) -> Result<()> {
        self.bookmarks
            .write()
            .await
            .retain(|b| b.id != bookmark_id);

        info!("Removed bookmark: {}", bookmark_id);

        Ok(())
    }

    pub async fn load_bookmarks(&self) -> Result<()> {
        let bookmarks_file = self.app_data_dir.join("bookmarks.json");

        if bookmarks_file.exists() {
            let content = tokio::fs::read_to_string(&bookmarks_file).await?;
            let bookmarks: Vec<Bookmark> = serde_json::from_str(&content)?;
            *self.bookmarks.write().await = bookmarks;
            info!(
                "Loaded {} bookmarks",
                self.bookmarks.read().await.len()
            );
        }

        Ok(())
    }

    pub async fn save_bookmarks(&self) -> Result<()> {
        let bookmarks_file = self.app_data_dir.join("bookmarks.json");
        let bookmarks = self.bookmarks.read().await;
        let content = serde_json::to_string_pretty(&*bookmarks)?;
        tokio::fs::write(&bookmarks_file, content).await?;

        info!("Bookmarks saved");

        Ok(())
    }
}
