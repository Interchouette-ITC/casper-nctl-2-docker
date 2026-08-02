mod common;

use casper_nctl_2_docker_mcp::assets;
use common::FakeRepo;

#[test]
fn faucet_info_returns_public_hex_hides_secret() {
    let _fx = FakeRepo::new();
    let text = assets::faucet_info();
    assert!(text.contains("FAUCETHEXDEADBEEF"));
    assert!(!text.contains("TOPSECRET"));
    assert!(text.contains("never returned"));
}

#[test]
fn read_public_key_identities() {
    let _fx = FakeRepo::new();
    assert!(assets::read_public_key("faucet").contains("FAUCETHEXDEADBEEF"));
    assert!(assets::read_public_key("user-1").contains("USER1HEX"));
    assert!(assets::read_public_key("node-1").contains("NODE1HEX"));
    assert!(assets::read_public_key("nope").contains("must be"));
}

#[test]
fn list_users_and_nodes() {
    let _fx = FakeRepo::new();
    let users = assets::list_users(true);
    assert!(users.contains("user-1"));
    assert!(users.contains("USER1HEX"));
    let nodes = assets::list_nodes();
    assert!(nodes.contains("node-1"));
    assert!(nodes.contains("keys=true"));
}

#[test]
fn summary_counts() {
    let _fx = FakeRepo::new();
    let s = assets::assets_summary();
    assert!(s.contains("users=2"));
    assert!(s.contains("nodes=2"));
    assert!(s.contains("public_key_hex=true"));
}

#[test]
fn chainspec_list_and_refuse_secret() {
    let _fx = FakeRepo::new();
    let list = assets::read_chainspec("chainspec", 100);
    assert!(list.contains("chainspec.toml"));
    let bad = assets::read_chainspec("faucet/secret_key.pem", 100);
    assert!(bad.to_lowercase().contains("refusing"));
}

#[test]
fn read_accounts_snippet() {
    let _fx = FakeRepo::new();
    let text = assets::read_chainspec("users/accounts.toml", 1024);
    assert!(text.contains("[[accounts]]"));
}
