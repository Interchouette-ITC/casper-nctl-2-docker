mod common;

use casper_nctl_2_docker_mcp::logs;
use common::FakeRepo;

#[test]
fn logs_list_finds_files() {
    let _fx = FakeRepo::new();
    let text = logs::logs_list();
    assert!(text.contains("stdout.log"));
    assert!(text.contains("stderr.log") || text.contains("node-1"));
}

#[test]
fn logs_grep_assets_stdout() {
    let _fx = FakeRepo::new();
    let out = logs::logs_grep("error", "assets_stdout", None, "stable", 40);
    assert!(out.contains("ERROR boom assets"), "{out}");
}

#[test]
fn logs_grep_node_stderr() {
    let _fx = FakeRepo::new();
    let out = logs::logs_grep("boom", "node", Some(1), "stable", 40);
    assert!(out.contains("ERROR boom node-1"), "{out}");
}

#[test]
fn logs_tail_node() {
    let _fx = FakeRepo::new();
    let out = logs::logs_tail("node", 10, Some(1), "stable");
    assert!(out.contains("stderr.log") || out.contains("ERROR boom"));
}

#[test]
fn logs_tail_missing_source() {
    let _fx = FakeRepo::new();
    let out = logs::logs_tail("nope", 10, None, "stable");
    assert!(out.contains("source must be"));
}
