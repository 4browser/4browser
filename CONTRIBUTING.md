# 🤝 Contributing to 4 Browser

Thank you for your interest in contributing to 4 Browser! We welcome contributions of all kinds.

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project, you agree to abide by its terms.

## Ways to Contribute

### 🐛 Report Bugs
- Check if bug already exists in [issues](https://github.com/4browser/4browser/issues)
- Provide clear description, steps to reproduce, and expected behavior
- Include system information (OS, Rust version, browser version)
- If possible, provide a minimal test case

### ✨ Suggest Features
- Open an issue with label `enhancement`
- Describe the feature and why it's useful
- Provide use cases and examples
- Be open to discussion and feedback

### 📝 Improve Documentation
- Fix typos and grammar
- Clarify confusing sections
- Add examples and use cases
- Improve code comments

### 🧪 Test & QA
- Test features on different platforms
- Report compatibility issues
- Verify bug fixes
- Test extensions and permissions

### 💻 Write Code

## Development Setup

### Prerequisites
- Rust 1.70+ ([Install](https://rustup.rs/))
- Git
- Text editor (VS Code recommended)

### Setup Steps
```bash
# Clone repository
git clone https://github.com/4browser/4browser.git
cd 4browser

# Create feature branch
git checkout -b feature/your-feature

# Make changes
# ... (edit files)

# Build and test
cargo build
cargo test
cargo clippy
cargo fmt
```

## Coding Standards

### Rust Best Practices
```rust
// ✅ Good: Clear, documented code
/// Creates a new  window in the browser.
/// 
/// # Arguments
/// * `title` - Window title
/// 
/// # Errors
/// Returns error if window creation fails
pub async fn create_window(title: &str) -> Result<String> {
    info!("Creating window: {}", title);
    // Implementation
}

// ❌ Bad: Undocumented, unclear
pub async fn cw(t: &str) -> Result<String> {
    // ...
}
```

### Naming Conventions
- `snake_case` for functions and variables
- `PascalCase` for types and traits
- `SCREAMING_SNAKE_CASE` for constants
- Prefix types with what they are: `UserAgent`, `PermissionState`

### Error Handling
```rust
// ✅ Use Result<T> for recoverable errors
pub async fn load_settings() -> Result<Settings>

// ✅ Use anyhow for context
return Err(anyhow!("Failed to load settings"));

// ✅ Use log for debugging
info!("Setting loaded successfully");
debug!("Setting details: {:?}", settings);
```

### Async Patterns
```rust
// ✅ Use async/await
pub async fn process_user(id: &str) -> Result<()> {
    let user = load_user(id).await?;
    save_cache(&user).await?;
    Ok(())
}

// ✅ Use Arc<RwLock<T>> for shared state
pub struct Manager {
    data: Arc<RwLock<HashMap<String, Data>>>,
}

// Don't make unnecessary functions async
fn get_config_path() -> PathBuf {  // Not async
    dirs::config_dir().unwrap_or_default()
}
```

### Documentation
```rust
/// Brief description (shown in hover).
///
/// Longer description explaining what this does
/// and when to use it. Can span multiple lines.
///
/// # Arguments
/// * `name` - What this argument does
/// * `value` - What this argument does
///
/// # Returns
/// Returns a description of what is returned.
///
/// # Errors
/// Explains what errors can be returned.
///
/// # Example
/// ```
/// let result = my_function("example", 42)?;
/// println!("{:?}", result);
/// ```
pub fn my_function(name: &str, value: u32) -> Result<String> {
}
```

## Testing

### Write Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://google.com"));
        assert!(!is_valid_url("not a url"));
    }

    #[tokio::test]
    async fn test_load_settings() {
        let settings = Settings::load(&test_dir()).await.unwrap();
        assert_eq!(settings.browser_name, "4 Browser");
    }
}
```

### Run Tests
```bash
# All tests
cargo test

# Specific module
cargo test permissions::

# With output
cargo test -- --nocapture

# Single test
cargo test test_is_valid_url
```

## Commit Guidelines

### Commit Messages
Follow conventional commits:

```
type(scope): subject

body (optional)

footer (optional)
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `test`: Test changes
- `chore`: Dependency updates, etc.

**Examples:**
```
feat(permissions): add clipboard_read permission
fix(browser): resolve memory leak in tab manager
docs: update API reference
refactor(storage): simplify database queries
```

### Example Commit
```bash
git commit -m "feat(extensions): add extension enable/disable feature

- Add enable_extension method to ExtensionManager
- Add disable_extension method to ExtensionManager
- Update settings UI to show toggle button
- Add tests for enable/disable functionality

Fixes #123"
```

## Pull Request Process

### Before Submitting
- [ ] Code builds without errors: `cargo build`
- [ ] All tests pass: `cargo test`
- [ ] Clippy lint passes: `cargo clippy`
- [ ] Code formatted: `cargo fmt`
- [ ] Commits are clean and meaningful
- [ ] Branch is up to date with develop

### Creating PR
1. Fork the repository
2. Create feature branch: `git checkout -b feature/description`
3. Make changes and commit
4. Push to your fork: `git push origin feature/description`
5. Open pull request from your fork to `4browser/4browser:develop`

### PR Template
```markdown
## Description
Clear description of changes.

## Type of Change
- [ ] Bug fix (fixes issue #...)
- [ ] New feature (closes issue #...)
- [ ] Documentation update
- [ ] Refactoring

## Testing
Describe tests added/modified:
- [ ] Unit tests
- [ ] Integration tests
- [ ] Manual testing

## Checklist
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] Changelog updated
- [ ] No breaking changes
```

### Review Process
1. Automated checks must pass
2. Code review by maintainers
3. Any requested changes made
4. Approval from at least 2 maintainers
5. Merge to develop branch

## Release Process

### Version Numbering
Follow [Semantic Versioning](semver.org):
- MAJOR.MINOR.PATCH (e.g., 1.2.3)
- MAJOR: Breaking changes
- MINOR: New features
- PATCH: Bug fixes

### Release Steps
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release branch: `git checkout -b release/1.0.0`
4. Make release commit: `git commit -m "chore: release 1.0.0"`
5. Tag release: `git tag v1.0.0`
6. Create GitHub release
7. Merge to main and develop

## Help & Community

### Getting Help
- **Questions**: [GitHub Discussions](https://github.com/4browser/4browser/discussions)
- **Chat**: [Discord Server](https://discord.gg/4browser)
- **Issues**: [GitHub Issues](https://github.com/4browser/4browser/issues)

### Community Resources
- [Architecture Guide](ARCHITECTURE.md)
- [API Reference](API.md)
- [Development Guide](DEVELOPMENT.md)
- [Extensions Guide](EXTENSIONS.md)

## Contributor Recognition

We appreciate all contributions! Contributors will be:
- Listed in [CONTRIBUTORS.md](CONTRIBUTORS.md)
- Mentioned in release notes
- Given credit in commits

## Questions?

Don't hesitate to ask:
- Comment on issues
- Open a discussion
- Join Discord
- Email: contributing@4browser.com

---

**Thank you for contributing to 4 Browser! 🙏**

Together, we're building a better browser. 🌐
