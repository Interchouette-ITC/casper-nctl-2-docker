//! JSON-schema parameter structs for rmcp `Parameters<T>` tool handlers.

use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ProfileArgs {
    #[serde(default)]
    pub profile: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct StartArgs {
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub pull_first: Option<bool>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct StartLogArgs {
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub log_lines: Option<u32>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ListUsersArgs {
    #[serde(default)]
    pub include_public_hex: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadPublicKeyArgs {
    pub identity: String,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ReadChainspecArgs {
    #[serde(default)]
    pub relative: Option<String>,
    #[serde(default)]
    pub max_bytes: Option<u32>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct LogsArgs {
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub lines: Option<u32>,
    #[serde(default)]
    pub node_id: Option<u32>,
    #[serde(default)]
    pub profile: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LogsGrepArgs {
    pub pattern: String,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub node_id: Option<u32>,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub max_matches: Option<u32>,
}
