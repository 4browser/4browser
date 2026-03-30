# 🚀 Quick Start Guide

Get 4 Browser up and running in minutes!

## Installation

### From Source

#### Step 1: Clone Repository
```bash
git clone https://github.com/4browser/4browser.git
cd 4browser
```

#### Step 2: Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### Step 3: Build
```bash
cargo build --release
```

#### Step 4: Run
```bash
./target/release/4browser
```

### From Binary (Coming Soon)
Download pre-built binaries for your platform from the [releases page](https://github.com/4browser/4browser/releases).

## First Steps

### 1. Configure Your Identity
When you first launch 4 Browser:

1. Click **Settings** (Ctrl+,)
2. Go to **Appearance**
   - Choose your theme (Light/Dark/System)
   - Select accent color (#FF6B35 is recommended)
3. Go to **Privacy & Security** 
   - Set custom **Browser Name** (e.g., "Firefox" for website compatibility)
   - Set custom **Device Name** (e.g., "Windows Machine")

### 2. Set Up Permissions
1. Go to **Settings** → **Privacy & Security**
2. Review default permissions for:
   - Camera
   - Microphone
   - Notifications
   - Geolocation
   - Clipboard access

### 3. Install Your First Extension
1. Go to **Settings** → **Extensions**
2. Click **Load Unpacked**
3. Select a local extension folder
4. Extension is installed and enabled!

## Basic Usage

### Keyboard Shortcuts
```
Ctrl+T      New Tab
Ctrl+W      Close Tab
Ctrl+N      New Window
Ctrl+L      Focus Address Bar
Ctrl+B      Toggle Bookmarks Bar
Ctrl+,      Settings
Ctrl+H      History
Alt+←       Back
Alt+→       Forward
Ctrl+R      Reload Page
```

### Managing Tabs
- Click **+** to create new tab
- Right-click tab for options
- Drag tabs to reorder
- Tabs auto-group by domain (Smart Tab Grouping)

### Using Bookmarks
1. Visit a website
2. Click the star icon in address bar
3. Bookmarks auto-organized and searchable

### Privacy Features
1. **Permissions**: Grant/deny per-website in dropdown
2. **Privacy Dashboard**: View blocked trackers (click shield icon)
3. **Private Mode**: Ctrl+Shift+P for no history saving
4. **Developer Tools**: Ctrl+Shift+I to inspect

## Creating Your First Extension

### Simple Extension: Page Color Inverter

1. **Create folder structure:**
```
my-extension/
├── manifest.json
├── popup.html
└── popup.js
```

2. **manifest.json:**
```json
{
  "manifest_version": 3,
  "name": "Color Inverter",
  "version": "1.0",
  "permissions": ["activeTab", "scripting"],
  "action": {
    "default_title": "Invert Colors",
    "default_popup": "popup.html"
  }
}
```

3. **popup.html:**
```html
<button id="invert">Invert Colors</button>
<script src="popup.js"></script>
```

4. **popup.js:**
```javascript
document.getElementById('invert').addEventListener('click', () => {
  chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
    chrome.scripting.executeScript({
      target: { tabId: tabs[0].id },
      function: () => {
        document.documentElement.style.filter = 'invert(1)';
      }
    });
  });
});
```

5. **Load in 4 Browser:**
   - Settings → Extensions → Load Unpacked
   - Select `my-extension` folder
   - Click the extension icon in toolbar to use!

## Tips & Tricks

### 🎨 Customize Appearance
- Try different themes and colors
- Use compact mode for more screen space
- Add custom CSS through developer tools

### 🔒 Privacy Pro Tips
- Review permissions on sites you don't fully trust
- Use Private Mode for shopping (prevents price tracking)
- Enable tracker blocking for all sites
- Use custom device names to prevent fingerprinting

### ⚡ Performance Tips
- Close unused tabs to save memory
- Use sessions to organize workflows
- Enable "Hardware acceleration" in Settings
- Clear cache regularly

### 🧩 Extension Tips
- Disable unused extensions
- Check extension permissions before installing
- Keep extensions updated
- Report suspicious extensions

### ⌨️ Power User Shortcuts
```
Ctrl+Shift+T    Restore last closed tab
Ctrl+Tab        Switch to next tab
Ctrl+Shift+Tab  Switch to previous tab
Ctrl+1-8        Go to specific tab
Ctrl+9          Go to last tab
Ctrl+Shift+M    Switch profile
Ctrl+Shift+P    Private Mode
```

## Troubleshooting

### Browser Won't Start
```bash
# Check logs
RUST_LOG=debug ./target/release/4browser

# Clear corrupted settings
rm ~/.local/share/4Browser/settings.json  # Linux
rm ~/Library/Application\ Support/4Browser/settings.json  # macOS
del %APPDATA%/4Browser/settings.json  # Windows
```

### Extension Not Loading
1. Check manifest.json is valid JSON
2. Verify all file paths are correct
3. Enable developer mode in Settings
4. Check console for errors (F12)

### Slow Performance
1. Close unused tabs
2. Disable unnecessary extensions
3. Clear cache: Settings → Privacy → Clear browsing data
4. Check available RAM: `top` or Task Manager
5. File an issue if persistent

## Getting Help

- **Documentation**: Read the [full README](README.md)
- **API Reference**: Check [API.md](API.md)
- **Extension Dev**: See [EXTENSIONS.md](EXTENSIONS.md)
- **Issues**: [GitHub Issues](https://github.com/4browser/4browser/issues)
- **Community**: [Discord Server](https://discord.gg/4browser)

## Next Steps

1. Explore unique features:
   - Save a session snapshot
   - Enable split-screen browsing
   - Try smart notes on articles
   
2. Create your first extension (see above)

3. Join the community and contribute!

4. Follow [@4BrowserApp](https://twitter.com/4BrowserApp) for updates

---

**Happy Browsing! 🌐**

Questions? Join our [Discord community](https://discord.gg/4browser) or open an issue on [GitHub](https://github.com/4browser/4browser/issues).
