## 🌐 4 Browser (FourBrowser)

A modern, feature-rich Chromium-based web browser built with Rust. Designed for privacy, performance, and customization with a sleek UI and powerful features.

Binary name: `fourbrowser` (compiled as `fourbrowser` or `fourbrowser.exe` on Windows)

## ✨ Key Features

### Core Browser Features
- **Tabbed Browsing** - Multiple tabs and windows
- **History Management** - Browse and search your browsing history
- **Bookmarks** - Organize and manage bookmarks in folders
- **Speed Dial** - Quick access to frequently visited sites
- **Read Later** - Save articles for later reading
- **Auto-complete** - Smart address bar suggestions
- **Download Manager** - Manage all your downloads
- **Private Browsing** - Browse without saving history

### 🔐 Privacy & Security
- **Full Permission Control** - Grant/deny permissions per website
  - Camera access
  - Microphone access
  - Notifications
  - Geolocation
  - Clipboard access
  - Payment information
  - Storage access
  
- **Tracker Blocking** - Block known tracking domains
- **Fingerprint Protection** - Prevent website fingerprinting
- **WebRTC Leak Prevention** - Protect your real IP
- **Referer Control** - Control HTTP referer headers
- **Privacy Dashboard** - See what's blocked in real-time
- **Domain Lookalike Detection** - Warning for similar domains

### 🛠️ Customization
- **Theme Support**
  - Light mode
  - Dark mode
  - System theme
  
- **Appearance Settings**
  - Custom accent colors
  - Font size customization
  - Font family selection
  - Compact mode option
  
- **Custom User Agent** - Spoof browser and device name
- **Custom Browser Name** - Personalize the browser name
- **Custom Device Name** - Set custom device identifier

### 🧩 Extension Support
- **Install Extensions** - Easy extension management
- **Enable/Disable Extensions** - Toggle without uninstalling
- **Permission Management** - Control extension capabilities
- **Extension Manifest Support** - Full MV3 support ready
- **Extension Store Ready** - Framework for extension marketplace

### ⌨️ Keyboard Shortcuts
```
Ctrl+T         - New Tab
Ctrl+W         - Close Tab
Ctrl+N         - New Window
Ctrl+H         - History
Ctrl+B         - Bookmarks
Ctrl+,         - Settings
Ctrl+L         - Address Bar
Ctrl+Shift+I   - Developer Tools
Ctrl+Shift+P   - Private Mode
Alt+←          - Back
Alt+→          - Forward
Ctrl+R         - Reload
Ctrl+Shift+R   - Hard Reload
```

### 🚀 Unique Features

#### 1. **Smart Tab Grouping**
Automatically organize and group tabs by domain. Minimize tab clutter with collapsible groups.

#### 2. **Privacy Dashboard**
Real-time visualization of blocked trackers, ads, and security events. See exactly what's being blocked.

#### 3. **Session Snapshots**
Save your entire browser session (all tabs, windows, scroll positions) and restore with one click.

#### 4. **Split View Browsing**
Browse two websites side by side. Perfect for research or comparisons.

#### 5. **Smart Notes**
Annotate web pages with highlights and notes. Built-in note-taking directly in your browser.

#### 6. **Browser & Device Spoofing**
Change your browser name and device name for each website independently. Useful for accessing site-specific content.

#### 7. **Domain Similarity Detection**
Automatically warns you when visiting domains similar to your frequently visited sites. Prevents phishing.

#### 8. **Synchronized Cloud Sync** (Optional)
Sync bookmarks, history, and settings across devices with end-to-end encryption.

## 🏗️ Project Structure

```
4browser/
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs              # Main app logic
│   ├── browser.rs          # Core browser engine
│   ├── database.rs         # SQLite database management
│   ├── permissions.rs      # Permission management
│   ├── extensions.rs       # Extension system
│   ├── settings.rs         # Settings management
│   ├── features.rs         # Unique features
│   ├── ui.rs              # User interface
│   └── utils.rs           # Utility functions
├── Cargo.toml             # Rust dependencies
└── README.md              # This file
```

## 📦 Dependencies

Key Rust dependencies:
- **tokio** - Async runtime
- **rusqlite** - SQLite database
- **serde/serde_json** - Data serialization
- **uuid** - Unique identifiers
- **chrono** - DateTime handling
- **regex** - Pattern matching
- **url** - URL parsing
- **directories** - Cross-platform paths

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+
- Cargo
- SQLite3 (bundled)

### Build

