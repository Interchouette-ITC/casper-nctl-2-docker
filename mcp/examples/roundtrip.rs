//! start_log → status → stop (default `2.2`). Needs Docker + image.

use casper_nctl_2_docker_mcp::ops;
use std::thread;
use std::time::Duration;

fn main() {
    let profile = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "2.2".to_string());
    let lines: u32 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(40);

    println!("=== nctl_start_log({profile}) ===");
    println!("{}", ops::start_log(&profile, lines));

    println!("\n=== waiting 45s for RPC warmup ===");
    thread::sleep(Duration::from_secs(45));

    println!("\n=== nctl_status({profile}) ===");
    println!("{}", ops::status(&profile));

    println!("\n=== nctl_stop({profile}) ===");
    println!("{}", ops::stop_profile(&profile));
}
