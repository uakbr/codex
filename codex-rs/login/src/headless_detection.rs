/// Detect if we're running in a headless or remote environment where
/// browser-based OAuth with localhost callback won't work.
///
/// This checks for common indicators of remote/headless environments:
/// - SSH sessions
/// - GitHub Codespaces
/// - VS Code Remote Containers
/// - Docker/container environments
/// - tmux/screen sessions
/// - Missing DISPLAY on Unix
pub fn is_headless_environment() -> bool {
    // SSH sessions - most reliable indicator
    if std::env::var("SSH_CONNECTION").is_ok()
        || std::env::var("SSH_CLIENT").is_ok()
        || std::env::var("SSH_TTY").is_ok()
    {
        return true;
    }

    // GitHub Codespaces
    if std::env::var("CODESPACE_NAME").is_ok() || std::env::var("CODESPACES").is_ok() {
        return true;
    }

    // VS Code Remote Containers / Remote Development
    if std::env::var("REMOTE_CONTAINERS").is_ok()
        || std::env::var("VSCODE_REMOTE_CONTAINERS_SESSION").is_ok()
    {
        return true;
    }

    // Docker container indicators
    if std::path::Path::new("/.dockerenv").exists() {
        return true;
    }

    // Check for container environment via cgroup
    if let Ok(cgroup) = std::fs::read_to_string("/proc/self/cgroup")
        && (cgroup.contains("/docker") || cgroup.contains("/lxc") || cgroup.contains("/kubepods")) {
            return true;
        }

    // tmux or screen sessions (often used in remote contexts)
    if let Ok(term) = std::env::var("TERM")
        && (term.contains("screen") || term.contains("tmux")) {
            // Only consider this headless if we also don't have DISPLAY
            #[cfg(not(windows))]
            if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
                return true;
            }
        }

    // Unix systems without DISPLAY or WAYLAND_DISPLAY (but not on macOS which doesn't use DISPLAY)
    #[cfg(all(unix, not(target_os = "macos")))]
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_ssh_connection() {
        temp_env::with_var("SSH_CONNECTION", Some("127.0.0.1"), || {
            assert!(is_headless_environment());
        });
    }

    #[test]
    fn detects_ssh_client() {
        temp_env::with_var("SSH_CLIENT", Some("127.0.0.1"), || {
            assert!(is_headless_environment());
        });
    }

    #[test]
    fn detects_ssh_tty() {
        temp_env::with_var("SSH_TTY", Some("/dev/pts/0"), || {
            assert!(is_headless_environment());
        });
    }

    #[test]
    fn detects_codespaces() {
        temp_env::with_var("CODESPACE_NAME", Some("my-codespace"), || {
            assert!(is_headless_environment());
        });
    }

    #[test]
    fn detects_vscode_remote_containers() {
        temp_env::with_var("REMOTE_CONTAINERS", Some("true"), || {
            assert!(is_headless_environment());
        });
    }
}
