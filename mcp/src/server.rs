//! MCP server (`mcpkit`) for `casper-nctl-2-docker-mcp` (stdio or Streamable HTTP).

#![allow(clippy::unused_async)]

use mcpkit::prelude::*;
use mcpkit::transport::stdio::StdioTransport;
use mcpkit_axum::McpRouter;

use crate::{assets, logs, ops};

/// MCP server handle exposing NCTL Docker tools.
pub struct NctlMcp;

// Keep in sync with Cargo.toml `version`.
#[mcp_server(name = "casper-nctl-2-docker", version = "2.2.2")]
impl NctlMcp {
    #[tool(description = "List profiles and Make↔MCP lifecycle parity (compose vs Hub docker run)")]
    async fn nctl_list_profiles(&self) -> ToolOutput {
        ToolOutput::text(ops::list_profiles())
    }

    #[tool(description = "make build <profile> — build compose image for profile")]
    async fn nctl_build(&self, profile: Option<String>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::build(&profile))
    }

    #[tool(description = "make build-no-cache <profile> — rebuild without cache")]
    async fn nctl_build_no_cache(&self, profile: Option<String>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::build_no_cache(&profile))
    }

    #[tool(
        description = "make start <profile> — compose up -d (NCTL only, no MCP sidecar). Optional pull_first."
    )]
    async fn nctl_start(&self, profile: Option<String>, pull_first: Option<bool>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::start_profile(&profile, pull_first.unwrap_or(false)))
    }

    #[tool(
        description = "make start-log parity: compose up -d then return recent docker logs (no foreground hang)"
    )]
    async fn nctl_start_log(&self, profile: Option<String>, log_lines: Option<u32>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::start_log(&profile, log_lines.unwrap_or(80)))
    }

    #[tool(description = "make build-start <profile> — build then compose up -d")]
    async fn nctl_build_start(&self, profile: Option<String>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::build_start(&profile))
    }

    #[tool(
        description = "make build-start-log parity: build-no-cache + start -d + recent logs (no foreground hang)"
    )]
    async fn nctl_build_start_log(
        &self,
        profile: Option<String>,
        log_lines: Option<u32>,
    ) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::build_start_log(&profile, log_lines.unwrap_or(80)))
    }

    #[tool(description = "make stop <profile> — compose down for NCTL profile")]
    async fn nctl_stop(&self, profile: Option<String>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::stop_profile(&profile))
    }

    #[tool(description = "make start-all <profile> — NCTL compose + MCP HTTP sidecar on :8788")]
    async fn nctl_start_all(&self, profile: Option<String>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::start_all(&profile))
    }

    #[tool(description = "make stop-all <profile> — stop NCTL compose + MCP sidecar")]
    async fn nctl_stop_all(&self, profile: Option<String>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::stop_all(&profile))
    }

    #[tool(
        description = "make start-docker parity: docker run Hub image interchouette/casper-nctl-2-docker:<profile> detached (Make uses -it)"
    )]
    async fn nctl_start_docker(&self, profile: Option<String>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::start_docker(&profile))
    }

    #[tool(description = "Stop/remove Hub-run container from nctl_start_docker")]
    async fn nctl_stop_docker(&self, profile: Option<String>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::stop_docker(&profile))
    }

    #[tool(description = "Container status (compose + hub-run + MCP), RPC, assets summary")]
    async fn nctl_status(&self, profile: Option<String>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::status(&profile))
    }

    #[tool(description = "Host URLs for RPC, REST, SSE, sidecar, CORS, and MCP HTTP")]
    async fn nctl_endpoints(&self, profile: Option<String>) -> ToolOutput {
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(ops::endpoints(&profile))
    }

    #[tool(description = "Start the cors-anywhere compose profile on port 11100")]
    async fn nctl_cors_start(&self) -> ToolOutput {
        ToolOutput::text(ops::start_cors())
    }

    #[tool(description = "Summarize host ./assets (faucet, users, nodes, chainspec, logs)")]
    async fn nctl_assets_summary(&self) -> ToolOutput {
        ToolOutput::text(assets::assets_summary())
    }

    #[tool(
        description = "Faucet public key and paths. Never returns secrets. No CSPR transfer in v1."
    )]
    async fn nctl_faucet_info(&self) -> ToolOutput {
        ToolOutput::text(assets::faucet_info())
    }

    #[tool(description = "List node-* under assets/nodes and whether keys/logs/storage exist")]
    async fn nctl_list_nodes(&self) -> ToolOutput {
        ToolOutput::text(assets::list_nodes())
    }

    #[tool(description = "List user-* under assets/users; optionally include public_key_hex")]
    async fn nctl_list_users(&self, include_public_hex: Option<bool>) -> ToolOutput {
        ToolOutput::text(assets::list_users(include_public_hex.unwrap_or(false)))
    }

    #[tool(description = "Read public_key_hex for 'faucet', 'user-N', or 'node-N'")]
    async fn nctl_read_public_key(&self, identity: String) -> ToolOutput {
        ToolOutput::text(assets::read_public_key(&identity))
    }

    #[tool(
        description = "List or read size-capped text under assets/ (default lists chainspec/). Refuses secrets."
    )]
    async fn nctl_read_chainspec(
        &self,
        relative: Option<String>,
        max_bytes: Option<u32>,
    ) -> ToolOutput {
        let relative = relative.unwrap_or_else(|| "chainspec".into());
        let max_bytes = usize::try_from(max_bytes.unwrap_or(16_384)).unwrap_or(16_384);
        ToolOutput::text(assets::read_chainspec(&relative, max_bytes))
    }

    #[tool(description = "List available log files under assets/logs and assets/nodes/*/logs")]
    async fn nctl_logs_list(&self) -> ToolOutput {
        ToolOutput::text(logs::logs_list())
    }

    #[tool(
        description = "Tail logs. source: docker | assets_stdout | sidecar | node (needs node_id)"
    )]
    async fn nctl_logs(
        &self,
        source: Option<String>,
        lines: Option<u32>,
        node_id: Option<u32>,
        profile: Option<String>,
    ) -> ToolOutput {
        let source = source.unwrap_or_else(|| "docker".into());
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(logs::logs_tail(
            &source,
            lines.unwrap_or(80),
            node_id,
            &profile,
        ))
    }

    #[tool(description = "Case-insensitive grep over log sources (capped matches)")]
    async fn nctl_logs_grep(
        &self,
        pattern: String,
        source: Option<String>,
        node_id: Option<u32>,
        profile: Option<String>,
        max_matches: Option<u32>,
    ) -> ToolOutput {
        let source = source.unwrap_or_else(|| "assets_stdout".into());
        let profile = profile.unwrap_or_else(|| "stable".into());
        ToolOutput::text(logs::logs_grep(
            &pattern,
            &source,
            node_id,
            &profile,
            max_matches.unwrap_or(40),
        ))
    }
}

