//! Repo root and safe assets path helpers.

use std::env;
use std::path::{Component, Path, PathBuf};

pub const PROFILES: &[&str] = &["stable", "2.2", "2.1", "2.0", "1.5.8", "dev"];
pub const DEFAULT_MCP_ADDR: &str = "0.0.0.0:8788";
pub const DEFAULT_MCP_URL: &str = "http://127.0.0.1:8788/mcp";
pub const MAX_TEXT_BYTES: usize = 96 * 1024;

pub fn repo_root() -> PathBuf {
    if let Ok(env_root) = env::var("NCTL_DOCKER_ROOT") {
        return PathBuf::from(env_root);
    }
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.join("docker-compose.yml").is_file() {
        return cwd;
    }
    // mcp/ is one level under repo when running via cargo from mcp/
    if cwd.join("../docker-compose.yml").is_file() {
        return cwd
            .join("..")
            .canonicalize()
            .unwrap_or_else(|_| cwd.join(".."));
    }
    cwd
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
        fs::write(dir.join("docker-compose.yml"), "services: {}\n").unwrap();
        env::set_var("NCTL_DOCKER_ROOT", &dir);
        assert!(safe_under_assets("../docker-compose.yml").is_err());
        env::remove_var("NCTL_DOCKER_ROOT");
        let _ = fs::remove_dir_all(&dir);
    }
}
