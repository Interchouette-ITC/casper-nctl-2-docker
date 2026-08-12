//! Docker / Make lifecycle helpers (parity with Makefile targets).

use std::process::Command;

use crate::assets;
use crate::paths::{
    container_name_for_profile, host_bind_root_is_unsafe, host_repo_root, repo_root,
    unsafe_host_bind_message, DEFAULT_MCP_URL, PROFILES,
};

fn run(cmd: &str, args: &[&str]) -> (i32, String, String) {
    let host = host_repo_root();
    let host_s = host.to_string_lossy();
    // cwd stays NCTL_DOCKER_ROOT (/workspace in MCP) so Makefile/compose files resolve;
    // NCTL_HOST_ROOT + PWD must be the *host* path so Docker bind sources are correct.
    match Command::new(cmd)
        .args(args)
        .current_dir(repo_root())
        .env("NCTL_HOST_ROOT", host_s.as_ref())
        .env("PWD", host_s.as_ref())
        .output()
    {
        Ok(out) => (
            out.status.code().unwrap_or(1),
            String::from_utf8_lossy(&out.stdout).into_owned(),
            String::from_utf8_lossy(&out.stderr).into_owned(),
        ),
        Err(e) => (127, String::new(), e.to_string()),
    }
}

fn refuse_unsafe_host_binds(tool: &str) -> Option<String> {
    if host_bind_root_is_unsafe() {
        Some(unsafe_host_bind_message(tool))
    } else {
        None
    }
}

fn require_profile(profile: &str) -> Result<(), String> {
    if PROFILES.contains(&profile) {
        Ok(())
    } else {
        Err(format!(
            "Unknown profile {profile:?}. Choose one of: {}",
            PROFILES.join(", ")
        ))
    }
}

fn make_profile(target: &str, profile: &str) -> String {
    if let Err(e) = require_profile(profile) {
        return e;
    }
    let (code, out, err) = run("make", &[target, profile]);
    if code != 0 {
        return format!(
            "make {target} {profile} failed ({code}):\n{}",
            if err.is_empty() { out } else { err }
        );
    }
    format!("make {target} {profile} ok.\n{out}{err}")
        .trim()
        .to_string()
}

fn docker_logs_tail(profile: &str, lines: u32) -> String {
    let name = container_name_for_profile(profile);
    let (code, out, err) = run("docker", &["logs", "--tail", &lines.to_string(), &name]);
    let text = if out.is_empty() {
        err
    } else {
        format!("{out}{err}")
    };
    if code != 0 && text.trim().is_empty() {
        return format!("(no docker logs yet for {name})");
    }
    format!("--- docker logs {name} (last {lines}) ---\n{text}")
}

pub fn list_profiles() -> String {
    let mut lines = vec![
        "Available compose profiles / image tags:".into(),
        String::new(),
    ];
    for p in PROFILES {
        lines.push(format!("- {p}"));
    }
    lines.push(String::new());
    lines.push("Compose (clone workflow) — MCP mirrors Make:".into());
    lines.push("  nctl_build / nctl_build_no_cache".into());
    lines.push("  nctl_start / nctl_start_log / nctl_build_start / nctl_build_start_log".into());
    lines.push("  nctl_stop / nctl_start_all / nctl_stop_all".into());
    lines.push("Hub image (no compose project):".into());
    lines.push("  nctl_start_docker / nctl_stop_docker  (like make start-docker, detached)".into());
    lines.push(format!(
        "MCP sidecar: make start-all / nctl_start_all → {DEFAULT_MCP_URL}"
    ));
    lines.push(String::new());
    lines.push(
        "Note: *-log Make targets attach to the terminal; MCP starts detached then returns a log tail."
            .into(),
    );
    lines.join("\n")
}

pub fn build(profile: &str) -> String {
    make_profile("build", profile)
}

pub fn build_no_cache(profile: &str) -> String {
    make_profile("build-no-cache", profile)
}

pub fn start_profile(profile: &str, pull_first: bool) -> String {
    if let Err(e) = require_profile(profile) {
        return e;
    }
    if let Some(msg) = refuse_unsafe_host_binds("nctl_start") {
        return msg;
    }
    if pull_first {
        let (code, out, err) = run("docker", &["compose", "--profile", profile, "pull"]);
        if code != 0 {
            return format!(
                "pull failed ({code}):\n{}",
                if err.is_empty() { out } else { err }
            );
        }
    }
    make_profile("start", profile)
}

