# 🚀 Development Guide

## Setting Up Development Environment

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Cargo (comes with Rust)
- SQLite3 development libraries (optional, bundled version used by default)
- Git

### Platform-Specific Setup

#### Linux (Ubuntu/Debian)
```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential libssl-dev pkg-config

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### macOS
```bash
# Install Xcode Command Line Tools (if not already installed)
xcode-select --install

# Use Homebrew for additional tools
brew install rust sqlite3
```

#### Windows
```powershell
# Install Visual Studio Build Tools or Visual Studio Community
# Then install Rust from https://rustup.rs/

# Or use Chocolatey
choco install rust sqlite
```

## Building the Project

### Development Build
```bash
# Debug build with full logging
cargo build

# Run with debug logging
RUST_LOG=debug cargo run
```

### Release Build
```bash
# Optimized release build
cargo build --release

# Run release version
./target/release/4browser
```

### Build with Specific Features (Future)
```bash
# Build with optional features
cargo build --release --features "cloud-sync,ai-features"
```

## Development Workflow

### Running Tests
```bash
# Run all tests
cargo test

# Run tests for specific module
cargo test permissions::

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_is_valid_url
```

### Code Quality

#### Format Code
```bash
# Auto-format code
cargo fmt

# Check formatting
cargo fmt --check
```

#### Lint with Clippy
```bash
# Run Clippy linter
cargo clippy

# Fix lint warnings automatically
cargo clippy --fix
```

#### Check Without Building
```bash
# Fast syntax/type checking
cargo check

# Fix errors automatically where possible
cargo fix
```

### Debugging

#### Using LLDB (Linux/macOS)
```bash
# Install LLDB
cargo install cargo-lldb

# Run with debugger
cargo lldb

# Then at (lldb) prompt:
# b main.rs:123    - Set breakpoint at line 123
# r               - Run
# c               - Continue
# n               - Next
# s               - Step
# p variable_name - Print variable
# bt              - Backtrace
```

#### Using GDB (Linux)
```bash
# Build with debug symbols
cargo build

# Run with GDB
gdb ./target/debug/4browser

# Then at (gdb) prompt:
# break main.rs:123  - Set breakpoint
# run                - Run program
# continue           - Continue
# next               - Next line
# step               - Step into
# print variable     - Print variable
# backtrace          - Print backtrace
```

#### Environment Variables for Debugging
```bash
# Verbose logging
RUST_LOG=trace cargo run

# Backtrace on panic
RUST_BACKTRACE=1 cargo run

# Full backtrace
RUST_BACKTRACE=full cargo run

# Specific module logging
RUST_LOG=4browser::extensions=debug cargo run
```

## Project Structure

```
4browser/
├── src/
│   ├── main.rs              # Entry point and initialization
│   ├── app.rs              # Main application logic
│   ├── browser.rs          # Core browser engine
│   ├── database.rs         # SQLite database layer
│   ├── permissions.rs      # Permission management
│   ├── extensions.rs       # Extension system
│   ├── settings.rs         # Settings management
│   ├── features.rs         # Unique browser features
│   ├── ui.rs              # User interface layer
│   └── utils.rs           # Utility functions
├── target/
│   ├── debug/             # Debug builds
│   └── release/           # Release builds
├── Cargo.toml             # Project manifest
├── Cargo.lock             # Dependency lock file
├── README.md              # User documentation
├── ARCHITECTURE.md        # Architecture documentation
├── API.md                 # API reference
├── DEVELOPMENT.md         # This file
└── .gitignore             # Git ignore rules
```

## Creating a New Module

### 1. Create the Module File
```bash
touch src/my_module.rs
```

### 2. Add to main.rs
```rust
mod my_module;
```

### 3. Write Module Code
```rust
// src/my_module.rs

use anyhow::Result;
use log::info;

pub struct MyComponent {
    // Fields
}

impl MyComponent {
    pub async fn new() -> Result<Self> {
        info!("Initializing MyComponent");
        Ok(Self {
            // Initialize fields
        })
    }

