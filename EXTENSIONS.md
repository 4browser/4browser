# 🧩 Extension Development Guide

## Getting Started with 4 Browser Extensions

4 Browser supports modern Web Extensions (MV3 compatible). Extensions allow you to add custom functionality and features to the browser.

## Extension Structure

```
my-extension/
├── manifest.json           # Extension metadata and permissions
├── icons/
│   ├── icon-16.png        # 16x16 icon
│   ├── icon-48.png        # 48x48 icon
│   └── icon-128.png       # 128x128 icon
├── popup/
│   ├── popup.html         # Popup UI
│   ├── popup.css          # Popup styles
│   └── popup.js           # Popup script
├── background/
│   └── service-worker.js  # Background service worker
├── content/
│   ├── content.js         # Content script
│   └── style.css          # Content styles
├── options/
│   ├── options.html       # Extension options page
│   ├── options.css        # Options styles
│   └── options.js         # Options script
└── lib/
    └── utils.js           # Shared utilities
```

## Manifest.json

The manifest.json file defines your extension:

```json
{
  "manifest_version": 3,
  "name": "My Awesome Extension",
  "version": "1.0.0",
  "description": "A brief description of what your extension does",
  "author": "Your Name",
  "homepage_url": "https://github.com/yourname/my-extension",
  
  "icons": {
    "16": "icons/icon-16.png",
    "48": "icons/icon-48.png",
    "128": "icons/icon-128.png"
  },
  
  "permissions": [
    "activeTab",
    "scripting",
    "storage",
    "tabs"
  ],
  
  "optional_permissions": [
    "clipboardWrite",
    "clipboardRead",
    "webRequest",
    "webNavigation"
  ],
  
  "host_permissions": [
    "https://*.example.com/*",
    "https://github.com/*"
  ],
  
  "action": {
    "default_title": "My Extension",
    "default_popup": "popup/popup.html",
    "default_icon": "icons/icon-128.png"
  },
  
  "background": {
    "service_worker": "background/service-worker.js"
  },
  
  "content_scripts": [
    {
      "matches": ["https://*.example.com/*"],
      "js": ["content/content.js"],
      "css": ["content/style.css"],
      "run_at": "document_start"
    }
  ],
  
  "options_page": "options/options.html"
}
```

## Types of Extensions

### 1. Action Extension (Simple Popup)

**manifest.json:**
```json
{
  "manifest_version": 3,
  "name": "Quick Notes",
  "version": "1.0",
  "permissions": ["storage"],
  "action": {
    "default_popup": "popup.html"
  }
}
```

**popup.html:**
```html
<!DOCTYPE html>
<html>
<head>
  <style>
    body { width: 300px; font-family: arial; }
    textarea { width: 100%; height: 150px; }
  </style>
</head>
<body>
  <h3>Quick Notes</h3>
  <textarea id="notes"></textarea>
  <button id="save">Save</button>
  <script src="popup.js"></script>
</body>
</html>
```

**popup.js:**
```javascript
document.getElementById('save').addEventListener('click', () => {
  const notes = document.getElementById('notes').value;
  chrome.storage.sync.set({ notes: notes }, () => {
    alert('Notes saved!');
  });
});

// Load saved notes
chrome.storage.sync.get(['notes'], (items) => {
  if (items.notes) {
    document.getElementById('notes').value = items.notes;
  }
});
```

### 2. Content Script Extension

Modifies web pages by injecting scripts:

**manifest.json:**
```json
{
  "manifest_version": 3,
  "name": "Dark Mode Everywhere",
  "version": "1.0",
  "permissions": ["scripting", "activeTab"],
  "content_scripts": [
    {
      "matches": ["<all_urls>"],
      "js": ["content.js"],
      "css": ["dark-mode.css"]
    }
  ]
}
```

**dark-mode.css:**
```css
* {
  background-color: #1a1a1a !important;
  color: #e0e0e0 !important;
}

a { color: #64b5f6 !important; }
```

**content.js:**
```javascript
console.log('Dark Mode Enabled!');

// Modify DOM as needed
document.documentElement.setAttribute('data-theme', 'dark');
```

### 3. Background Service Worker Extension

Runs persistent logic in background:

