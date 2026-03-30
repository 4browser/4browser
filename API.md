# 📚 API Documentation

## Browser Core API

### BrowserEngine

The core browser functionality for managing windows, tabs, navigation, history, and bookmarks.

#### Creating a Window
```rust
let window_id = browser_engine.create_window().await?;
// Returns: "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
```

#### Managing Tabs
```rust
// Create a new tab
let tab = browser_engine.create_tab(&window_id, "https://google.com").await?;

// Navigate to a URL
browser_engine.navigate(&window_id, &tab.id, "https://github.com").await?;

// Update tab title
browser_engine.update_tab_title(&window_id, &tab.id, "GitHub").await?;

// Close tab
browser_engine.close_tab(&window_id, &tab.id).await?;
```

#### History Operations
```rust
// Add to history
browser_engine.add_to_history("https://rust-lang.org", "The Rust Programming Language", 5000).await?;

// Get recent history (limit: 50 entries)
let history = browser_engine.get_history(50).await;

// Search history
let results = browser_engine.search_history("rust").await;

// Clear all history
browser_engine.clear_history().await?;
```

#### Bookmark Management
```rust
// Add bookmark
let bookmark = browser_engine
    .add_bookmark("https://github.com", "GitHub", "/development")
    .await?;

// Get bookmarks from folder
let bookmarks = browser_engine.get_bookmarks(Some("/development")).await;

// Get all bookmarks
let all = browser_engine.get_bookmarks(None).await;

// Remove bookmark
browser_engine.remove_bookmark(&bookmark.id).await?;
```

### Permission Manager API

#### Request Permission
```rust
// Check if website can access camera
let status = permission_manager
    .request_permission("example.com", "camera")
    .await?;

// Returns: PermissionStatus::Granted | Denied | NotResponded
```

#### Set Permission
```rust
use crate::permissions::PermissionStatus;

// Grant microphone access to domain
permission_manager
    .set_permission("example.com", "microphone", PermissionStatus::Granted)
    .await?;

// Deny clipboard access
permission_manager
    .set_permission("example.com", "clipboard_write", PermissionStatus::Denied)
    .await?;
```

#### Retrieve Permissions
```rust
// Get all permissions for a domain
let perms = permission_manager.get_permissions("example.com").await;

// Access individual permissions
if let Some(perm_state) = perms {
    println!("Camera: {:?}", perm_state.camera);
    println!("Microphone: {:?}", perm_state.microphone);
    println!("Notifications: {:?}", perm_state.notifications);
}
```

#### Permission Cleanup
```rust
// Clear permissions for one domain
permission_manager.clear_permissions("example.com").await?;

// Clear all permissions globally
permission_manager.clear_all_permissions().await?;
```

### Extension Manager API

#### Install Extension
```rust
// Install from directory
let extension_id = extension_manager
    .install_extension(Path::new("/path/to/extension"))
    .await?;

// Install from ZIP file
let extension_id = extension_manager
    .install_extension(Path::new("/path/to/extension.zip"))
    .await?;
```

#### Manage Extensions
```rust
// Get all extensions
let extensions = extension_manager.get_extensions().await;

// Get only enabled extensions
let enabled = extension_manager.get_enabled_extensions().await;

// Get specific extension
if let Some(ext) = extension_manager.get_extension(&extension_id).await {
    println!("Extension: {}", ext.manifest.name);
    println!("Version: {}", ext.manifest.version);
}
```

#### Enable/Disable Extensions
```rust
// Enable extension
extension_manager.enable_extension(&extension_id).await?;

// Disable extension
extension_manager.disable_extension(&extension_id).await?;

// Uninstall extension
extension_manager.uninstall_extension(&extension_id).await?;
```

### Settings API

#### Updating Appearance
```rust
let mut settings = Settings::load(&app_data_dir).await?;

// Update theme
settings.update_theme("dark".to_string());

// Update accent color
settings.update_accent_color("#FF6B35".to_string());

// Update font size
settings.update_font_size(14);

// Save changes
settings.save(&app_data_dir).await?;
```

#### Custom Browser Identity
```rust
// Change browser name
settings.update_browser_name("Quantum Browser".to_string());

// Change device name for websites
settings.update_device_name("MacBook Pro".to_string());

// The browser will now identify as:
// Mozilla/5.0 ... Quantum Browser/1.0 ...
```

#### Developer Mode
```rust
settings.toggle_developer_mode();

// Access dev tools and advanced features
if settings.developer_mode {
    println!("Developer mode enabled");
}
```

#### Get User Agent
```rust
let ua = settings.get_user_agent();
// "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 ... 4 Browser/1.0 ..."
```

### Feature API

#### Keyboard Shortcuts
```rust
use crate::features::FeatureManager;

FeatureManager::show_keyboard_shortcuts_help()?;
```

#### Smart Tab Grouping
```rust
FeatureManager::enable_tab_grouping()?;
// Tabs are now grouped by domain
```

