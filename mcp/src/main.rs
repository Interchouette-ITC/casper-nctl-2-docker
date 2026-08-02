//! `casper-nctl-2-docker-mcp` — MCP server (stdio by default, optional Streamable HTTP).

use anyhow::Result;
use casper_nctl_2_docker_mcp::server::{run, run_http, DEFAULT_HTTP_LISTEN};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "casper-nctl-2-docker-mcp",
    about = "casper-nctl-2-docker MCP server (stdio or Streamable HTTP)",
    version
)]
struct Cli {
    /// Serve Streamable HTTP instead of stdio (also: `NCTL_MCP_HTTP=1`).
    #[arg(long, env = "NCTL_MCP_HTTP")]
    http: bool,

    /// HTTP bind address when `--http` is set (also: `NCTL_MCP_ADDR`).
    #[arg(long, env = "NCTL_MCP_ADDR", default_value = DEFAULT_HTTP_LISTEN)]
    listen: String,
}

fn init_logging() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let cli = Cli::parse();

    if cli.http {
        tracing::info!(addr = %cli.listen, "casper-nctl-2-docker-mcp starting (HTTP)");
        run_http(&cli.listen)
            .await
            .map_err(|err| anyhow::anyhow!("{err}"))?;
    } else {
        tracing::info!("casper-nctl-2-docker-mcp starting (stdio)");
        run().await.map_err(|err| anyhow::anyhow!("{err}"))?;
    }
    Ok(())
}
