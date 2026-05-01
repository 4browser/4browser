use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use serde::{Deserialize, Serialize};
use log::info;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigateRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct TabInfo {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct BookmarkInfo {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub history_count: usize,
    pub bookmarks_count: usize,
    pub extensions_count: usize,
    pub total_tabs: usize,
    pub total_windows: usize,
}

pub struct AppState {
    pub app: Arc<RwLock<crate::app::BrowserApp>>,
}

// API Routes

pub async fn get_index() -> HttpResponse {
    match tokio::fs::read_to_string("public/index.html").await {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(content),
        Err(_) => HttpResponse::NotFound().body("index.html not found"),
    }
}

pub async fn get_css(filename: web::Path<String>) -> HttpResponse {
    let path = format!("public/css/{}", filename.into_inner());
    match tokio::fs::read_to_string(&path).await {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/css; charset=utf-8")
            .body(content),
        Err(_) => HttpResponse::NotFound().body("CSS file not found"),
    }
}

pub async fn get_js(filename: web::Path<String>) -> HttpResponse {
    let path = format!("public/js/{}", filename.into_inner());
    match tokio::fs::read_to_string(&path).await {
        Ok(content) => HttpResponse::Ok()
            .content_type("application/javascript; charset=utf-8")
            .body(content),
        Err(_) => HttpResponse::NotFound().body("JS file not found"),
    }
}

pub async fn navigate(
    _state: web::Data<AppState>,
    req: web::Json<NavigateRequest>,
) -> HttpResponse {
    info!("Navigate request: {}", req.url);
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "url": &req.url
    }))
}

pub async fn history_back(_state: web::Data<AppState>) -> HttpResponse {
    info!("Go back request");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "action": "back"
    }))
}

pub async fn history_forward(_state: web::Data<AppState>) -> HttpResponse {
    info!("Go forward request");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "action": "forward"
    }))
}

pub async fn reload(_state: web::Data<AppState>) -> HttpResponse {
    info!("Reload request");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "action": "reload"
    }))
}

pub async fn get_stats(state: web::Data<AppState>) -> HttpResponse {
    let app = state.app.read().await;
    let stats = app.get_stats().await;

    let response = StatsResponse {
        history_count: stats.history_count,
        bookmarks_count: stats.bookmarks_count,
        extensions_count: stats.extensions_count,
        total_tabs: stats.total_tabs,
        total_windows: stats.total_windows,
    };

    HttpResponse::Ok().json(response)
}

pub async fn get_tabs(_state: web::Data<AppState>) -> HttpResponse {
    let tabs = vec![
        TabInfo {
            id: "tab-1".to_string(),
            name: "Home".to_string(),
            url: "about:home".to_string(),
        },
    ];

    HttpResponse::Ok().json(tabs)
}

pub async fn new_tab(_state: web::Data<AppState>) -> HttpResponse {
    info!("New tab request");
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "tab_id": "tab-new"
    }))
}

pub async fn select_tab(
    _state: web::Data<AppState>,
    tab_id: web::Path<String>,
) -> HttpResponse {
    info!("Select tab: {}", tab_id.into_inner());
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

pub async fn get_extensions(state: web::Data<AppState>) -> HttpResponse {
    let app = state.app.read().await;
    let extensions = app.extension_manager.get_extensions().await;

    let ext_list: Vec<ExtensionInfo> = extensions
        .iter()
        .map(|e| ExtensionInfo {
            id: e.id.clone(),
            name: e.manifest.name.clone(),
            description: e.manifest.description.clone().unwrap_or_default(),
            version: e.manifest.version.clone(),
        })
        .collect();

    HttpResponse::Ok().json(ext_list)
}

pub async fn get_bookmarks(state: web::Data<AppState>) -> HttpResponse {
    let app = state.app.read().await;
    let bookmarks = app.browser_engine.get_bookmarks(None).await;

    let bookmark_list: Vec<BookmarkInfo> = bookmarks
        .iter()
        .map(|b| BookmarkInfo {
            name: b.title.clone(),
            url: b.url.clone(),
        })
        .collect();

    HttpResponse::Ok().json(bookmark_list)
}

pub async fn update_browser_name(
    state: web::Data<AppState>,
    req: web::Json<SettingRequest>,
) -> HttpResponse {
    let app = state.app.read().await;
    if let Err(e) = app.update_browser_name(req.name.clone()).await {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }));
    }

    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

pub async fn update_device_name(
    state: web::Data<AppState>,
    req: web::Json<SettingRequest>,
) -> HttpResponse {
    let app = state.app.read().await;
    if let Err(e) = app.update_device_name(req.name.clone()).await {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }));
    }

    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

pub async fn start_server(app: Arc<RwLock<crate::app::BrowserApp>>) -> anyhow::Result<()> {
    let app_state = web::Data::new(AppState {
        app: app.clone(),
    });

    info!("🚀 Starting web server on http://localhost:8080");

    let _server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            // Static files
            .route("/", web::get().to(get_index))
            .route("/css/{filename}", web::get().to(get_css))
            .route("/js/{filename}", web::get().to(get_js))
            // API routes
            .service(
                web::scope("/api")
                    // Navigation
                    .route("/navigate", web::post().to(navigate))
                    .route("/history/back", web::post().to(history_back))
                    .route("/history/forward", web::post().to(history_forward))
                    .route("/reload", web::post().to(reload))
                    // Stats
                    .route("/stats", web::get().to(get_stats))
                    // Tabs
                    .route("/tabs", web::get().to(get_tabs))
                    .route("/tabs/new", web::post().to(new_tab))
                    .route("/tabs/{tab_id}", web::post().to(select_tab))
                    // Extensions
                    .route("/extensions", web::get().to(get_extensions))
                    // Bookmarks
                    .route("/bookmarks", web::get().to(get_bookmarks))
                    // Settings
                    .route("/settings/browser-name", web::post().to(update_browser_name))
                    .route("/settings/device-name", web::post().to(update_device_name))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