/// Serves MCP over stdio until the client disconnects.
pub async fn run() -> Result<(), McpError> {
    let transport = StdioTransport::new();
    let server = ServerBuilder::new(NctlMcp)
        .with_tools(NctlMcp)
        .build();
    server.serve(transport).await
}

/// Default HTTP bind address for Streamable MCP.
pub const DEFAULT_HTTP_LISTEN: &str = "0.0.0.0:8788";

/// Serves MCP over Streamable HTTP until the process is stopped.
pub async fn run_http(addr: &str) -> std::io::Result<()> {
    McpRouter::new(NctlMcp).serve(addr).await
}

impl ResourceHandler for NctlMcp {
    async fn list_resources(&self, _ctx: &Context<'_>) -> Result<Vec<Resource>, McpError> {
        Ok(Vec::new())
    }

    async fn read_resource(
        &self,
        uri: &str,
        _ctx: &Context<'_>,
    ) -> Result<Vec<ResourceContents>, McpError> {
        Err(McpError::invalid_params(
            "resources/read",
            format!("unknown resource: {uri}"),
        ))
    }
}

impl PromptHandler for NctlMcp {
    async fn list_prompts(&self, _ctx: &Context<'_>) -> Result<Vec<Prompt>, McpError> {
        Ok(Vec::new())
    }

    async fn get_prompt(
        &self,
        name: &str,
        _args: Option<serde_json::Map<String, serde_json::Value>>,
        _ctx: &Context<'_>,
    ) -> Result<GetPromptResult, McpError> {
        Err(McpError::invalid_params(
            "prompts/get",
            format!("unknown prompt: {name}"),
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn mcp_server_version_matches_crate() {
        assert_eq!(
            env!("CARGO_PKG_VERSION"),
            "2.2.2",
            "bump #[mcp_server(version = …)] when changing Cargo.toml version"
        );
    }
}
