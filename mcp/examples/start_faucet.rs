//! `nctl_start` then `nctl_faucet_info` (default profile `2.2`). Needs Docker + image.

use casper_nctl_2_docker_mcp::{assets, ops};

fn main() {
    let profile = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "2.2".to_string());
    println!("=== nctl_start({profile}) ===");
    println!("{}", ops::start_profile(&profile, false));
    println!("\n=== nctl_faucet_info ===");
    println!("{}", assets::faucet_info());
}