**manifest.json:**
```json
{
  "manifest_version": 3,
  "name": "Auto Save",
  "version": "1.0",
  "permissions": ["storage", "webRequest"],
  "background": {
    "service_worker": "service-worker.js"
  }
}
```

**service-worker.js:**
```javascript
// Listen for page navigation
chrome.webNavigation.onCompleted.addListener((details) => {
  console.log('Page loaded:', details.url);
  
  // Increment page counter
  chrome.storage.local.get(['pageCount'], (items) => {
    const count = (items.pageCount || 0) + 1;
    chrome.storage.local.set({ pageCount: count });
  });
});

// Listen for messages from content scripts
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.action === 'getCount') {
    chrome.storage.local.get(['pageCount'], (items) => {
      sendResponse({ count: items.pageCount || 0 });
    });
    return true; // Will respond asynchronously
  }
});
```

## Available APIs

### Storage API
```javascript
// Save data
chrome.storage.sync.set({ key: 'value' }, () => {
  console.log('Saved');
});

// Get data
chrome.storage.sync.get(['key'], (items) => {
  console.log(items.key);
});

// Listen for changes
chrome.storage.onChanged.addListener((changes, areaName) => {
  for (let [key, { oldValue, newValue }] of Object.entries(changes)) {
    console.log(`${key} changed from ${oldValue} to ${newValue}`);
  }
});
```

### Messaging API
```javascript
// Send message from content to background
chrome.runtime.sendMessage({ action: 'myAction', data: 'data' }, (response) => {
  console.log(response);
});

// Listen for messages
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.action === 'myAction') {
    sendResponse({ result: 'success' });
  }
});
```

### Tab API
```javascript
// Get current tab
chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
  const currentTab = tabs[0];
  console.log(currentTab.url);
});

// Execute script in tab
chrome.scripting.executeScript({
  target: { tabId: tab.id },
  function: () => console.log('Executed in page!')
});
```

### Notifications API
```javascript
// Show notification
chrome.notifications.create({
  type: 'basic',
  iconUrl: 'icons/icon-128.png',
  title: 'Notification Title',
  message: 'Notification message'
});

// Listen for notification clicks
chrome.notifications.onClicked.addListener((notificationId) => {
  console.log('Notification clicked:', notificationId);
});
```

## Permission Reference

### Common Permissions
```json
"permissions": [
  "activeTab",           // Access current tab info
  "scripting",          // Execute scripts
  "tabs",               // Get tab information
  "storage",            // Use storage API
  "alarms",             // Schedule tasks
  "background",         // Run background code
  "contextMenus",       // Add context menu items
  "notifications",      // Show notifications
  "cookies",            // Access cookies
  "webRequest",         // Monitor web requests
  "webNavigation"       // Monitor page navigation
]
```

### Host Permissions
```json
"host_permissions": [
  "https://*.example.com/*",          // Specific domain
  "https://*/api/*",                   // API endpoints
  "https://*/*",                       // HTTPS everywhere
  "<all_urls>"                         // All websites (use sparingly)
]
```

## Debugging Extensions

### 1. Enable Developer Mode
- Open 4Browser Settings
- Go to Extensions tab
- Toggle "Developer Mode"

### 2. Load Unpacked Extension
- Click "Load Unpacked"
- Select your extension folder

### 3. Check Errors and Logs
- View extension logs in 4Browser console
- Check browser console for errors
- Use popup.html for debugging popups

### 4. Test Your Extension
```javascript
// Add test logs
console.log('Extension loaded');
console.log('Manifest:', chrome.runtime.getManifest());

// Test storage
chrome.storage.local.set({ test: 'value' });
chrome.storage.local.get(['test'], console.log);
```

## Best Practices

### 1. Performance
```javascript
// ❌ Bad: Heavy computation on every event
window.addEventListener('mousemove', () => {
  expensiveCalculation();
});

// ✅ Good: Throttle/debounce
function debounce(func, wait) {
  let timeout;
  return () => {
    clearTimeout(timeout);
    timeout = setTimeout(func, wait);
  };
}

window.addEventListener('mousemove', debounce(expensiveCalculation, 250));
```

