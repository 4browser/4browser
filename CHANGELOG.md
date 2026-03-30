# Changelog

All notable changes to 4 Browser will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- Foundation for browser engine with tab and window management
- SQLite database for persistent storage
- Permission system with per-domain control
  - Camera, microphone, notifications
  - Geolocation, clipboard, storage, payment
- Extension system with MV3 manifest support
- Settings system with theme and appearance customization
- Browser and device name spoofing
- User agent customization
- History tracking and management
- Bookmark system with folder organization
- Privacy and security foundations
- Domain lookalike detection (anti-phishing)
- Async/await architecture with Tokio
- Comprehensive logging system
- Utility functions for URL handling and formatting

### Fixed
- N/A

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Security
- N/A

## [0.1.0] - 2024-03-30

### Added
- Initial project structure and setup
- Core modules:
  - `app.rs` - Main application orchestrator
  - `browser.rs` - Core browser engine
  - `database.rs` - SQLite wrapper and schema
  - `permissions.rs` - Permission management
  - `extensions.rs` - Extension system
  - `settings.rs` - Settings management
  - `features.rs` - Unique browser features
  - `ui.rs` - UI foundation
  - `utils.rs` - Utility functions
- Comprehensive documentation:
  - README with full feature list
  - Architecture guide
  - API reference
  - Development guide
  - Extension guide
  - Quick start guide
- Build configuration (Cargo.toml)
- Example extension with popup and service worker
- Contributing guidelines
- MIT License
- .gitignore file

### Features Implemented
- **Browser Core**
  - Tab management (create, navigate, close)
  - Window management
  - History tracking
  - Bookmarks with folders
  - Async architecture

- **Permissions**
  - Camera permission
  - Microphone permission
  - Notifications
  - Geolocation
  - Clipboard read/write
  - Storage access
  - Payment information
  - Per-domain configuration

- **Settings**
  - Theme (Light/Dark/System)
  - Accent color
  - Font size and family
  - Compact mode
  - Browser name customization
  - Device name customization
  - User agent override
  - Privacy settings

- **Extensions**
  - Installation support
  - Enable/disable toggling
  - Uninstall capability
  - Manifest parsing
  - Service worker support
  - Permission management

- **Unique Features**
  - Browser name spoofing
  - Device name spoofing
  - Domain lookalike detection
  - User agent spoofing

- **Development**
  - Full documentation
  - Example extension
  - Quick start guide
  - Architecture guide
  - API reference

## Future Versions

### [0.2.0] - UI Foundation
- egui/Tauri UI implementation
- Web rendering engine integration
- Tab bar rendering
- Address bar with autocomplete
- Settings panel UI
- Permission dialogs

### [0.3.0] - Appearance & Theming
- Material Design 3 implementation
- Complete theme system
- Custom CSS support
- Live theme switching

### [0.4.0] - Privacy & Security
- Privacy dashboard
- Tracker blocking
- Cookie management
- Password manager
- Fingerprint protection

### [0.5.0] - Unique Features
- Smart Tab Grouping
- Session Snapshots
- Split View Browsing
- Smart Notes
- Keyboard Shortcuts Pro

### [0.6.0] - Sync & Cloud
- Cross-device sync
- Cloud backup
- End-to-end encryption
- Device management

### [0.7.0] - Performance
- Multi-process architecture
- GPU acceleration
- Memory optimization
- Network optimization

### [0.8.0] - Extension Store
- Extension marketplace
- Extension signing
- Automatic updates
- User reviews

### [1.0.0] - Production Ready
- Multi-language support
- Cross-platform optimization
- Full security audit
- Performance targets met

---

## Versioning Policy

### Semantic Versioning
- **MAJOR** (1.0.0): Breaking changes, major features
- **MINOR** (0.1.0): New features, new capabilities
- **PATCH** (0.0.1): Bug fixes, security patches

### Release Schedule
- Patch releases: Monthly
- Minor releases: Every 2-3 months
- Major releases: Annually

### Support Policy
- **Current**: Latest version receives all updates
- **LTS**: Every major version supported for 1 year
- **EOL**: Older versions no longer receive updates

## Changelog Guidelines

This changelog records:
- ✅ New features and capabilities
- ✅ Bug fixes and corrections
- ✅ Breaking changes
- ✅ Deprecations
- ✅ Security fixes
- ❌ Internal refactoring (unless significant)
- ❌ Test additions
- ❌ Documentation updates

## References

- [Keep a Changelog](https://keepachangelog.com/)
- [Semantic Versioning](https://semver.org/)
- [GitHub Releases](https://help.github.com/articles/creating-releases/)
