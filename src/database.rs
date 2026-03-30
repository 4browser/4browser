use anyhow::Result;
use log::info;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

pub static DB: once_cell::sync::Lazy<Mutex<Option<Connection>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(None));

pub async fn init(app_data_dir: &Path) -> Result<()> {
    let db_path = app_data_dir.join("4browser.db");
    info!("Initializing database at: {:?}", db_path);

    let conn = Connection::open(&db_path)?;
    
    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Create tables
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS permissions (
            id TEXT PRIMARY KEY,
            domain TEXT NOT NULL UNIQUE,
            camera INTEGER DEFAULT 0,
            microphone INTEGER DEFAULT 0,
            notifications INTEGER DEFAULT 0,
            geolocation INTEGER DEFAULT 0,
            clipboard_read INTEGER DEFAULT 0,
            clipboard_write INTEGER DEFAULT 0,
            storage_access INTEGER DEFAULT 0,
            payment INTEGER DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS permission_requests (
            id TEXT PRIMARY KEY,
            domain TEXT NOT NULL,
            permission_type TEXT NOT NULL,
            status TEXT NOT NULL,
            requested_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            expires_at TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS extensions (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            version TEXT NOT NULL,
            enabled INTEGER DEFAULT 1,
            permissions TEXT,
            manifest TEXT NOT NULL,
            install_path TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS user_agents (
            id TEXT PRIMARY KEY,
            domain TEXT NOT NULL UNIQUE,
            custom_user_agent TEXT NOT NULL,
            custom_device_name TEXT NOT NULL DEFAULT 'Custom Device',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS history (
            id TEXT PRIMARY KEY,
            url TEXT NOT NULL,
            title TEXT,
            visited_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            time_spent_ms INTEGER DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS bookmarks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            folder_path TEXT NOT NULL DEFAULT '/',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            is_folder INTEGER DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS browser_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS sync_data (
            id TEXT PRIMARY KEY,
            data_type TEXT NOT NULL,
            content TEXT NOT NULL,
            synced_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            device_id TEXT NOT NULL
        );
        "#,
    )?;

    *DB.lock().unwrap() = Some(conn);
    info!("Database initialized successfully");

    Ok(())
}

pub fn get_connection() -> Result<std::sync::MutexGuard<'static, Option<Connection>>> {
    Ok(DB.lock().unwrap())
}

pub fn execute_query(sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<Vec<Vec<String>>> {
    let db = DB.lock().unwrap();
    let conn = db.as_ref().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;

    let mut stmt = conn.prepare(sql)?;
    let results = stmt
        .query_map(params, |row| {
            let mut row_data = Vec::new();
            let col_count = row.as_ref().column_count();
            for i in 0..col_count {
                let val: String = row.get(i).unwrap_or_default();
                row_data.push(val);
            }
            Ok(row_data)
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(results)
}

pub fn add_permission(domain: &str) -> Result<()> {
    let db = DB.lock().unwrap();
    let conn = db.as_ref().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;

    let id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT OR IGNORE INTO permissions (id, domain) VALUES (?1, ?2)",
        params![&id, domain],
    )?;

    Ok(())
}

pub fn update_permission(domain: &str, permission: &str, allowed: bool) -> Result<()> {
    let db = DB.lock().unwrap();
    let conn = db.as_ref().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;

    let allowed_int = if allowed { 1 } else { 0 };
    let sql = format!("UPDATE permissions SET {} = ?1, updated_at = CURRENT_TIMESTAMP WHERE domain = ?2", permission);

    conn.execute(&sql, params![allowed_int, domain])?;

    Ok(())
}

pub fn get_permission(domain: &str, permission: &str) -> Result<bool> {
    let db = DB.lock().unwrap();
    let conn = db.as_ref().ok_or_else(|| anyhow::anyhow!("Database not initialized"))?;

    let sql = format!("SELECT {} FROM permissions WHERE domain = ?1", permission);
    let mut stmt = conn.prepare(&sql)?;

    let allowed: i32 = stmt
        .query_row(params![domain], |row| row.get(0))
        .unwrap_or(0);

    Ok(allowed != 0)
}
