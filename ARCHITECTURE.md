# 🎯 Architecture Guide

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      4 Browser UI                           │
│           (egui/Tauri - Modern Material Design)            │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────┴──────────────────────────────────────┐
│                   Application Layer                         │
├──────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐    │
│  │  App Core   │  │   Settings   │  │   Permissions   │    │
│  └─────────────┘  └──────────────┘  └─────────────────┘    │
│  ┌──────────────────────────────────────────────────────┐   │
│  │            Browser Engine                            │   │
│  │  • Tab Management                                    │   │
│  │  • Navigation                                        │   │
│  │  • History Tracking                                  │   │
│  │  • Bookmark Management                               │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Extensions  │  │  Features    │  │  Utils       │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────┴──────────────────────────────────────┐
│              Data Storage Layer                             │
├──────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐    │
│  │         SQLite Database (4browser.db)              │    │
│  │  • Permissions Table                               │    │
│  │  • Extensions Table                                │    │
│  │  • History Table                                   │    │
│  │  • Bookmarks Table                                 │    │
│  │  • User Agents Table                               │    │
│  │  • Settings Table                                  │    │
│  └─────────────────────────────────────────────────────┘    │
│  ┌─────────────────────────────────────────────────────┐    │
│  │         File System Storage                         │    │
│  │  • settings.json                                    │    │
│  │  • permissions.json                                 │    │
│  │  • history.json                                     │    │
│  │  • bookmarks.json                                   │    │
│  │  • extensions/                                      │    │
│  └─────────────────────────────────────────────────────┘    │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────┴──────────────────────────────────────┐
│              Runtime Layer                                  │
├──────────────────────────────────────────────────────────────┤
│         Tokio Async Runtime (Multi-threaded)               │
│         • Task Scheduling                                  │
│         • Thread Pool                                      │
│         • I/O Operations                                   │
└──────────────────────────────────────────────────────────────┘
```

## Module Dependencies

```
main.rs
  ├── app.rs (Orchestrator)
  │   ├── browser.rs (Core Engine)
  │   ├── permissions.rs (Permission Manager)
  │   ├── extensions.rs (Extension Manager)
  │   ├── settings.rs (Settings Manager)
  │   └── ui.rs (UI Layer)
  │
  ├── database.rs (SQLite Layer)
  ├── utils.rs (Utilities)
  └── features.rs (Unique Features)
```

## Data Flow

### Permission Request Flow
```
Website API Request
    ↓
Permissions Module
    ↓
Check Permission Status in Database
    ↓
If NotResponded → Show Permission Dialog
    ↓
Update Database
    ↓
Return Permission Response to Website
```

### Extension Installation Flow
```
User selects extension file/folder
    ↓
ExtensionManager validates manifest.json
    ↓
Copy to extensions directory
    ↓
Load manifest into memory
    ↓
Inject content scripts if enabled
    ↓
Add to installed extensions list
```

### Settings Update Flow
```
User changes setting in UI
    ↓
Settings struct updated in memory
    ↓
Settings saved to settings.json
    ↓
Database updated if applicable
    ↓
Browser restart or hot reload
    ↓
Changes applied
```

## Threading Model

- **Main Thread**: UI event loop and rendering
- **Async Tasks**: Database operations, I/O, network requests
- **Background Tasks**: 
  - History cleanup
  - Permission expiration checking
  - Extension background scripts
  - Sync operations

## Key Design Patterns

### 1. Arc + RwLock Pattern
Used for thread-safe shared state:
```rust
Arc<RwLock<HashMap<String, Extension>>>
```

### 2. Async/Await
All I/O operations are async:
```rust
pub async fn load_permissions(&self) -> Result<()> { ... }
```

### 3. Result-based Error Handling
All functions return Result<T>:
```rust
pub async fn install_extension(&self, path: &Path) -> Result<String>
```

### 4. Builder Pattern
For complex initialization:
```rust
BrowserApp::new(app_data_dir, settings, permission_manager, extension_manager)
```

## State Management

```
BrowserApp (Root State)
  ├── Arc<RwLock<Settings>> (Global Settings)
  ├── BrowserEngine
  │   ├── Arc<RwLock<HashMap<Window>>>
  │   ├── Arc<RwLock<Vec<History>>>
  │   └── Arc<RwLock<Vec<Bookmark>>>
  ├── PermissionManager
  │   ├── Arc<RwLock<HashMap<Domain, PermissionState>>>
  │   └── Arc<RwLock<Vec<PermissionRequest>>>
  └── ExtensionManager
      └── Arc<RwLock<HashMap<ExtensionId, Extension>>>
```

## Persistence Strategy

### SQLite Database
- **When**: Critical data (permissions, extensions metadata)
- **What**: Structured queries, relationships
- **Speed**: O(log n) lookups

### JSON Files
- **When**: Settings and user data
- **What**: User preferences, history, bookmarks
- **Speed**: O(n) but fast for small datasets

### File System
- **When**: Extension files, cached resources
- **What**: Extension code, assets
- **Speed**: Direct file I/O

## Performance Considerations

### Memory Optimization
- Lazy loading of extensions
- History pruning (auto-delete old entries)
- Efficient HashMap structures

### CPU Optimization
- Async I/O operations
- Batch database updates
- Tokio's efficient task scheduling

### I/O Optimization
- Connection pooling for SQLite
- Async file operations
- Buffered writes

## Security Considerations

### Permission System
- Per-domain isolation
- Explicit permission model
- No implicit grants

### Extension Safety
- Sandbox execution
- Limited API surface
- Permission manifest requirements

### Data Protection
- User agent spoofing prevents fingerprinting
- Private browsing doesn't store history
- Encrypted sync capability (future)

## Extensibility Points

1. **Custom Search Engines**: Add SearchEngineConfig
2. **Themes**: Extend AppearanceSettings
3. **Privacy Filters**: Add FilterRules to privacy system
4. **Custom Keyboard Shortcuts**: Extend UI event handlers
5. **Plugin System**: Build on ExtensionManager

## Future Scalability

### Planned Improvements
1. Multi-process architecture (separate renderer processes)
2. Cloud sync with end-to-end encryption
3. AI-powered features (smart suggestions, auto-tagging)
4. Performance profiling dashboard
5. Advanced tab management with ML grouping

### Modular Design
All components are loosely coupled and can be:
- Replaced (different database backend)
- Extended (custom extension type)
- Improved (faster algorithms)
- Tested (unit testable modules)
