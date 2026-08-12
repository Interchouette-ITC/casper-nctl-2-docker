//! MCP server (`rmcp`) for `casper-nctl-2-docker-mcp` (stdio or Streamable HTTP).

#![allow(clippy::unused_async)]

use std::sync::Arc;

use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, ContentBlock, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
    transport::stdio,
    ErrorData as McpError, ServerHandler, ServiceExt,
};

use crate::tool_args::*;
use crate::{assets, logs, ops};

/// MCP server handle exposing NCTL Docker tools.
#[derive(Clone, Default)]
pub struct NctlMcp;

/// Default HTTP bind address for Streamable MCP.
pub const DEFAULT_HTTP_LISTEN: &str = "0.0.0.0:8790";

fn text_ok(text: impl Into<String>) -> CallToolResult {
    CallToolResult::success(vec![ContentBlock::text(text.into())])
}

fn profile_or_stable(profile: Option<String>) -> String {
    profile.unwrap_or_else(|| "stable".into())
}

#[tool_router]
impl NctlMcp {
    #[tool(description = "List profiles and Make↔MCP lifecycle parity (compose vs Hub docker run)")]
    async fn nctl_list_profiles(&self) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::list_profiles()))
    }

    #[tool(description = "make build <profile> - build compose image for profile")]
    async fn nctl_build(
        &self,
        Parameters(ProfileArgs { profile }): Parameters<ProfileArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::build(&profile_or_stable(profile))))
    }

    #[tool(description = "make build-no-cache <profile> - rebuild without cache")]
    async fn nctl_build_no_cache(
        &self,
        Parameters(ProfileArgs { profile }): Parameters<ProfileArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::build_no_cache(&profile_or_stable(profile))))
    }

    #[tool(
        description = "make start <profile> - compose up -d (NCTL only, no MCP sidecar). Optional pull_first."
    )]
    async fn nctl_start(
        &self,
        Parameters(StartArgs {
            profile,
            pull_first,
        }): Parameters<StartArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::start_profile(
            &profile_or_stable(profile),
            pull_first.unwrap_or(false),
        )))
    }

    #[tool(
        description = "make start-log parity: compose up -d then return recent docker logs (no foreground hang)"
    )]
    async fn nctl_start_log(
        &self,
        Parameters(StartLogArgs { profile, log_lines }): Parameters<StartLogArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::start_log(
            &profile_or_stable(profile),
            log_lines.unwrap_or(80),
        )))
    }

    #[tool(description = "make build-start <profile> - build then compose up -d")]
    async fn nctl_build_start(
        &self,
        Parameters(ProfileArgs { profile }): Parameters<ProfileArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::build_start(&profile_or_stable(profile))))
    }

    #[tool(
        description = "make build-start-log parity: build-no-cache + start -d + recent logs (no foreground hang)"
    )]
    async fn nctl_build_start_log(
        &self,
        Parameters(StartLogArgs { profile, log_lines }): Parameters<StartLogArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::build_start_log(
            &profile_or_stable(profile),
            log_lines.unwrap_or(80),
        )))
    }

    #[tool(description = "make stop <profile> - compose down for NCTL profile")]
    async fn nctl_stop(
        &self,
        Parameters(ProfileArgs { profile }): Parameters<ProfileArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::stop_profile(&profile_or_stable(profile))))
    }

    #[tool(description = "make start-all <profile> - NCTL compose + MCP HTTP sidecar on :8790")]
    async fn nctl_start_all(
        &self,
        Parameters(ProfileArgs { profile }): Parameters<ProfileArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::start_all(&profile_or_stable(profile))))
    }

    #[tool(description = "make stop-all <profile> - stop NCTL compose + MCP sidecar")]
    async fn nctl_stop_all(
        &self,
        Parameters(ProfileArgs { profile }): Parameters<ProfileArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::stop_all(&profile_or_stable(profile))))
    }

    #[tool(
        description = "make start-docker parity: docker run Hub image interchouette/casper-nctl-2-docker:<profile> detached (Make uses -it)"
    )]
    async fn nctl_start_docker(
        &self,
        Parameters(ProfileArgs { profile }): Parameters<ProfileArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::start_docker(&profile_or_stable(profile))))
    }

    #[tool(description = "Stop/remove Hub-run container from nctl_start_docker")]
    async fn nctl_stop_docker(
        &self,
        Parameters(ProfileArgs { profile }): Parameters<ProfileArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::stop_docker(&profile_or_stable(profile))))
    }

    #[tool(description = "Container status (compose + hub-run + MCP), RPC, assets summary")]
    async fn nctl_status(
        &self,
        Parameters(ProfileArgs { profile }): Parameters<ProfileArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::status(&profile_or_stable(profile))))
    }

    #[tool(description = "Host URLs for RPC, REST, SSE, sidecar, CORS, and MCP HTTP")]
    async fn nctl_endpoints(
        &self,
        Parameters(ProfileArgs { profile }): Parameters<ProfileArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::endpoints(&profile_or_stable(profile))))
    }

    #[tool(description = "Start the cors-anywhere compose profile on port 11100")]
    async fn nctl_cors_start(&self) -> Result<CallToolResult, McpError> {
        Ok(text_ok(ops::start_cors()))
    }

    #[tool(description = "Summarize host ./assets (faucet, users, nodes, chainspec, logs)")]
    async fn nctl_assets_summary(&self) -> Result<CallToolResult, McpError> {
        Ok(text_ok(assets::assets_summary()))
    }

    #[tool(
        description = "Faucet public key and paths. Never returns secrets. No CSPR transfer in v1."
    )]
    async fn nctl_faucet_info(&self) -> Result<CallToolResult, McpError> {
        Ok(text_ok(assets::faucet_info()))
    }

    #[tool(description = "List node-* under assets/nodes and whether keys/logs/storage exist")]
    async fn nctl_list_nodes(&self) -> Result<CallToolResult, McpError> {
        Ok(text_ok(assets::list_nodes()))
    }

    #[tool(description = "List user-* under assets/users; optionally include public_key_hex")]
    async fn nctl_list_users(
        &self,
        Parameters(ListUsersArgs {
            include_public_hex,
        }): Parameters<ListUsersArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(assets::list_users(
            include_public_hex.unwrap_or(false),
        )))
    }

    #[tool(description = "Read public_key_hex for 'faucet', 'user-N', or 'node-N'")]
    async fn nctl_read_public_key(
        &self,
        Parameters(ReadPublicKeyArgs { identity }): Parameters<ReadPublicKeyArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(text_ok(assets::read_public_key(&identity)))
    }

    #[tool(
        description = "List or read size-capped text under assets/ (default lists chainspec/). Refuses secrets."
    )]
    async fn nctl_read_chainspec(
        &self,
        Parameters(ReadChainspecArgs { relative, max_bytes }): Parameters<ReadChainspecArgs>,
    ) -> Result<CallToolResult, McpError> {
        let relative = relative.unwrap_or_else(|| "chainspec".into());
        let max_bytes = usize::try_from(max_bytes.unwrap_or(16_384)).unwrap_or(16_384);
        Ok(text_ok(assets::read_chainspec(&relative, max_bytes)))
    }

    #[tool(description = "List available log files under assets/logs and assets/nodes/*/logs")]
    async fn nctl_logs_list(&self) -> Result<CallToolResult, McpError> {
        Ok(text_ok(logs::logs_list()))
    }

    #[tool(
        description = "Tail logs. source: docker | assets_stdout | sidecar | node (needs node_id)"
    )]
    async fn nctl_logs(
        &self,
        Parameters(LogsArgs {
            source,
            lines,
            node_id,
            profile,
        }): Parameters<LogsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let source = source.unwrap_or_else(|| "docker".into());
        Ok(text_ok(logs::logs_tail(
            &source,
            lines.unwrap_or(80),
            node_id,
            &profile_or_stable(profile),
        )))
    }

    #[tool(description = "Case-insensitive grep over log sources (capped matches)")]
    async fn nctl_logs_grep(
        &self,
        Parameters(LogsGrepArgs {
            pattern,
            source,
            node_id,
            profile,
            max_matches,
        }): Parameters<LogsGrepArgs>,
    ) -> Result<CallToolResult, McpError> {
        let source = source.unwrap_or_else(|| "assets_stdout".into());
        Ok(text_ok(logs::logs_grep(
            &pattern,
            &source,
            node_id,
            &profile_or_stable(profile),
            max_matches.unwrap_or(40),
        )))
    }
}

