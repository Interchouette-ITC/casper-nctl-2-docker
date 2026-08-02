//! Host `./assets` inventory. Never returns secret keys.

use std::fs;
use std::path::Path;

use crate::paths::{assets_root, is_secret_filename, safe_under_assets, MAX_TEXT_BYTES};

fn dir_names(path: &Path, prefix: &str) -> Vec<String> {
    let Ok(rd) = fs::read_dir(path) else {
        return Vec::new();
    };
    let mut names: Vec<String> = rd
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.starts_with(prefix))
        .collect();
    names.sort();
    names
}

fn file_size(path: &Path) -> Option<u64> {
    fs::metadata(path).ok().map(|m| m.len())
}

pub fn assets_summary() -> String {
    let root = assets_root();
    let faucet = root.join("faucet");
    let users = root.join("users");
    let nodes = root.join("nodes");
    let chainspec = root.join("chainspec");
    let logs = root.join("logs");
    let mut lines = vec![
        format!("assets_root={}", root.display()),
        format!(
            "faucet_dir={} public_key_hex={}",
            faucet.is_dir(),
            faucet.join("public_key_hex").is_file()
        ),
        format!("users={} under {}", dir_names(&users, "user-").len(), users.display()),
        format!("nodes={} under {}", dir_names(&nodes, "node-").len(), nodes.display()),
        format!(
            "chainspec_dir={} files={}",
            chainspec.is_dir(),
            fs::read_dir(&chainspec).map(|rd| rd.count()).unwrap_or(0)
        ),
        format!("logs_dir={}", logs.is_dir()),
    ];
    if logs.is_dir() {
        if let Ok(rd) = fs::read_dir(&logs) {
            let mut files: Vec<_> = rd.filter_map(|e| e.ok()).collect();
            files.sort_by_key(|e| e.file_name());
            for e in files {
                let p = e.path();
                if p.is_file() {
                    lines.push(format!(
                        "  log {} size={}",
                        e.file_name().to_string_lossy(),
                        file_size(&p).unwrap_or(0)
                    ));
                }
            }
        }
    }
    lines.join("\n")
}

pub fn faucet_info() -> String {
    let faucet = assets_root().join("faucet");
    if !faucet.is_dir() {
        return format!("No faucet directory at {}", faucet.display());
    }
    let hex_path = faucet.join("public_key_hex");
    let pub_hex = fs::read_to_string(&hex_path)
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "(missing)".into());
    [
        format!("path={}", faucet.display()),
        format!("public_key_hex={pub_hex}"),
        format!("public_key.pem={}", faucet.join("public_key.pem").is_file()),
        "secret_key.pem present but never returned by MCP".into(),
        "CSPR transfers: not exposed in v1 (use nctl / casper-client manually)".into(),
    ]
    .join("\n")
}

pub fn list_nodes() -> String {
    let nodes = assets_root().join("nodes");
    let names = dir_names(&nodes, "node-");
    if names.is_empty() {
        return format!(
            "No node-* dirs under {} (network not set up yet?)",
            nodes.display()
        );
    }
    let mut lines = vec![format!("nodes under {}:", nodes.display()), String::new()];
    for name in names {
        let n = nodes.join(&name);
        lines.push(format!(
            "- {name}: keys={} logs={} storage={} config={}",
            n.join("keys").is_dir(),
            n.join("logs").is_dir(),
            n.join("storage").is_dir(),
            n.join("config").is_dir()
        ));
    }
    lines.join("\n")
}

pub fn list_users(include_public_hex: bool) -> String {
    let users = assets_root().join("users");
    let names = dir_names(&users, "user-");
    let mut lines = vec![format!("users under {}:", users.display()), String::new()];
    if names.is_empty() {
        lines.push("(none)".into());
    }
    for name in names {
        let mut line = format!("- {name}");
        if include_public_hex {
            let hx = users.join(&name).join("public_key_hex");
            if let Ok(s) = fs::read_to_string(&hx) {
                line.push_str(&format!("  {}", s.trim()));
            } else {
                line.push_str("  (no public_key_hex)");
            }
        }
        lines.push(line);
    }
    lines.push(String::new());
    lines.push("Faucet: use nctl_faucet_info".into());
    lines.join("\n")
}

