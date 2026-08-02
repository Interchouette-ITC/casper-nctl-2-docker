//! `nctl_start_log` for a profile (default `2.2`). Needs Docker + image.

use casper_nctl_2_docker_mcp::ops;

fn main() {
    let profile = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "2.2".to_string());
    let lines: u32 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(40);
    println!("{}", ops::start_log(&profile, lines));
}