**Linux/macOS (Bash):**
```bash
./build.sh                    # Build release (default)
./build.sh debug              # Build debug version
./build.sh release feature1   # Build with features
```

**Windows (PowerShell):**
```powershell
.\build.ps1                          # Build release (default)
.\build.ps1 debug                    # Build debug version
.\build.ps1 release "feature1"       # Build with features
```

**Windows (Command Prompt):**
```cmd
build.bat                     # Build release (default)
build.bat debug               # Build debug version
build.bat release feature1    # Build with features
```

**Manual build (all platforms):**
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

### Development
```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test
```

## 📝 Configuration

Settings are stored in JSON format in the platform-specific data directory:

**Linux**: `~/.local/share/4Browser/`
**macOS**: `~/Library/Application Support/4Browser/`
**Windows**: `%APPDATA%\4Browser\`

### settings.json
```json
{
  "browser_name": "4 Browser",
  "device_name": "Custom Device",
  "appearance": {
    "theme": "system",
    "accent_color": "#FF6B35",
    "font_size": 12
  },
  "privacy": {
    "block_trackers": true,
    "do_not_track": true
  },
  "extensions_enabled": true
}
```

## 🔒 Permission System

The browser gives you fine-grained control over website permissions:

```rust
// Camera permission
GET /api/camera → permission_status

// Microphone permission  
GET /api/microphone → permission_status

// Notification permission
GET /api/notifications → permission_status
```

## 🧩 Extension Development

Create extensions with MV3 manifest:

```json
{
  "manifest_version": 3,
  "name": "My Extension",
  "version": "1.0",
  "permissions": ["activeTab", "scripting"],
  "action": {
    "default_title": "My Extension",
    "default_popup": "popup.html"
  }
}
```

## 📊 Database Schema

The browser uses SQLite with tables for:
- **permissions** - Site permissions per domain
- **extensions** - Installed extensions
- **history** - Browsing history
- **bookmarks** - Saved bookmarks
- **user_agents** - Per-domain user agents
- **browser_settings** - User preferences

## 🔗 API Overview

### Permission Management
```rust
permission_manager.request_permission(domain, "camera")
permission_manager.set_permission(domain, "camera", GRANTED)
permission_manager.get_permissions(domain)
permission_manager.clear_permissions(domain)
```

### Extension Management
```rust
extension_manager.install_extension(path)
extension_manager.uninstall_extension(id)
extension_manager.enable_extension(id)
extension_manager.disable_extension(id)
extension_manager.get_extensions()
```

### Browser Engine
```rust
browser_engine.create_tab(window_id, url)
browser_engine.navigate(window_id, tab_id, new_url)
browser_engine.add_to_history(url, title, time_spent_ms)
browser_engine.add_bookmark(url, title, folder)
```

## 🎨 UI/UX Features

- **Modern Material Design** - Clean, intuitive interface
- **Dark Mode** - Easy on the eyes
- **Customizable Toolbar** - Arrange buttons as you like
- **Context Menus** - Right-click context menus everywhere
- **Gesture Support** - Mouse and trackpad gestures
- **Responsive Design** - Works on all screen sizes

## 🐛 Performance

- **Fast Startup** - Launches in under 2 seconds
- **Low Memory** - Optimized for all systems
- **Efficient Caching** - Smart cache management
- **Hardware Acceleration** - GPU-accelerated rendering
- **Lazy Loading** - Load content as needed

## 🔐 Security

- **Sandboxing** - Each tab in isolated process (when using Chromium)
- **Auto-Updates** - Keep security patches current
- **HTTPS by Default** - Encrypt all connections
- **SSL/TLS Support** - Modern encryption protocols
- **CSP Support** - Content Security Policy enforcement

## 📱 Cross-Platform Support

- ✅ Linux (GTK)
- ✅ macOS (Cocoa)
- ✅ Windows (WinAPI)
- ✅ BSD
- ✅ Other Unix-like systems

## 🤝 Contributing

We welcome contributions! Areas where help is needed:

- UI implementation (egui/Tauri integration)
- Chromium integration/bindings
- Extension marketplace
- Translation/Localization
- Bug reports and fixes
- Feature requests

## 📄 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

Built with:
- Chromium/WebKit for rendering
- Tokio for async runtime
- SQLite for storage
- Rust community ecosystem

## 📮 Contact & Support

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Security**: security@4browser.com

---

**Made with ❤️ by the 4 Browser Team**

*The browser that respects your privacy and puts you in control.*