    pub async fn do_something(&self) -> Result<()> {
        info!("Doing something");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_something() {
        // Test code
    }
}
```

## Adding Dependencies

### Add to Cargo.toml
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

### Update Dependencies
```bash
# Update all dependencies to latest compatible versions
cargo update

# Add new dependency
cargo add serde

# Add specific version
cargo add serde@1.0

# Add with features
cargo add serde --features "derive"
```

### Managing Dependency Features
Common patterns:
```toml
# Optional feature group
tokio = { version = "1.0", features = ["full"] }

# Specific features
serde = { version = "1.0", features = ["derive", "serde_json"] }

# Development only
[dev-dependencies]
mockall = "0.11"
```

## Documentation

### Writing Rust Docs
```rust
/// Creates a new window in the browser.
///
/// # Arguments
///
/// * `title` - The window title
///
/// # Returns
///
/// Returns the window ID if successful
///
/// # Example
///
/// ```ignore
/// let window_id = create_window("My Browser").await?;
/// ```
pub async fn create_window(title: &str) -> Result<String> {
    // Implementation
}
```

### Generate and View Documentation
```bash
# Generate HTML documentation
cargo doc

# Generate and open in browser
cargo doc --open

# Include private items in documentation
cargo doc --document-private-items
```

## Performance Profiling

### Using flamegraph
```bash
# Install flamegraph
cargo install flamegraph

# Run with profiling
cargo flamegraph

# View result
open flamegraph.svg
```

### Memory Profiling
```bash
# Using Valgrind (Linux)
valgrind --leak-check=full ./target/debug/4browser

# Using heaptrack (Linux)
heaptrack ./target/debug/4browser
```

## Publishing Builds

### Creating Release Binary
```bash
# Build for current platform
cargo build --release

# Binary location:
# Linux/macOS: ./target/release/4browser
# Windows: ./target/release/4browser.exe
```

### Cross-Platform Compilation
```bash
# Install cross
cargo install cross

# Compile for Linux from macOS
cross build --target x86_64-unknown-linux-gnu --release

# Compile for Windows from Linux
cross build --target x86_64-pc-windows-gnu --release

# Compile for macOS from Linux
# (Note: requires macOS to actually build)
```

## Continuous Integration Setup

### GitHub Actions Example
```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --verbose
      - run: cargo clippy -- -D warnings
```

## Troubleshooting Build Issues

### Common Issues

#### "error: linker 'cc' not found"
```bash
# Linux: Install build tools
sudo apt-get install build-essential

# macOS: Install Xcode Command Line Tools
xcode-select --install

# Windows: Install Visual Studio Build Tools
```

#### "cannot find -lssl"
```bash
# Linux: Install OpenSSL dev
sudo apt-get install libssl-dev pkg-config

# macOS: Install OpenSSL with Homebrew
brew install openssl
export LDFLAGS="-L/usr/local/opt/openssl/lib"
export CPPFLAGS="-I/usr/local/opt/openssl/include"
```

#### Slow compilation
```bash
# Use sccache for faster rebuilds
cargo install sccache
export RUSTC_WRAPPER=sccache

# Or use mold linker (much faster)
cargo install mold
# Then add to .cargo/config.toml:
# [build]
# rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

## Git Workflow

### Branch Naming
```
feature/feature-name
bugfix/bug-description
refactor/what-is-being-refactored
docs/documentation-update
```

### Commit Messages
```
feat: Add new camera permission UI
fix: Resolve memory leak in extension manager
docs: Update API documentation
refactor: Simplify permission checking logic
test: Add tests for utility functions
```

## Release Checklist

- [ ] Update version in Cargo.toml
- [ ] Update CHANGELOG.md
- [ ] Run full test suite
- [ ] Build release binaries for all platforms
- [ ] Tag release in git: `git tag v0.1.0`
- [ ] Create GitHub release
- [ ] Announce on social media

## Contributing

1. Fork repository
2. Create feature branch: `git checkout -b feature/my-feature`
3. Make changes and commit: `git commit -am 'Add my feature'`
4. Push to branch: `git push origin feature/my-feature`
5. Open pull request

Refer to [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.