/// `make start-log` parity: detached start + recent logs (no foreground hang).
pub fn start_log(profile: &str, log_lines: u32) -> String {
    if let Some(msg) = refuse_unsafe_host_binds("nctl_start_log") {
        return msg;
    }
    let started = make_profile("start", profile);
    if started.contains("failed") {
        return started;
    }
    format!(
        "{started}\n\n{}",
        docker_logs_tail(profile, log_lines.clamp(20, 200))
    )
}

pub fn build_start(profile: &str) -> String {
    if let Some(msg) = refuse_unsafe_host_binds("nctl_build_start") {
        return msg;
    }
    make_profile("build-start", profile)
}

/// `make build-start-log` parity: no-cache build + detached start + log tail.
pub fn build_start_log(profile: &str, log_lines: u32) -> String {
    if let Some(msg) = refuse_unsafe_host_binds("nctl_build_start_log") {
        return msg;
    }
    let built = make_profile("build-no-cache", profile);
    if built.contains("failed") {
        return built;
    }
    let started = make_profile("start", profile);
    if started.contains("failed") {
        return format!("{built}\n\n{started}");
    }
    format!(
        "{built}\n\n{started}\n\n{}",
        docker_logs_tail(profile, log_lines.clamp(20, 200))
    )
}

pub fn stop_profile(profile: &str) -> String {
    make_profile("stop", profile)
}

pub fn start_all(profile: &str) -> String {
    if let Some(msg) = refuse_unsafe_host_binds("nctl_start_all") {
        return msg;
    }
    make_profile("start-all", profile)
}

pub fn stop_all(profile: &str) -> String {
    make_profile("stop-all", profile)
}

fn hub_image(profile: &str) -> String {
    format!("interchouette/casper-nctl-2-docker:{profile}")
}

fn hub_container_name(profile: &str) -> String {
    format!("casper-nctl-2-docker-hub-{profile}")
}

/// `make start-docker` parity via detached `docker run` (MCP cannot attach `-it`).
pub fn start_docker(profile: &str) -> String {
    if let Err(e) = require_profile(profile) {
        return e;
    }
    if let Some(msg) = refuse_unsafe_host_binds("nctl_start_docker") {
        return msg;
    }
    let name = hub_container_name(profile);
    let image = hub_image(profile);
    // Bind sources must be host paths (Docker daemon does not remap MCP /workspace).
    let assets = host_repo_root().join("assets");

    // Remove leftover container with same name
    let _ = run("docker", &["rm", "-f", &name]);

    let faucet = assets.join("faucet").to_string_lossy().into_owned();
    let users = assets.join("users").to_string_lossy().into_owned();
    let chainspec = assets.join("chainspec").to_string_lossy().into_owned();
    let nodes = assets.join("nodes").to_string_lossy().into_owned();

    let (code, out, err) = run(
        "docker",
        &[
            "run",
            "-d",
            "--name",
            &name,
            "-p",
            "11101-11105:11101-11105",
            "-p",
            "14101-14105:14101-14105",
            "-p",
            "18101-18105:18101-18105",
            "-p",
            "25101-25105:25101-25105",
            "-p",
            "28101-28105:28101-28105",
            "-v",
            &format!("{faucet}:/app/casper-nctl/assets/net-1/faucet"),
            "-v",
            &format!("{users}:/app/casper-nctl/assets/net-1/users"),
            "-v",
            &format!("{chainspec}:/app/casper-nctl/assets/net-1/chainspec"),
            "-v",
            &format!("{nodes}:/app/casper-nctl/assets/net-1/nodes"),
            &image,
        ],
    );
    if code != 0 {
        return format!(
            "nctl_start_docker failed ({code}):\n{}",
            if err.is_empty() { out } else { err }
        );
    }
    format!(
        "Started Hub image {image} as container {name} (detached; Make start-docker is -it).\n\
         Prefer nctl_start (compose) for day-to-day clone workflow.\n{out}{err}"
    )
    .trim()
    .to_string()
}

pub fn stop_docker(profile: &str) -> String {
    if let Err(e) = require_profile(profile) {
        return e;
    }
    let name = hub_container_name(profile);
    let (code, out, err) = run("docker", &["rm", "-f", &name]);
    if code != 0 {
        return format!(
            "nctl_stop_docker failed ({code}):\n{}",
            if err.is_empty() { out } else { err }
        );
    }
    format!("Removed Hub-run container {name}.\n{out}{err}")
        .trim()
        .to_string()
}

