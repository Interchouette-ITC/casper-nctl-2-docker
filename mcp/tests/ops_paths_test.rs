mod common;

use casper_nctl_2_docker_mcp::ops;
use casper_nctl_2_docker_mcp::paths::{
    container_name_for_profile, is_secret_filename, safe_under_assets, PROFILES,
};
use common::FakeRepo;

#[test]
fn profiles_include_expected() {
    for p in ["stable", "2.2", "dev", "1.5.8"] {
        assert!(PROFILES.contains(&p), "missing {p}");
    }
}

#[test]
fn list_profiles_mentions_parity() {
    let text = ops::list_profiles();
    assert!(text.contains("nctl_start"));
    assert!(text.contains("nctl_start_docker"));
    assert!(text.contains("2.2"));
}

#[test]
fn endpoints_include_mcp_and_rpc() {
    let text = ops::endpoints("2.2");
    assert!(text.contains("11101"));
    assert!(text.contains("8790"));
    assert!(text.contains("casper-nctl-2-docker-2.2"));
}

#[test]
fn container_naming() {
    assert_eq!(
        container_name_for_profile("2.2"),
        "casper-nctl-2-docker-2.2"
    );
}

#[test]
fn safe_under_assets_rejects_escape() {
    let _fx = FakeRepo::new();
    assert!(safe_under_assets("../docker-compose.yml").is_err());
    assert!(safe_under_assets("/etc/passwd").is_err());
}

#[test]
fn secret_filename_detection() {
    assert!(is_secret_filename("secret_key.pem"));
    assert!(!is_secret_filename("public_key_hex"));
}

#[test]
fn unknown_profile_start_errors() {
    let out = ops::start_profile("no-such-profile", false);
    assert!(out.contains("Unknown profile"), "{out}");
}

#[test]
fn start_docker_refuses_workspace_host_binds() {
    use std::env;

    let _g = common::lock_env();
    env::set_var("NCTL_DOCKER_ROOT", "/workspace");
    env::remove_var("NCTL_HOST_ROOT");
    let out = ops::start_docker("dev");
    assert!(out.contains("REFUSING"), "{out}");
    assert!(out.contains("NCTL_HOST_ROOT"), "{out}");
    env::remove_var("NCTL_DOCKER_ROOT");
}

#[test]
fn status_runs_with_fixture() {
    let _fx = FakeRepo::new();
    let text = ops::status("2.2");
    assert!(text.contains("profile=2.2"));
    assert!(text.contains("assets_root="));
    assert!(text.contains("users=2"));
}