/// Serves MCP over stdio until the client disconnects.
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server = NctlMcp;
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

/// Serves MCP over Streamable HTTP until the process is stopped.
pub async fn run_http(addr: &str) -> std::io::Result<()> {
    let config =
        rmcp::transport::streamable_http_server::tower::StreamableHttpServerConfig::default();
    let service = rmcp::transport::streamable_http_server::tower::StreamableHttpService::new(
        || Ok(NctlMcp),
        Arc::new(
            rmcp::transport::streamable_http_server::session::local::LocalSessionManager::default(),
        ),
        config,
    );
    let method_router = axum::routing::any_service(service);
    let app = axum::Router::new()
        .route("/mcp", method_router.clone())
        .route("/mcp/", method_router);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "casper-nctl-2-docker-mcp HTTP listening");
    axum::serve(listener, app).await?;
    Ok(())
}

#[tool_handler]
impl ServerHandler for NctlMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(rmcp::model::Implementation::new(
                "casper-nctl-2-docker",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "MCP tools for local casper-nctl-2-docker: compose/Hub lifecycle, assets, faucet public keys, and logs. Set NCTL_DOCKER_ROOT / NCTL_HOST_ROOT for lifecycle tools.",
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_server_version_matches_crate() {
        let info = NctlMcp.get_info();
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(info.server_info.name.as_str(), "casper-nctl-2-docker");
    }
}
