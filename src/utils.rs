use regex::Regex;
use url::Url;

pub fn is_valid_url(url_str: &str) -> bool {
    Url::parse(url_str).is_ok()
}

pub fn extract_domain(url_str: &str) -> Option<String> {
    match Url::parse(url_str) {
        Ok(url) => url.domain().map(|d| d.to_string()),
        Err(_) => None,
    }
}

pub fn is_special_url(url: &str) -> bool {
    url.starts_with("about:")
        || url.starts_with("chrome://")
        || url.starts_with("data:")
        || url.starts_with("javascript:")
}

pub fn sanitize_filename(filename: &str) -> String {
    let re = Regex::new(r"[<>:\"/\\|?*]").unwrap();
    re.replace_all(filename, "_").to_string()
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

pub fn format_duration_ms(ms: u64) -> String {
    let seconds = ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{} days", days)
    } else if hours > 0 {
        format!("{} hours", hours)
    } else if minutes > 0 {
        format!("{} minutes", minutes)
    } else {
        format!("{} seconds", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://www.google.com"));
        assert!(is_valid_url("http://localhost:8080"));
        assert!(!is_valid_url("not a url"));
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("https://www.google.com/search"),
            Some("www.google.com".to_string())
        );
        assert_eq!(extract_domain("invalid"), None);
    }

    #[test]
    fn test_is_special_url() {
        assert!(is_special_url("about:home"));
        assert!(is_special_url("chrome://settings"));
        assert!(!is_special_url("https://google.com"));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
    }
}
