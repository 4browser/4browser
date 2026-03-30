// Update stats on popup open
document.addEventListener('DOMContentLoaded', updateStats);

// Update stats every second
setInterval(updateStats, 1000);

// Reset counter button
document.getElementById('resetBtn').addEventListener('click', () => {
  chrome.storage.local.clear(() => {
    console.log('Storage cleared');
    updateStats();
  });
});

// Settings button
document.getElementById('settingsBtn').addEventListener('click', () => {
  console.log('Settings clicked - open options page');
  // In a full implementation, this would open an options page
});

function updateStats() {
  // Get page count
  chrome.storage.local.get(['pageCount'], (result) => {
    const count = result.pageCount || 0;
    document.getElementById('pageCount').textContent = count.toString();
  });
  
  // Get current domain
  chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
    if (tabs.length > 0) {
      try {
        const url = new URL(tabs[0].url);
        const domain = url.hostname || 'unknown';
        document.getElementById('currentDomain').textContent = domain;
      } catch (e) {
        document.getElementById('currentDomain').textContent = 'N/A';
      }
    }
  });
}
