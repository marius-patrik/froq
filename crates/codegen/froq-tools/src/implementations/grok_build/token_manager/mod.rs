//! ManageTokens tool -- manage API tokens and provider keys.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::requirements::{Expr, ToolRequirement};
use crate::types::tool::{ToolKind, ToolNamespace};

pub const MANAGE_TOKENS_TOOL_NAME: &str = "manage_tokens";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ManageTokensInput {
    #[schemars(
        description = "Action to perform: 'list' (view all tokens), 'set' (configure a provider token), 'get' (query a provider token), or 'delete' (clear a provider token)."
    )]
    pub action: String,

    #[schemars(
        description = "Optional. Provider name: 'openai', 'anthropic', 'kimi', 'google', 'github', 'groq', 'xai', etc."
    )]
    pub provider: Option<String>,

    #[schemars(
        description = "Optional. The API key or token string to set when action='set'."
    )]
    pub token: Option<String>,
}

#[derive(Debug, Default)]
pub struct ManageTokensTool;

impl crate::types::tool_metadata::ToolMetadata for ManageTokensTool {
    fn kind(&self) -> ToolKind {
        ToolKind::Other
    }

    fn tool_namespace(&self) -> ToolNamespace {
        ToolNamespace::GrokBuild
    }

    fn description_template(&self) -> &str {
        "View, configure, or update API tokens and provider keys (OpenAI, Anthropic, Kimi, Google, GitHub, Groq, xAI, etc.)."
    }

    fn requires_expr(&self) -> Expr<ToolRequirement> {
        Expr::True
    }
}

impl froq_tool_runtime::Tool for ManageTokensTool {
    type Args = ManageTokensInput;
    type Output = String;

    fn id(&self) -> froq_tool_protocol::ToolId {
        froq_tool_protocol::ToolId::new(MANAGE_TOKENS_TOOL_NAME).expect("valid tool id")
    }

    fn description(
        &self,
        _ctx: &froq_tool_runtime::ListToolsContext,
    ) -> froq_tool_types::ToolDescription {
        froq_tool_types::ToolDescription::new(
            MANAGE_TOKENS_TOOL_NAME,
            crate::types::tool_metadata::ToolMetadata::description_template(self),
        )
    }

    fn capabilities(&self) -> froq_tool_protocol::ToolCapabilities {
        froq_tool_protocol::ToolCapabilities {
            is_read_only: false,
            tool_scope: Some(froq_tool_protocol::ToolScope::ReadWrite),
            ..Default::default()
        }
    }

    async fn run(
        &self,
        _ctx: froq_tool_runtime::ToolCallContext,
        input: ManageTokensInput,
    ) -> Result<String, froq_tool_runtime::ToolError> {
        let action = input.action.to_lowercase();
        match action.as_str() {
            "list" | "status" => Ok(format_token_status()),
            "get" => {
                let provider = input.provider.as_deref().unwrap_or("all");
                if provider == "all" {
                    Ok(format_token_status())
                } else {
                    let (env_var, val) = get_provider_var(provider);
                    match val {
                        Some(key) => Ok(format!("{provider} ({env_var}): {}", mask_key(&key))),
                        None => Ok(format!("{provider} ({env_var}): Not set")),
                    }
                }
            }
            "set" => {
                let Some(provider) = input.provider.as_deref() else {
                    return Ok("Error: 'provider' parameter is required for action='set'".to_string());
                };
                let Some(token) = input.token.as_deref() else {
                    return Ok("Error: 'token' parameter is required for action='set'".to_string());
                };

                let env_var = match resolve_env_var_name(provider) {
                    Some(v) => v,
                    None => {
                        return Ok(format!(
                            "Error: Unknown provider '{provider}'. Supported: openai, anthropic, kimi, google, github, groq, xai"
                        ));
                    }
                };

                unsafe {
                    std::env::set_var(env_var, token);
                }

                Ok(format!(
                    "Successfully configured {env_var} for '{provider}' ({})",
                    mask_key(token)
                ))
            }
            "delete" | "clear" | "unset" => {
                let Some(provider) = input.provider.as_deref() else {
                    return Ok("Error: 'provider' parameter is required for action='delete'".to_string());
                };
                let env_var = match resolve_env_var_name(provider) {
                    Some(v) => v,
                    None => {
                        return Ok(format!(
                            "Error: Unknown provider '{provider}'. Supported: openai, anthropic, kimi, google, github, groq, xai"
                        ));
                    }
                };

                unsafe {
                    std::env::remove_var(env_var);
                }

                Ok(format!("Cleared {env_var} for '{provider}'"))
            }
            other => Ok(format!(
                "Unknown action '{other}'. Valid actions: list, get, set, delete"
            )),
        }
    }
}

fn resolve_env_var_name(provider: &str) -> Option<&'static str> {
    match provider.to_lowercase().as_str() {
        "openai" => Some("OPENAI_API_KEY"),
        "anthropic" => Some("ANTHROPIC_API_KEY"),
        "kimi" | "moonshot" => Some("KIMI_API_KEY"),
        "google" | "gemini" => Some("GEMINI_API_KEY"),
        "github" | "gh" => Some("GITHUB_TOKEN"),
        "groq" => Some("GROQ_API_KEY"),
        "xai" | "frog" | "frog-build" => Some("XAI_API_KEY"),
        _ => None,
    }
}

fn get_provider_var(provider: &str) -> (&'static str, Option<String>) {
    let var = resolve_env_var_name(provider).unwrap_or("UNKNOWN");
    let val = std::env::var(var).ok();
    (var, val)
}

fn format_token_status() -> String {
    let openai = std::env::var("OPENAI_API_KEY").map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
    let anthropic = std::env::var("ANTHROPIC_API_KEY").map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
    let kimi = std::env::var("KIMI_API_KEY").or_else(|_| std::env::var("MOONSHOT_API_KEY")).map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
    let google = std::env::var("GEMINI_API_KEY").or_else(|_| std::env::var("GOOGLE_API_KEY")).map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
    let github = std::env::var("GITHUB_TOKEN").or_else(|_| std::env::var("GH_TOKEN")).map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
    let groq = std::env::var("GROQ_API_KEY").map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
    let xai = std::env::var("XAI_API_KEY").or_else(|_| std::env::var("FROG_BUILD_API_KEY")).or_else(|_| std::env::var("FROG_API_KEY")).map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());

    format!(
        "Provider Token Status:\n\
         • OpenAI:     OPENAI_API_KEY      [{openai}]\n\
         • Anthropic:  ANTHROPIC_API_KEY   [{anthropic}]\n\
         • Kimi:       KIMI_API_KEY        [{kimi}]\n\
         • Google:     GEMINI_API_KEY      [{google}]\n\
         • GitHub:     GITHUB_TOKEN        [{github}]\n\
         • Groq:       GROQ_API_KEY        [{groq}]\n\
         • xAI / Frog: XAI_API_KEY         [{xai}]"
    )
}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        "***".to_string()
    } else {
        format!("{}...{}", &key[..4], &key[key.len() - 4..])
    }
}