pub fn start_cors() -> String {
    let (code, out, err) = run(
        "docker",
        &[
            "compose",
            "--profile",
            "cors-anywhere",
            "up",
            "--remove-orphans",
            "-d",
        ],
    );
    if code != 0 {
        return format!(
            "cors start failed ({code}):\n{}",
            if err.is_empty() { out } else { err }
        );
    }
    format!(
        "CORS proxy started on http://127.0.0.1:11100\n{}",
        if out.is_empty() { err } else { out }
    )
}

pub fn endpoints(profile: &str) -> String {
    let name = container_name_for_profile(profile);
    [
        format!("Profile: {profile}"),
        format!("Compose container: {name}"),
        format!("Hub-run container: {}", hub_container_name(profile)),
        String::new(),
        "RPC (node-1 typical):  http://127.0.0.1:11101/rpc".into(),
        "RPC nodes 1-5:         http://127.0.0.1:11101 … 11105/rpc".into(),
        "REST:                  http://127.0.0.1:14101 … 14105".into(),
        "SSE:                   http://127.0.0.1:18101 … 18105".into(),
        "Sidecar (2.x):         http://127.0.0.1:25101 … 25105".into(),
        "Other:                 28101-28105".into(),
        "CORS proxy (optional): http://127.0.0.1:11100".into(),
        format!("MCP (HTTP sidecar):   {DEFAULT_MCP_URL}"),
        String::new(),
        "Compose: nctl_start / nctl_build_start / …".into(),
        "Hub image: nctl_start_docker (like make start-docker)".into(),
        format!("Compose+MCP: nctl_start_all {profile}"),
    ]
    .join("\n")
}

fn docker_ps_names() -> Vec<String> {
    let (code, out, _) = run("docker", &["ps", "--format", "{{.Names}}"]);
    if code != 0 {
        return Vec::new();
    }
    out.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn rpc_reachable() -> bool {
    // Cursor MCP often runs this binary inside Docker. 127.0.0.1 there is the
    // MCP container, not the host-published NCTL ports. Probe host gateway first.
    let body = r#"{"jsonrpc":"2.0","id":1,"method":"info_get_status","params":[]}"#;
    let mut urls: Vec<String> = Vec::new();
    if let Ok(u) = std::env::var("NCTL_RPC_PROBE_URL") {
        if !u.is_empty() {
            urls.push(u);
        }
    }
    urls.push("http://host.docker.internal:11101/rpc".into());
    urls.push("http://127.0.0.1:11101/rpc".into());
    for url in urls {
        let (code, _, _) = run(
            "curl",
            &[
                "-sS",
                "-m",
                "2",
                "-X",
                "POST",
                &url,
                "-H",
                "Content-Type: application/json",
                "-d",
                body,
            ],
        );
        if code == 0 {
            return true;
        }
    }
    false
}

pub fn status(profile: &str) -> String {
    let name = container_name_for_profile(profile);
    let hub = hub_container_name(profile);
    let names = docker_ps_names();
    let nctl_running = names.iter().any(|n| n.contains(&name));
    let hub_running = names.iter().any(|n| n == &hub || n.contains(&hub));
    let mcp_running = names.iter().any(|n| n.contains("casper-nctl-2-docker-mcp"));
    let rpc_ok = if nctl_running || hub_running {
        rpc_reachable()
    } else {
        false
    };
    let mut lines = vec![
        format!("profile={profile}"),
        format!("compose_container={name} running={nctl_running}"),
        format!("hub_container={hub} running={hub_running}"),
        format!("mcp_container=casper-nctl-2-docker-mcp running={mcp_running}"),
        format!("rpc_11101_reachable={rpc_ok}"),
        format!("mcp_url={DEFAULT_MCP_URL}"),
        String::new(),
        assets::assets_summary(),
    ];
    if !names.is_empty() {
        lines.push(String::new());
        lines.push("docker ps (names):".into());
        for n in names {
            lines.push(format!("- {n}"));
        }
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_profiles_include_stable() {
        assert!(require_profile("stable").is_ok());
        assert!(require_profile("nope").is_err());
    }

    #[test]
    fn hub_names() {
        assert_eq!(hub_image("2.2"), "interchouette/casper-nctl-2-docker:2.2");
        assert_eq!(hub_container_name("2.2"), "casper-nctl-2-docker-hub-2.2");
    }
}