### 2. Security
```javascript
// ❌ Bad: Using eval
eval(userInput);

// ✅ Good: Safe alternatives
JSON.parse(userInput);
new Function(userInput)();  // Still risky, use carefully
```

### 3. Memory Management
```javascript
// Always clean up listeners
function addListener() {
  const handler = () => console.log('Event');
  element.addEventListener('click', handler);
  
  // Clean up when done
  return () => element.removeEventListener('click', handler);
}
```

### 4. Error Handling
```javascript
try {
  const result = await complexOperation();
} catch (error) {
  console.error('Operation failed:', error);
  // Notify user appropriately
}
```

## Publishing Your Extension

### 1. Prepare for Release
- Test thoroughly
- Add proper icons (16x16, 48x48, 128x128)
- Write clear description
- Add version number

### 2. Package Extension
```bash
# Create ZIP file
zip -r my-extension my-extension/

# Exclude unnecessary files
zip -r my-extension my-extension/ -x "node_modules/*" ".git/*" ".gitignore"
```

### 3. Submit to Store (Future)
- Create account on 4 Browser Extension Store
- Upload ZIP file
- Complete store listing
- Wait for review and approval

## Example: Website Tracker Extension

Complete example of a tracking extension:

**manifest.json:**
```json
{
  "manifest_version": 3,
  "name": "Website Time Tracker",
  "version": "1.0",
  "description": "Track time spent on websites",
  "permissions": ["tabs", "webNavigation", "storage"],
  "background": {
    "service_worker": "service-worker.js"
  },
  "action": {
    "default_popup": "popup.html",
    "default_title": "Time Tracker"
  }
}
```

**service-worker.js:**
```javascript
let currentTab = null;
let lastTimestamp = Date.now();

chrome.webNavigation.onCommitted.addListener((details) => {
  if (details.frameId === 0) {  // Main frame only
    currentTab = details.tabId;
    lastTimestamp = Date.now();
  }
});

chrome.tabs.onActivated.addListener((activeInfo) => {
  currentTab = activeInfo.tabId;
  lastTimestamp = Date.now();
});

chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === 'complete' && currentTab === tabId) {
    lastTimestamp = Date.now();
  }
});

// Track time every minute
setInterval(async () => {
  if (!currentTab) return;
  
  const tab = await chrome.tabs.get(currentTab);
  const url = new URL(tab.url);
  const domain = url.hostname;
  
  const key = `time_${domain}`;
  const data = await chrome.storage.local.get([key]);
  const currentTime = data[key] || 0;
  const elapsed = (Date.now() - lastTimestamp) / 1000;  // seconds
  
  await chrome.storage.local.set({
    [key]: currentTime + elapsed
  });
  
  lastTimestamp = Date.now();
}, 60000);
```

**popup.html:**
```html
<!DOCTYPE html>
<html>
<head>
  <style>
    body { width: 300px; font-family: monospace; margin: 10px; }
    .domain { display: flex; justify-content: space-between; padding: 5px 0; }
    .time { color: #666; }
  </style>
</head>
<body>
  <h2>Time Tracked</h2>
  <div id="stats"></div>
  <script src="popup.js"></script>
</body>
</html>
```

**popup.js:**
```javascript
async function showStats() {
  const data = await chrome.storage.local.get(null);
  const stats = document.getElementById('stats');
  
  for (let [key, value] of Object.entries(data)) {
    if (key.startsWith('time_')) {
      const domain = key.replace('time_', '');
      const minutes = Math.round(value / 60);
      const hours = Math.floor(minutes / 60);
      const mins = minutes % 60;
      
      const timeStr = hours > 0 
        ? `${hours}h ${mins}m`
        : `${mins}m`;
      
      stats.innerHTML += `
        <div class="domain">
          <span>${domain}</span>
          <span class="time">${timeStr}</span>
        </div>
      `;
    }
  }
}

showStats();
```

## Resources

- [MDN Web Docs - WebExtensions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions)
- [Chrome Web Store API](https://developer.chrome.com/docs/extensions/)
- [4 Browser Extension Examples](https://github.com/4browser/extension-examples)

## Support

- Discord: [4 Browser Developers](https://discord.gg/4browser)
- Issues: [GitHub Issues](https://github.com/4browser/4browser/issues)
- Email: extensions@4browser.com
