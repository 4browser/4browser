# 🎯 4 Browser - Quick Reference Card

## Installation & Setup

```bash
# Clone repository
git clone https://github.com/4browser/4browser.git
cd 4browser

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build project
cargo build --release

# Run browser
./target/release/4browser

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Check code
cargo check
```

## Project Structure Quick Map

```
src/main.rs              → Entry point
src/app.rs              → Main orchestrator
src/browser.rs          → Tab/window/history management
src/database.rs         → SQLite persistence
src/permissions.rs      → Permission management
src/extensions.rs       → Extension system
src/settings.rs         → User settings
src/features.rs         → Unique features
src/ui.rs              → UI foundation
src/utils.rs           → Utility functions
```

## Core API Quick Reference

### Browser Engine
```rust
browser_engine.create_window().await?
browser_engine.create_tab(&window_id, url).await?
browser_engine.navigate(&window_id, &tab_id, url).await?
browser_engine.add_to_history(url, title, ms).await?
browser_engine.add_bookmark(url, title, folder).await?
```

### Permissions
```rust
permission_manager.request_permission(domain, "camera").await?
permission_manager.set_permission(domain, "camera", GRANTED).await?
permission_manager.get_permissions(domain).await
permission_manager.clear_permissions(domain).await?
```

### Extensions
```rust
extension_manager.install_extension(path).await?
extension_manager.enable_extension(id).await?
extension_manager.disable_extension(id).await?
extension_manager.uninstall_extension(id).await?
extension_manager.get_extensions().await
```

### Settings
```rust
let mut settings = Settings::load(app_data_dir).await?;
settings.update_browser_name("Firefox");
settings.update_device_name("Windows PC");
settings.update_theme("dark");
settings.save(app_data_dir).await?;
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+T | New Tab |
| Ctrl+W | Close Tab |
| Ctrl+N | New Window |
| Ctrl+L | Focus Address Bar |
| Ctrl+H | History |
| Ctrl+B | Bookmarks |
| Ctrl+, | Settings |
| Ctrl+Shift+I | Developer Tools |
| Ctrl+Shift+P | Private Mode |
| Alt+← | Back |
| Alt+→ | Forward |
| Ctrl+R | Reload |
| Ctrl+Shift+R | Hard Reload |

## Permission Types

1. **camera** - Access to camera device
2. **microphone** - Access to microphone
3. **notifications** - Show notifications
4. **geolocation** - Access to GPS location
5. **clipboard_read** - Read clipboard
6. **clipboard_write** - Write to clipboard
7. **storage_access** - Access local storage
8. **payment** - Payment method access

## Database Tables

```
permissions          → Site permissions
extensions          → Installed extensions
history             → Browsing history
bookmarks           → Saved bookmarks
user_agents         → Custom user agents
browser_settings    → App settings
permission_requests → Pending requests
sync_data          → Cloud sync data
```

## File Paths (Platform-Specific)

```
Linux:   ~/.local/share/4Browser/
macOS:   ~/Library/Application Support/4Browser/
Windows: %APPDATA%\4Browser\

Within directory:
4browser.db        → SQLite database
settings.json      → User settings
permissions.json   → Site permissions
history.json       → Browsing history
bookmarks.json     → Bookmarks
extensions/        → Installed extensions
```

## Creating an Extension

### 1. Create manifest.json
```json
{
  "manifest_version": 3,
  "name": "My Extension",
  "version": "1.0",
  "permissions": ["activeTab", "scripting"],
  "action": {
    "default_popup": "popup.html"
  }
}
```

### 2. Create popup.html
```html
<!DOCTYPE html>
<html>
<head><style>body { width: 400px; }</style></head>
<body>
  <h1>My Extension</h1>
  <button id="btn">Click Me</button>
  <script src="popup.js"></script>
</body>
</html>
```

### 3. Create popup.js
```javascript
document.getElementById('btn').addEventListener('click', () => {
  console.log('Button clicked!');
});
```

### 4. Load in 4 Browser
Settings → Extensions → Load Unpacked → Select folder

## Common Patterns

### Async Database
```rust
pub async fn example() -> Result<()> {
    database::add_permission("example.com")?;
    database::update_permission("example.com", "camera", true)?;
    Ok(())
}
```

### Arc<RwLock> Pattern
```rust
pub struct Manager {
    data: Arc<RwLock<HashMap<String, Data>>>,
}

