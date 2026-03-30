# 📦 4 Browser Example Extension

A simple example extension for 4 Browser to help you get started with extension development.

## What This Extension Does

- **Counts pages you visit** - Maintains a counter in the popup
- **Shows current domain** - Displays the domain of the active tab
- **Demonstrates storage** - Uses chrome.storage API
- **Background service worker** - Shows how to run persistent logic
- **Popup interface** - Example of interactive UI

## Installation

### In 4 Browser

1. Go to **Settings** → **Extensions**
2. Enable **Developer Mode**
3. Click **Load Unpacked**
4. Navigate to this folder (`examples/example-extension`)
5. The extension is now installed!

### Using the Extension

1. Click the extension icon in the toolbar
2. See your page count and current domain
3. Click "Reset Counter" to clear the count
4. Browse websites to see the counter increment

## File Structure

```
example-extension/
├── manifest.json              # Extension metadata
├── popup/
│   ├── popup.html            # Popup UI
│   └── popup.js              # Popup logic
└── background/
    └── service-worker.js     # Background service worker
```

## How It Works

### manifest.json
Defines the extension:
- Permissions needed (storage, tabs, scripting)
- Popup UI location
- Background service worker
- Extension metadata

### popup/popup.html
Simple popup UI showing:
- Page counter
- Current domain
- Reset button
- Settings button

### popup/popup.js
Updates the popup:
- Fetches page count from storage
- Gets current domain
- Handles button clicks

### background/service-worker.js
Runs in background:
- Listens for page navigation
- Increments counter
- Stores data in chrome.storage

## Learning Path

### 1. Basic
- Modify popup colors
- Change counter display format
- Add new stats

### 2. Intermediate
- Add content scripts
- Inject CSS into pages
- Modify page content

### 3. Advanced
- Add context menus
- Use fetch API to call external services
- Implement options page
- Create shortcuts

## Next Steps

1. **Modify this extension** - Try changing colors, adding features
2. **Read the docs** - Check [EXTENSIONS.md](../../EXTENSIONS.md)
3. **Try other examples** - Look in `/examples` directory
4. **Share your extension** - Submit to 4 Browser store (coming soon)

## Common Modifications

### Change Extension Name
Edit `manifest.json`:
```json
"name": "My Custom Extension"
```

### Add New Permission
Edit `manifest.json` permissions:
```json
"permissions": [
  "activeTab",
  "scripting",
  "storage",
  "notifications"  // NEW
]
```

### Add New Popup Button
In `popup.html`:
```html
<button id="myBtn">Do Something</button>
```

In `popup.js`:
```javascript
document.getElementById('myBtn').addEventListener('click', () => {
  console.log('Button clicked!');
});
```

### Call Service Worker
```javascript
chrome.runtime.sendMessage(
  { action: 'myAction' },
  (response) => {
    console.log('Response:', response);
  }
);
```

## Debugging

### Check Console Logs
1. Open 4 Browser developer tools (Ctrl+Shift+I)
2. Logs from service worker appear in console
3. Logs from popup appear when popup is open

### Reload Extension
1. Go to Settings → Extensions
2. Find "Example Extension"
3. Click reload button
4. Changes apply immediately

### Clear Storage
```javascript
// In console or extension code
chrome.storage.local.clear(() => console.log('Cleared'));
```

## Features to Add

Try adding these features to learn more:

1. **Dark mode toggle** - Add checkbox in popup
2. **Domain whitelist** - Only count specific domains
3. **Time tracking** - Track time spent per site
4. **Export stats** - Download history as CSV
5. **Chart visualization** - Show pie chart of time per site
6. **Notifications** - Alert when spending time on site
7. **Filters** - Block certain sites

## Resources

- [4 Browser Extension Guide](../../EXTENSIONS.md)
- [API Reference](../../API.md)
- [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions)
- [Chrome Developer Docs](https://developer.chrome.com/docs/extensions/)

## Support

- Found a bug? [Report it](https://github.com/4browser/4browser/issues)
- Need help? [Join Discord](https://discord.gg/4browser)
- Want to learn more? [Read the docs](../../README.md)

---

**Happy coding! 🚀**

This example is licensed under MIT. Feel free to use it as a starting point for your own extensions!
