//! Print endpoints for a profile (offline text; default `stable`).

use casper_nctl_2_docker_mcp::ops;

fn main() {
    let profile = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "stable".to_string());
    println!("{}", ops::endpoints(&profile));
}
