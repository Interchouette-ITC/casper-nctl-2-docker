//! Repo root and safe assets path helpers.

use std::env;
use std::path::{Component, Path, PathBuf};

pub const PROFILES: &[&str] = &["stable", "2.2", "2.1", "2.0", "1.5.8", "dev"];
pub const DEFAULT_MCP_ADDR: &str = "0.0.0.0:8790";
pub const DEFAULT_MCP_URL: &str = "http://127.0.0.1:8790/mcp";
pub const MAX_TEXT_BYTES: usize = 96 * 1024;

pub fn repo_root() -> PathBuf {
    if let Ok(env_root) = env::var("NCTL_DOCKER_ROOT") {
        return PathBuf::from(env_root);
    }
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.join("docker/docker-compose.yml").is_file() {
        return cwd;
    }
    // mcp/ is one level under repo when running via cargo from mcp/
    if cwd.join("../docker/docker-compose.yml").is_file() {
        return cwd
            .join("..")
            .canonicalize()
            .unwrap_or_else(|_| cwd.join(".."));
    }
    cwd
}

/// Absolute path on the **Docker host** for `-v` bind sources.
///
/// MCP runs with the repo bind-mounted at `/workspace` and talks to the host
/// Docker socket. Paths inside the MCP container (`NCTL_DOCKER_ROOT=/workspace`)
/// must **not** be passed as `-v` sources — the daemon would create host
/// `/workspace` on the root disk. Set `NCTL_HOST_ROOT` to the host clone path
/// (launch scripts / compose do this). When unset (host-native MCP), falls back
/// to [`repo_root`].
pub fn host_repo_root() -> PathBuf {
    if let Ok(host) = env::var("NCTL_HOST_ROOT") {
        let host = host.trim();
        if !host.is_empty() {
            return PathBuf::from(host);
        }
    }
    repo_root()
}

/// MCP container layout: in-container root is `/workspace` (host clone bind).
pub fn mcp_uses_workspace_mount() -> bool {
    matches!(
        env::var("NCTL_DOCKER_ROOT").ok().as_deref().map(str::trim),
        Some("/workspace")
    )
}

/// True when Docker bind sources must not be used (would hit host `/`, `/workspace`, etc.).
///
/// This is a hard refuse for lifecycle tools — not a soft default. In the MCP
/// container (`NCTL_DOCKER_ROOT=/workspace`), `NCTL_HOST_ROOT` is **required**
/// and must be an absolute host path outside `/workspace` and not `/`.
pub fn host_bind_root_is_unsafe() -> bool {
    if mcp_uses_workspace_mount() {
        match env::var("NCTL_HOST_ROOT") {
            Ok(h) if !h.trim().is_empty() => {}
            _ => return true,
        }
    }

    let host = host_repo_root();
    if !host.is_absolute() {
        return true;
    }
    if host == Path::new("/") {
        return true;
    }
    if host == Path::new("/workspace") || host.starts_with("/workspace/") {
        return true;
    }
    false
}

/// Error text when refusing Hub/compose starts that would trash the host disk.
pub fn unsafe_host_bind_message(tool: &str) -> String {
    format!(
        "REFUSING {tool}: unsafe Docker bind root (would write under host /workspace or /). \
NCTL_HOST_ROOT is required when NCTL_DOCKER_ROOT=/workspace and must be an absolute \
host clone path (e.g. /opt2/casper/casper-nctl-2-docker) — never /workspace and never /. \
Compose/Cursor launch scripts must pass -e NCTL_HOST_ROOT=<host-path>. Prefer host \
`make start` with PWD on a large disk. Note: docker.sock access is still host-root \
equivalent; this guard only blocks known footgun bind sources in MCP lifecycle tools."
    )
}

pub fn assets_root() -> PathBuf {
    repo_root().join("assets")
}

pub fn container_name_for_profile(profile: &str) -> String {
    format!("casper-nctl-2-docker-{profile}")
}

/// Resolve `relative` under `assets/`; reject absolute paths and `..` traversal.
pub fn safe_under_assets(relative: &str) -> Result<PathBuf, String> {
    let root = assets_root()
        .canonicalize()
        .unwrap_or_else(|_| assets_root());
    let rel = Path::new(relative);
    if rel.is_absolute() {
        return Err("path must be relative to assets/".into());
    }
    for c in rel.components() {
        if matches!(c, Component::ParentDir) {
            return Err(format!("path escapes assets/: {relative}"));
        }
    }
    let target = root.join(rel);
    let canon = target.canonicalize().unwrap_or(target);
    if !canon.starts_with(&root) {
        return Err(format!("path escapes assets/: {relative}"));
    }
    Ok(canon)
}

pub fn is_secret_filename(name: &str) -> bool {
    name == "secret_key.pem" || name == "secret_key"
}

#[cfg(test)]
pub mod test_env {
    use std::sync::Mutex;
    /// Serialize tests that mutate `NCTL_DOCKER_ROOT`.
    pub static ENV_LOCK: Mutex<()> = Mutex::new(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::test_env::ENV_LOCK;
    use std::fs;

    #[test]
    fn rejects_parent_dir_escape() {
        let _g = ENV_LOCK.lock().unwrap();
        let mut dir = env::temp_dir();
        dir.push(format!("nctl-mcp-path-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("assets")).unwrap();
        fs::create_dir_all(dir.join("docker")).unwrap();
        fs::write(dir.join("docker/docker-compose.yml"), "services: {}\n").unwrap();
        env::set_var("NCTL_DOCKER_ROOT", &dir);
        assert!(safe_under_assets("../docker-compose.yml").is_err());
        env::remove_var("NCTL_DOCKER_ROOT");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn host_repo_root_prefers_nctl_host_root() {
        let _g = ENV_LOCK.lock().unwrap();
        env::set_var("NCTL_DOCKER_ROOT", "/workspace");
        env::set_var("NCTL_HOST_ROOT", "/opt2/casper/casper-nctl-2-docker");
        assert_eq!(
            host_repo_root(),
            PathBuf::from("/opt2/casper/casper-nctl-2-docker")
        );
        assert!(!host_bind_root_is_unsafe());
        env::remove_var("NCTL_HOST_ROOT");
        assert!(host_bind_root_is_unsafe());
        env::remove_var("NCTL_DOCKER_ROOT");
    }

    #[test]
    fn host_bind_refuses_root_and_relative() {
        let _g = ENV_LOCK.lock().unwrap();
        env::remove_var("NCTL_DOCKER_ROOT");
        env::set_var("NCTL_HOST_ROOT", "/");
        assert!(host_bind_root_is_unsafe());
        env::set_var("NCTL_HOST_ROOT", "relative/path");
        assert!(host_bind_root_is_unsafe());
        env::set_var("NCTL_HOST_ROOT", "/workspace");
        assert!(host_bind_root_is_unsafe());
        env::set_var("NCTL_HOST_ROOT", "/opt2/casper/casper-nctl-2-docker");
        assert!(!host_bind_root_is_unsafe());
        env::remove_var("NCTL_HOST_ROOT");
    }
}
