//! Shared fake repo under `NCTL_DOCKER_ROOT` for integration tests.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

pub struct FakeRepo {
    pub root: PathBuf,
    _guard: std::sync::MutexGuard<'static, ()>,
}

impl FakeRepo {
    pub fn new() -> Self {
        let guard = env_lock().lock().unwrap_or_else(|e| e.into_inner());
        let mut root = env::temp_dir();
        root.push(format!(
            "nctl-mcp-fixture-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("assets/faucet")).unwrap();
        fs::create_dir_all(root.join("assets/users/user-1")).unwrap();
        fs::create_dir_all(root.join("assets/users/user-2")).unwrap();
        fs::create_dir_all(root.join("assets/nodes/node-1/logs")).unwrap();
        fs::create_dir_all(root.join("assets/nodes/node-1/keys")).unwrap();
        fs::create_dir_all(root.join("assets/nodes/node-2/logs")).unwrap();
        fs::create_dir_all(root.join("assets/chainspec")).unwrap();
        fs::create_dir_all(root.join("assets/logs")).unwrap();

        fs::write(root.join("docker-compose.yml"), "services: {}\n").unwrap();
        fs::write(root.join("assets/faucet/public_key_hex"), "FAUCETHEXDEADBEEF\n").unwrap();
        fs::write(root.join("assets/faucet/secret_key.pem"), "TOPSECRET\n").unwrap();
        fs::write(root.join("assets/faucet/public_key.pem"), "PUBPEM\n").unwrap();
        fs::write(root.join("assets/users/user-1/public_key_hex"), "USER1HEX\n").unwrap();
        fs::write(root.join("assets/users/user-2/public_key_hex"), "USER2HEX\n").unwrap();
        fs::write(
            root.join("assets/nodes/node-1/keys/public_key_hex"),
            "NODE1HEX\n",
        )
        .unwrap();
        fs::write(
            root.join("assets/nodes/node-1/logs/stderr.log"),
            "ok\nERROR boom node-1\nbye\n",
        )
        .unwrap();
        fs::write(
            root.join("assets/logs/stdout.log"),
            "hello\nERROR boom assets\n",
        )
        .unwrap();
        fs::write(
            root.join("assets/logs/sidecar-stdout.log"),
            "sidecar line\n",
        )
        .unwrap();
        fs::write(root.join("assets/chainspec/chainspec.toml"), "[protocol]\n").unwrap();
        fs::write(root.join("assets/users/accounts.toml"), "[[accounts]]\n").unwrap();

        env::set_var("NCTL_DOCKER_ROOT", &root);
        Self {
            root,
            _guard: guard,
        }
    }
}

impl Drop for FakeRepo {
    fn drop(&mut self) {
        env::remove_var("NCTL_DOCKER_ROOT");
        let _ = fs::remove_dir_all(&self.root);
    }
}
