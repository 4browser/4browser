/**
 * Background Service Worker for 4 Browser Example Extension
 * 
 * This service worker:
 * - Tracks page navigation
 * - Maintains a page counter in storage
 * - Demonstrates basic extension patterns
 */

console.log('📦 Example Extension Service Worker loaded');

// Initialize storage
chrome.storage.local.get(['pageCount'], (result) => {
  if (result.pageCount === undefined) {
    chrome.storage.local.set({ pageCount: 0 });
    console.log('📊 Page counter initialized to 0');
  }
});

// Track navigation to a new page
chrome.webNavigation.onCommitted.addListener((details) => {
  // Only track main frame navigation, not iframes
  if (details.frameId === 0) {
    incrementPageCounter();
    
    const url = new URL(details.url);
    console.log(`📄 Navigating to: ${url.hostname}`);
  }
});

// Track tab updates
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === 'complete') {
    console.log(`✓ Tab ${tabId} fully loaded`);
  }
});

/**
 * Increment the page counter
 */
function incrementPageCounter() {
  chrome.storage.local.get(['pageCount'], (result) => {
    const newCount = (result.pageCount || 0) + 1;
    chrome.storage.local.set({ pageCount: newCount });
    console.log(`📊 Page count updated to: ${newCount}`);
  });
}

/**
 * Listen for messages from popup or content scripts
 */
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  console.log('📨 Message received:', request);
  
  if (request.action === 'getStats') {
    chrome.storage.local.get(null, (items) => {
      sendResponse({
        pageCount: items.pageCount || 0,
        timestamp: new Date().toISOString()
      });
    });
    return true; // Will respond asynchronously
  }
  
  if (request.action === 'resetStats') {
    chrome.storage.local.clear(() => {
      chrome.storage.local.set({ pageCount: 0 });
      sendResponse({ success: true });
      console.log('🔄 Stats reset');
    });
    return true;
  }
});

/**
 * Listen for extension events
 */
chrome.runtime.onInstalled.addListener((details) => {
  if (details.reason === 'install') {
    console.log('🎉 Example Extension installed!');
    // Could open welcome page here
  } else if (details.reason === 'update') {
    console.log('📦 Example Extension updated');
  }
});

/**
 * Periodic task example (runs every 30 seconds)
 */
setInterval(() => {
  chrome.storage.local.get(['pageCount'], (result) => {
    const count = result.pageCount || 0;
    console.log(`⏰ Periodic check - Page count: ${count}`);
  });
}, 30000);

console.log('✅ Example Extension ready!');