// Read
let data = manager.data.read().await;

// Write
let mut data = manager.data.write().await;
data.insert(key, value);
```

### Error Handling
```rust
let result = operation().map_err(|e| {
    log::error!("Operation failed: {}", e);
    e
})?;
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run specific module tests
cargo test permissions::

# Run tests with logging
RUST_LOG=debug cargo test
```

## Debugging

```bash
# Enable full logging
RUST_LOG=trace cargo run

# Backtrace on panic
RUST_BACKTRACE=1 cargo run

# Specific module logging
RUST_LOG=4browser::extensions=debug cargo run

# View generated docs
cargo doc --open
```

## Dependencies Overview

| Name | Purpose |
|------|---------|
| tokio | Async runtime |
| rusqlite | SQLite database |
| serde | Serialization |
| uuid | Unique IDs |
| chrono | Date/time |
| log | Logging |
| anyhow | Error handling |

## Common Issues & Solutions

| Issue | Solution |
|-------|----------|
| "cargo not found" | Install Rust: https://rustup.rs |
| Slow compilation | Use `mold` linker or `sccache` |
| Database locked | Ensure only one app instance |
| Extension not loading | Check manifest.json is valid |
| Permission denied | Check file permissions |

## Performance Tips

- Use `cargo build --release` for production
- Enable LTO: Add `[profile.release] lto = true` to Cargo.toml
- Profile with `cargo flamegraph`
- Check memory with `valgrind` (Linux)

## Git Workflow

```bash
# Create feature branch
git checkout -b feature/my-feature

# Make changes
git add .
git commit -m "feat: my feature description"

# Push to fork
git push origin feature/my-feature

# Create pull request on GitHub
```

## Documentation Files

| File | Content |
|------|---------|
| README.md | Main overview |
| QUICKSTART.md | Getting started |
| API.md | Complete API docs |
| ARCHITECTURE.md | System design |
| DEVELOPMENT.md | Build & debug |
| EXTENSIONS.md | Extension dev |
| CONTRIBUTING.md | How to contribute |
| ROADMAP.md | Feature roadmap |
| CHANGELOG.md | Version history |

## Feature Roadmap at a Glance

```
v0.1.0 ✅  Core Architecture (DONE)
v0.2.0 🔄 UI Foundation       (Q2 2024)
v0.3.0    Theming             (Q3 2024)
v0.4.0    Privacy             (Q4 2024)
v0.5.0    Unique Features     (Q1 2025)
v0.6.0    Cloud Sync          (Q2 2025)
v0.7.0    Performance         (Q3 2025)
v0.8.0    Extension Store     (Q4 2025)
v1.0.0    Production Ready    (Q1 2026)
```

## Getting Help

- **Docs**: See README.md and other gen docs
- **Issues**: https://github.com/4browser/4browser/issues
- **Discord**: https://discord.gg/4browser
- **Email**: contact@4browser.com

## Key Module Functions

### app.rs
```rust
BrowserApp::new()
app.run()
app.get_user_agent()
app.update_browser_name()
app.update_device_name()
```

### browser.rs
```rust
BrowserEngine::new()
engine.create_window()
engine.create_tab()
engine.navigate()
engine.add_to_history()
engine.add_bookmark()
```

### permissions.rs
```rust
PermissionManager::new()
pm.request_permission()
pm.set_permission()
pm.get_permissions()
pm.clear_permissions()
```

### extensions.rs
```rust
ExtensionManager::new()
em.install_extension()
em.enable_extension()
em.disable_extension()
em.uninstall_extension()
```

---

**📍 Pro Tips**:
- Keep DEVELOPMENT.md open while coding
- Reference examples/example-extension when creating extensions
- Use `RUST_LOG=debug` for troubleshooting
- Check tests in each module for usage examples

**Happy coding! 🚀**
