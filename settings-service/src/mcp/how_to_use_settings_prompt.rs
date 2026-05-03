use std::collections::HashMap;

use mcp_server_middleware::*;

const PROMPT_BODY: &str = include_str!("../../MCP_PROMPT_HOW_TO_USE_SETTINGS.md");

pub struct HowToUseSettingsPromptHandler;

impl PromptDefinition for HowToUseSettingsPromptHandler {
    const PROMPT_NAME: &'static str = "how_to_use_settings";
    const DESCRIPTION: &'static str = "Loads the full guide on how to use this SettingsService MCP server: what templates and secrets are, how the Shared scope works, how template compilation renders local vs remote YAML, what missing-secret markers look like, the privacy rules of each tool, and the recommended discovery → read → write workflow. Read it before invoking any other tool of this MCP.";

    fn get_argument_descriptions() -> Vec<PromptArgumentDescription> {
        Vec::new()
    }
}

#[async_trait::async_trait]
impl McpPromptService for HowToUseSettingsPromptHandler {
    async fn execute_prompt(
        &self,
        _arguments: &HashMap<String, String>,
    ) -> Result<PromptExecutionResult, String> {
        Ok(PromptExecutionResult {
            description: "Guide for using the SettingsService MCP server.".to_string(),
            message: PROMPT_BODY.to_string(),
        })
    }
}
