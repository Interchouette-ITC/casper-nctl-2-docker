//! List profiles / Make↔MCP map (offline).

use casper_nctl_2_docker_mcp::ops;

fn main() {
    println!("{}", ops::list_profiles());
}
