#![allow(clippy::unwrap_used)]

use codex_login::is_headless_environment;

/// Test that various headless indicators are properly detected
#[test]
fn test_headless_detection_with_ssh_connection() {
    temp_env::with_var("SSH_CONNECTION", Some("127.0.0.1"), || {
        assert!(
            is_headless_environment(),
            "Should detect SSH_CONNECTION as headless"
        );
    });
}

#[test]
fn test_headless_detection_with_ssh_client() {
    temp_env::with_var("SSH_CLIENT", Some("127.0.0.1"), || {
        assert!(
            is_headless_environment(),
            "Should detect SSH_CLIENT as headless"
        );
    });
}

#[test]
fn test_headless_detection_with_ssh_tty() {
    temp_env::with_var("SSH_TTY", Some("/dev/pts/0"), || {
        assert!(
            is_headless_environment(),
            "Should detect SSH_TTY as headless"
        );
    });
}

#[test]
fn test_headless_detection_with_codespace_name() {
    temp_env::with_var("CODESPACE_NAME", Some("test-codespace"), || {
        assert!(
            is_headless_environment(),
            "Should detect CODESPACE_NAME as headless"
        );
    });
}

#[test]
fn test_headless_detection_with_codespaces() {
    temp_env::with_var("CODESPACES", Some("true"), || {
        assert!(
            is_headless_environment(),
            "Should detect CODESPACES as headless"
        );
    });
}

#[test]
fn test_headless_detection_with_remote_containers() {
    temp_env::with_var("REMOTE_CONTAINERS", Some("true"), || {
        assert!(
            is_headless_environment(),
            "Should detect REMOTE_CONTAINERS as headless"
        );
    });
}

#[test]
fn test_headless_detection_with_vscode_remote() {
    temp_env::with_var("VSCODE_REMOTE_CONTAINERS_SESSION", Some("true"), || {
        assert!(
            is_headless_environment(),
            "Should detect VSCODE_REMOTE_CONTAINERS_SESSION as headless"
        );
    });
}

#[test]
fn test_dockerenv_detection() {
    // This test can only verify the logic works, not that it detects Docker
    // since we'd need to actually be in a Docker container
    let exists = std::path::Path::new("/.dockerenv").exists();
    if exists {
        assert!(
            is_headless_environment(),
            "Should detect Docker container as headless"
        );
    }
    // If not in Docker, this doesn't prove anything, but the test doesn't fail
}

#[test]
#[cfg(all(unix, not(target_os = "macos")))]
fn test_unix_no_display_detection() {
    // Test with no DISPLAY and no WAYLAND_DISPLAY on non-macOS Unix
    temp_env::with_vars(
        [
            ("SSH_CONNECTION", None::<&str>),
            ("SSH_CLIENT", None::<&str>),
            ("SSH_TTY", None::<&str>),
            ("DISPLAY", None::<&str>),
            ("WAYLAND_DISPLAY", None::<&str>),
        ],
        || {
            assert!(
                is_headless_environment(),
                "Unix without DISPLAY should be headless"
            );
        },
    );
}

#[test]
#[cfg(all(unix, not(target_os = "macos")))]
fn test_unix_with_display_not_headless() {
    // Test that having DISPLAY on Unix makes it not headless (unless other indicators present)
    temp_env::with_vars(
        [
            ("SSH_CONNECTION", None::<&str>),
            ("SSH_CLIENT", None::<&str>),
            ("SSH_TTY", None::<&str>),
            ("CODESPACE_NAME", None::<&str>),
            ("REMOTE_CONTAINERS", None::<&str>),
            ("DISPLAY", Some(":0")),
        ],
        || {
            assert!(
                !is_headless_environment(),
                "Unix with DISPLAY and no other indicators should not be headless"
            );
        },
    );
}

