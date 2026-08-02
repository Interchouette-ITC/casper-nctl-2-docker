//! Print faucet public key via `assets::faucet_info` (needs assets after NCTL setup).

use casper_nctl_2_docker_mcp::assets;

fn main() {
    println!("{}", assets::faucet_info());
    println!();
    println!("{}", assets::read_public_key("faucet"));
}