#### Privacy Dashboard
```rust
FeatureManager::show_privacy_dashboard()?;
// Opens real-time privacy metrics
```

#### Session Snapshots
```rust
// Save current session
FeatureManager::save_session_snapshot("Work Session")?;

// Restore later (implementation in UI layer)
// browser.load_session_snapshot("Work Session").await?;
```

#### Split View Browsing
```rust
FeatureManager::enable_split_view()?;
// Opens split screen mode
```

#### Smart Notes
```rust
FeatureManager::add_smart_note("https://article.com", "This is important!")?;
```

#### Domain Lookalike Detection
```rust
let is_similar = FeatureManager::check_domain_lookalike(
    "github.com",
    "gitthub.com"
);

if is_similar {
    println!("⚠️ Warning: Similar domain detected!");
}
```

## Database API

### Direct Query Execution
```rust
use crate::database;

// Execute raw SQL query
let results = database::execute_query(
    "SELECT domain FROM permissions WHERE camera = 1",
    &[]
)?;

for row in results {
    println!("Domain with camera: {}", row[0]);
}
```

### Permission Operations
```rust
use crate::database;

// Add permission record for domain
database::add_permission("example.com")?;

// Update specific permission
database::update_permission("example.com", "camera", true)?;

// Check if permission is granted
let allowed = database::get_permission("example.com", "microphone")?;
```

## Utility Functions

### URL Utilities
```rust
use crate::utils;

// Validate URL
if utils::is_valid_url("https://google.com") {
    println!("Valid URL");
}

// Extract domain from URL
if let Some(domain) = utils::extract_domain("https://www.google.com/search") {
    println!("Domain: {}", domain); // "www.google.com"
}

// Check if special URL (about:, chrome://, etc.)
if utils::is_special_url("about:settings") {
    println!("Special URL - don't navigate!");
}
```

### File Utilities
```rust
// Sanitize filename for safe file storage
let safe_name = utils::sanitize_filename("my<file|name>.txt");
// "my_file_name_.txt"

// Format bytes for display
let size_str = utils::format_bytes(1048576);
// "1.00 MB"

// Format duration in milliseconds
let duration_str = utils::format_duration_ms(65000);
// "1 minutes"
```

## Event System (Future)

Planned event system for internal communication:

```rust
// Listen for permission changes
browser.on_permission_changed(|domain, permission, status| {
    println!("{}: {} = {:?}", domain, permission, status);
});

// Listen for navigation events
browser.on_navigate(|tab_id, url| {
    println!("Navigating to: {}", url);
});

// Listen for extension events
browser.on_extension_installed(|ext_id, manifest| {
    println!("Extension installed: {}", manifest.name);
});
```

## Configuration

### Privacy Settings
```rust
let privacy = settings.privacy;

if privacy.block_trackers {
    println!("Tracker blocking is ON");
}

if privacy.do_not_track {
    println!("DNT header will be sent");
}
```

### Search Engine Configuration
```rust
use crate::features::SearchEngineConfig;

let google = SearchEngineConfig {
    name: "Google".to_string(),
    url: "https://www.google.com/search?q=".to_string(),
    icon_url: Some("https://www.google.com/favicon.ico".to_string()),
    suggestions_enabled: true,
};

// Use custom search engine
settings.browser.default_search_engine = google.url;
```

## Error Handling

All API functions return `Result<T>`:

```rust
use anyhow::Result;

async fn example() -> Result<()> {
    let perms = permission_manager.get_permissions("example.com").await;
    
    match perms {
        Some(p) => {
            // Process permissions
        }
        None => {
            // No permissions set for this domain
        }
    }
    
    Ok(())
}
```

## Concurrency Safety

All APIs are thread-safe:

```rust
// Arc<RwLock<T>> allows concurrent access
let app = Arc::new(app);

// Clone for multiple threads
let app_clone1 = app.clone();
let app_clone2 = app.clone();

tokio::spawn(async move {
    // Use app_clone1
});

tokio::spawn(async move {
    // Use app_clone2
});
```

## Complete Example

```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize app
    let app_data_dir = get_app_data_dir()?;
    let settings = Settings::load(&app_data_dir).await?;
    let permission_manager = PermissionManager::new(&app_data_dir).await?;
    let extension_manager = ExtensionManager::new(&app_data_dir).await?;
    
    let app = BrowserApp::new(
        app_data_dir,
        settings,
        permission_manager,
        extension_manager,
    ).await?;

    // Create window and tab
    let window_id = app.browser_engine.create_window().await?;
    let tab = app.browser_engine.create_tab(&window_id, "https://google.com").await?;

    // Request permission
    let status = app.permission_manager
        .request_permission("google.com", "camera")
        .await?;

    // Install extension
    let ext_id = app.extension_manager
        .install_extension(Path::new("./my-extension"))
        .await?;

    // Start browser
    app.run().await?;

    Ok(())
}
```
