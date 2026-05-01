// Browser API interactions
const browserAPI = {
    baseURL: 'http://localhost:8080/api',

    async navigate() {
        const addressInput = document.getElementById('addressInput');
        const url = addressInput.value;
        if (!url) return;
        
        try {
            const response = await fetch(`${this.baseURL}/navigate`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ url })
            });
            const data = await response.json();
            console.log('Navigated to:', data);
        } catch (error) {
            console.error('Navigation error:', error);
        }
    },

    async goBack() {
        try {
            const response = await fetch(`${this.baseURL}/history/back`, { method: 'POST' });
            console.log('Going back:', await response.json());
        } catch (error) {
            console.error('Go back error:', error);
        }
    },

    async goForward() {
        try {
            const response = await fetch(`${this.baseURL}/history/forward`, { method: 'POST' });
            console.log('Going forward:', await response.json());
        } catch (error) {
            console.error('Go forward error:', error);
        }
    },

    async reload() {
        try {
            const response = await fetch(`${this.baseURL}/reload`, { method: 'POST' });
            console.log('Reloaded:', await response.json());
        } catch (error) {
            console.error('Reload error:', error);
        }
    },

    async newTab() {
        try {
            const response = await fetch(`${this.baseURL}/tabs/new`, { method: 'POST' });
            const data = await response.json();
            console.log('New tab created:', data);
            loadTabs();
        } catch (error) {
            console.error('New tab error:', error);
        }
    },

    async saveBrowserName() {
        const browserName = document.getElementById('browserName').value;
        try {
            const response = await fetch(`${this.baseURL}/settings/browser-name`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name: browserName })
            });
            console.log('Browser name saved:', await response.json());
        } catch (error) {
            console.error('Save browser name error:', error);
        }
    },

    async saveDeviceName() {
        const deviceName = document.getElementById('deviceName').value;
        try {
            const response = await fetch(`${this.baseURL}/settings/device-name`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name: deviceName })
            });
            console.log('Device name saved:', await response.json());
        } catch (error) {
            console.error('Save device name error:', error);
        }
    }
};

// Modal Functions
function openSettings() {
    const panel = document.getElementById('settingsPanel');
    panel.classList.remove('hidden');
}

function closeSettings() {
    const panel = document.getElementById('settingsPanel');
    panel.classList.add('hidden');
}

function openExtensions() {
    const panel = document.getElementById('extensionsPanel');
    panel.classList.remove('hidden');
    loadExtensions();
}

function closeExtensions() {
    const panel = document.getElementById('extensionsPanel');
    panel.classList.add('hidden');
}

function openBookmarks() {
    const panel = document.getElementById('bookmarksPanel');
    panel.classList.remove('hidden');
    loadBookmarks();
}

function closeBookmarks() {
    const panel = document.getElementById('bookmarksPanel');
    panel.classList.add('hidden');
}

// Load Data Functions
async function loadStats() {
    try {
        const response = await fetch(`${browserAPI.baseURL}/stats`);
        const stats = await response.json();
        
        document.getElementById('historyCount').textContent = stats.history_count || 0;
        document.getElementById('bookmarkCount').textContent = stats.bookmarks_count || 0;
        document.getElementById('extensionCount').textContent = stats.extensions_count || 0;
    } catch (error) {
        console.error('Failed to load stats:', error);
    }
}

async function loadTabs() {
    try {
        const response = await fetch(`${browserAPI.baseURL}/tabs`);
        const tabs = await response.json();
        
        const tabsList = document.getElementById('tabsList');
        tabsList.innerHTML = '';
        
        tabs.forEach(tab => {
            const tabElement = document.createElement('div');
            tabElement.className = 'tab-item';
            tabElement.textContent = tab.name || 'New Tab';
            tabElement.onclick = () => selectTab(tab.id);
            tabsList.appendChild(tabElement);
        });
    } catch (error) {
        console.error('Failed to load tabs:', error);
    }
}

async function loadExtensions() {
    try {
        const response = await fetch(`${browserAPI.baseURL}/extensions`);
        const extensions = await response.json();
        
        const list = document.getElementById('extensionsList');
        list.innerHTML = '';
        
        extensions.forEach(ext => {
            const item = document.createElement('div');
            item.className = 'extension-item';
            item.innerHTML = `
                <strong>${ext.name}</strong>
                <p>${ext.description}</p>
                <small>Version: ${ext.version}</small>
            `;
            list.appendChild(item);
        });
    } catch (error) {
        console.error('Failed to load extensions:', error);
    }
}

async function loadBookmarks() {
    try {
        const response = await fetch(`${browserAPI.baseURL}/bookmarks`);
        const bookmarks = await response.json();
        
        const list = document.getElementById('bookmarksList');
        list.innerHTML = '';
        
        bookmarks.forEach(bookmark => {
            const item = document.createElement('div');
            item.className = 'bookmark-item';
            item.innerHTML = `
                <strong>${bookmark.name}</strong>
                <p><a href="${bookmark.url}" target="_blank">${bookmark.url}</a></p>
            `;
            list.appendChild(item);
        });
    } catch (error) {
        console.error('Failed to load bookmarks:', error);
    }
}

async function selectTab(tabId) {
    try {
        const response = await fetch(`${browserAPI.baseURL}/tabs/${tabId}`, {
            method: 'POST'
        });
        console.log('Tab selected:', await response.json());
        loadTabs();
    } catch (error) {
        console.error('Failed to select tab:', error);
    }
}

// Address bar keyboard handling
document.addEventListener('DOMContentLoaded', () => {
    const addressInput = document.getElementById('addressInput');
    
    addressInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            browserAPI.navigate();
        }
    });

    // Close modals on background click
    const modals = document.querySelectorAll('.modal');
    modals.forEach(modal => {
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                modal.classList.add('hidden');
            }
        });
    });

    // Load initial data
    loadStats();
    loadTabs();
});

// Refresh stats every 10 seconds
setInterval(loadStats, 10000);
