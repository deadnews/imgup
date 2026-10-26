use std::path::PathBuf;

use anyhow::{Context, Result};
use tracing::warn;

/// Get the default config path.
pub fn get_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("imgup/.env"))
}

/// Get a required environment variable, with a helpful error message.
pub fn get_env(name: &str) -> Result<String> {
    std::env::var(name).with_context(|| {
        let hint = get_config_path()
            .map(|p| format!(" or in '{}'", p.display()))
            .unwrap_or_default();
        format!("please set {name} in environment variables{hint}")
    })
}

/// Convert bytes to human-readable format (e.g., "1.2 MiB").
#[expect(clippy::cast_precision_loss)]
pub fn human_size(bytes: usize) -> String {
    const UNITS: &[&str] = &["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi"];
    const THRESHOLD: f64 = 1024.0;

    let mut num = bytes as f64;
    for unit in UNITS {
        if num < THRESHOLD {
            return format!("{num:.1} {unit}B");
        }
        num /= THRESHOLD;
    }
    format!("{num:.1} YiB")
}

/// Copy text to clipboard.
pub fn clipboard_copy(text: &str) {
    if let Err(e) = arboard::Clipboard::new().and_then(|mut cb| cb.set_text(text)) {
        warn!("failed to copy to clipboard: {e}");
    }
}

/// Send a desktop notification.
pub fn notify_send(text: &str) {
    if let Err(e) = notify_rust::Notification::new()
        .appname("imgup")
        .summary("Upload complete")
        .body(text)
        .show()
    {
        warn!("failed to send notification: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_size_bytes() {
        assert_eq!(human_size(0), "0.0 B");
        assert_eq!(human_size(100), "100.0 B");
    }

    #[test]
    fn test_human_size_kib() {
        assert_eq!(human_size(1536), "1.5 KiB");
    }

    #[test]
    fn test_human_size_mib() {
        assert_eq!(human_size(1_234_567), "1.2 MiB");
    }

    #[test]
    fn test_human_size_gib() {
        assert_eq!(human_size(2_147_483_648), "2.0 GiB");
    }

    #[test]
    fn test_get_env_missing() {
        let msg = get_env("NONEXISTENT_VAR_12345")
            .expect_err("should be an error")
            .to_string();
        assert!(msg.contains("NONEXISTENT_VAR_12345"), "{msg}");
    }

    #[test]
    fn test_get_env_present() {
        let var = if cfg!(windows) { "USERNAME" } else { "HOME" };
        let result = get_env(var);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_path_structure() {
        if let Some(path) = get_config_path() {
            let path_str = path.to_string_lossy();
            assert!(path_str.contains("imgup"), "{path_str}");
            assert!(path_str.ends_with(".env"), "{path_str}");
        }
    }
}
