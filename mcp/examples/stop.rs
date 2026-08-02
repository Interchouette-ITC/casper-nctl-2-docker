//! `nctl_stop` for a profile (default `2.2`). Needs Docker.

use casper_nctl_2_docker_mcp::ops;

fn main() {
    let profile = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "2.2".to_string());
    println!("{}", ops::stop_profile(&profile));
}