pub fn read_public_key(identity: &str) -> String {
    let identity = identity.trim();
    let root = assets_root();
    let path = if identity == "faucet" {
        root.join("faucet").join("public_key_hex")
    } else if identity.starts_with("user-") {
        root.join("users").join(identity).join("public_key_hex")
    } else if identity.starts_with("node-") {
        let primary = root
            .join("nodes")
            .join(identity)
            .join("keys")
            .join("public_key_hex");
        if primary.is_file() {
            primary
        } else {
            root.join("nodes").join(identity).join("public_key_hex")
        }
    } else {
        return "identity must be 'faucet', 'user-N', or 'node-N'".into();
    };
    match fs::read_to_string(&path) {
        Ok(s) => format!("{identity}: {}", s.trim()),
        Err(_) => format!("public_key_hex not found: {}", path.display()),
    }
}

pub fn read_chainspec(relative: &str, max_bytes: usize) -> String {
    let root = assets_root();
    if relative == "chainspec" || relative == "chainspec/" {
        let d = root.join("chainspec");
        if !d.is_dir() {
            return format!("missing {}", d.display());
        }
        let mut files: Vec<String> = fs::read_dir(&d)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect();
        files.sort();
        let mut lines = vec![format!("{}:", d.display())];
        for n in files {
            lines.push(format!("- {n}"));
        }
        let accounts = root.join("users").join("accounts.toml");
        if accounts.is_file() {
            lines.push(format!(
                "also: {} size={}",
                accounts.display(),
                file_size(&accounts).unwrap_or(0)
            ));
        }
        return lines.join("\n");
    }

    let name = Path::new(relative)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    if is_secret_filename(name) {
        return "refusing to read secret key material".into();
    }

    let path = match safe_under_assets(relative) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if !path.is_file() {
        return format!("not a file: {}", path.display());
    }
    let data = fs::read(&path).unwrap_or_default();
    let take = max_bytes.clamp(1, MAX_TEXT_BYTES).min(data.len());
    let text = String::from_utf8_lossy(&data[..take]);
    format!("{} ({take} bytes shown):\n{text}", path.display())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::test_env::ENV_LOCK;
    use std::env;
    use std::path::Path;

    fn with_fake_repo<F: FnOnce(&Path)>(f: F) {
        let _g = ENV_LOCK.lock().unwrap();
        let dir = tempfile_dir();
        fs::create_dir_all(dir.join("assets/faucet")).unwrap();
        fs::write(dir.join("assets/faucet/public_key_hex"), "FAUCETHEX\n").unwrap();
        fs::write(dir.join("assets/faucet/secret_key.pem"), "SECRET\n").unwrap();
        fs::create_dir_all(dir.join("assets/users/user-1")).unwrap();
        fs::write(dir.join("assets/users/user-1/public_key_hex"), "USER1\n").unwrap();
        fs::create_dir_all(dir.join("assets/nodes/node-1/logs")).unwrap();
        fs::create_dir_all(dir.join("assets/chainspec")).unwrap();
        fs::write(dir.join("assets/chainspec/chainspec.toml"), "[protocol]\n").unwrap();
        fs::write(dir.join("docker-compose.yml"), "services: {}\n").unwrap();
        env::set_var("NCTL_DOCKER_ROOT", &dir);
        f(&dir);
        env::remove_var("NCTL_DOCKER_ROOT");
        let _ = fs::remove_dir_all(&dir);
    }

    fn tempfile_dir() -> std::path::PathBuf {
        let mut p = env::temp_dir();
        p.push(format!(
            "nctl-mcp-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn faucet_info_hides_secret() {
        with_fake_repo(|_| {
            let text = faucet_info();
            assert!(text.contains("FAUCETHEX"));
            assert!(!text.contains("SECRET"));
        });
    }

    #[test]
    fn refuses_secret_chainspec_path() {
        with_fake_repo(|_| {
            let text = read_chainspec("faucet/secret_key.pem", 100);
            assert!(text.to_lowercase().contains("refusing"));
        });
    }

    #[test]
    fn summary_counts() {
        with_fake_repo(|_| {
            let text = assets_summary();
            assert!(text.contains("users=1"));
            assert!(text.contains("nodes=1"));
        });
    }
}
