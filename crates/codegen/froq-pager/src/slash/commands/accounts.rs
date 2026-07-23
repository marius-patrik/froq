//! `/accounts` (alias `/tokens`) -- Manage provider logins and API tokens.

use crate::slash::command::{AppCtx, ArgItem, CommandExecCtx, CommandResult, SlashCommand};

pub struct AccountsCommand;

impl SlashCommand for AccountsCommand {
    fn name(&self) -> &str {
        "accounts"
    }

    fn aliases(&self) -> &[&str] {
        &["tokens", "keys", "providers"]
    }

    fn description(&self) -> &str {
        "Manage accounts and API tokens (OpenAI, Anthropic, Kimi, Google, GitHub, etc.)"
    }

    fn usage(&self) -> &str {
        "/accounts [provider] [token]"
    }

    fn takes_args(&self) -> bool {
        true
    }

    fn suggest_args(&self, _ctx: &AppCtx, _args_query: &str) -> Option<Vec<ArgItem>> {
        Some(vec![
            ArgItem {
                display: "status".into(),
                match_text: "status".into(),
                insert_text: "status".into(),
                description: "View status of all provider credentials".into(),
            },
            ArgItem {
                display: "openai".into(),
                match_text: "openai".into(),
                insert_text: "openai ".into(),
                description: "Set OpenAI API key (OPENAI_API_KEY)".into(),
            },
            ArgItem {
                display: "anthropic".into(),
                match_text: "anthropic".into(),
                insert_text: "anthropic ".into(),
                description: "Set Anthropic API key (ANTHROPIC_API_KEY)".into(),
            },
            ArgItem {
                display: "kimi".into(),
                match_text: "kimi".into(),
                insert_text: "kimi ".into(),
                description: "Set Kimi/Moonshot API key (KIMI_API_KEY)".into(),
            },
            ArgItem {
                display: "google".into(),
                match_text: "google".into(),
                insert_text: "google ".into(),
                description: "Set Google Gemini API key (GEMINI_API_KEY)".into(),
            },
            ArgItem {
                display: "github".into(),
                match_text: "github".into(),
                insert_text: "github ".into(),
                description: "Set GitHub token (GITHUB_TOKEN)".into(),
            },
            ArgItem {
                display: "groq".into(),
                match_text: "groq".into(),
                insert_text: "groq ".into(),
                description: "Set Groq API key (GROQ_API_KEY)".into(),
            },
            ArgItem {
                display: "xai".into(),
                match_text: "xai".into(),
                insert_text: "xai ".into(),
                description: "Set xAI / Frog API key (XAI_API_KEY)".into(),
            },
        ])
    }

    fn run(&self, _ctx: &mut CommandExecCtx, args: &str) -> CommandResult {
        let parts: Vec<&str> = args.split_whitespace().collect();
        if parts.is_empty() || parts[0].eq_ignore_ascii_case("status") || parts[0].eq_ignore_ascii_case("list") {
            let openai = std::env::var("OPENAI_API_KEY").map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
            let anthropic = std::env::var("ANTHROPIC_API_KEY").map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
            let kimi = std::env::var("KIMI_API_KEY").or_else(|_| std::env::var("MOONSHOT_API_KEY")).map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
            let google = std::env::var("GEMINI_API_KEY").or_else(|_| std::env::var("GOOGLE_API_KEY")).map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
            let github = std::env::var("GITHUB_TOKEN").or_else(|_| std::env::var("GH_TOKEN")).map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
            let groq = std::env::var("GROQ_API_KEY").map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());
            let xai = std::env::var("XAI_API_KEY").or_else(|_| std::env::var("FROG_BUILD_API_KEY")).or_else(|_| std::env::var("FROG_API_KEY")).map(|k| mask_key(&k)).unwrap_or_else(|_| "Not set".into());

            let status_msg = format!(
                "Account & Token Provider Status:\n\n\
                   • OpenAI:     OPENAI_API_KEY      [{openai}]\n\
                   • Anthropic:  ANTHROPIC_API_KEY   [{anthropic}]\n\
                   • Kimi:       KIMI_API_KEY        [{kimi}]\n\
                   • Google:     GEMINI_API_KEY      [{google}]\n\
                   • GitHub:     GITHUB_TOKEN        [{github}]\n\
                   • Groq:       GROQ_API_KEY        [{groq}]\n\
                   • xAI / Frog: XAI_API_KEY         [{xai}]\n\n\
                 To configure a provider key in this session:\n\
                   /accounts <provider> <token>\n\
                 Example: /accounts openai sk-...\n\
                 Example: /tokens github ghp_..."
            );
            return CommandResult::Message(status_msg);
        }

        if parts.len() < 2 {
            return CommandResult::Error(
                "Usage: /accounts <provider> <token> (e.g., /accounts openai sk-...)".into(),
            );
        }

        let provider = parts[0].to_lowercase();
        let token = parts[1..].join(" ");

        let env_var = match provider.as_str() {
            "openai" => "OPENAI_API_KEY",
            "anthropic" => "ANTHROPIC_API_KEY",
            "kimi" | "moonshot" => "KIMI_API_KEY",
            "google" | "gemini" => "GEMINI_API_KEY",
            "github" | "gh" => "GITHUB_TOKEN",
            "groq" => "GROQ_API_KEY",
            "xai" | "frog" | "frog-build" => "XAI_API_KEY",
            other => {
                return CommandResult::Error(format!(
                    "Unknown provider '{other}'. Supported providers: openai, anthropic, kimi, google, github, groq, xai"
                ));
            }
        };

        unsafe {
            std::env::set_var(env_var, &token);
        }

        CommandResult::Message(format!(
            "Successfully set {env_var} for provider '{provider}' ({})",
            mask_key(&token)
        ))
    }
}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        "***".to_string()
    } else {
        format!("{}...{}", &key[..4], &key[key.len() - 4..])
    }
}
