use std::collections::HashSet;
use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use rust_common::placeholders::{ContentToken, PlaceholdersIterator};
use serde::{Deserialize, Serialize};

use crate::caches::SecretsSnapshot;
use crate::{app_ctx::AppContext, models::ProductId};

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct CompileTemplateYamlInputData {
    #[property(description: "Product identifier the template belongs to. Use \"Shared\" for shared templates.")]
    pub product_id: String,

    #[property(description: "Template identifier within the product.")]
    pub template_id: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct CompileTemplateYamlResponse {
    #[property(description: "YAML body of the template. For each `${secret_id}` placeholder: if the secret exists AND has `visible_for_mcp = true` its actual value is INJECTED in place; if it exists but is NOT visible to MCP the placeholder becomes the marker `SECRET_<id>_VALUE`; if it does not exist it becomes `SECRET_<id>_NOT_FOUND`. Hidden secrets are listed in `hidden_keys`, missing ones in `missing_keys`.")]
    pub yaml: String,

    #[property(description: "True when at least one referenced secret is missing in the rendered YAML.")]
    pub has_missing_keys: bool,

    #[property(description: "List of secret ids referenced by the template but not found in the secret store.")]
    pub missing_keys: Vec<String>,

    #[property(description: "List of secret ids that exist but are not visible to MCP (rendered as `SECRET_<id>_VALUE` markers instead of their values).")]
    pub hidden_keys: Vec<String>,
}

pub struct CompileTemplateYamlHandler {
    app: Arc<AppContext>,
}

impl CompileTemplateYamlHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for CompileTemplateYamlHandler {
    const FUNC_NAME: &'static str = "compile_template_yaml";
    const DESCRIPTION: &'static str = "Read a YAML template for a given product_id + template_id. Each `${secret_id}` placeholder is resolved: secrets marked `visible_for_mcp` have their real value injected; secrets that exist but are hidden from MCP become the marker `SECRET_<id>_VALUE`; missing secrets become `SECRET_<id>_NOT_FOUND`. Hidden/missing ids are also returned separately so the AI can report them.";
}

#[async_trait::async_trait]
impl McpToolCall<CompileTemplateYamlInputData, CompileTemplateYamlResponse>
    for CompileTemplateYamlHandler
{
    async fn execute_tool_call(
        &self,
        model: CompileTemplateYamlInputData,
    ) -> Result<CompileTemplateYamlResponse, String> {
        let product_id = model.product_id.trim();
        let template_id = model.template_id.trim();

        if product_id.is_empty() {
            return Err(
                "`product_id` must not be empty (use \"Shared\" for shared templates)".to_string(),
            );
        }
        if template_id.is_empty() {
            return Err("`template_id` must not be empty".to_string());
        }

        let template_content = self
            .app
            .templates
            .get_by_id(product_id, template_id, |itm| itm.content.clone())
            .await
            .ok_or_else(|| format!("Template {}/{} not found", product_id, template_id))?;

        let secrets_snapshot = self.app.secrets.get_snapshot().await;
        let lookup_product: ProductId = product_id.into();

        let mut yaml = String::new();
        let mut missing_keys: Vec<String> = Vec::new();
        let mut hidden_keys: Vec<String> = Vec::new();
        let mut seen_missing = HashSet::new();
        let mut seen_hidden = HashSet::new();

        expand_for_mcp(
            template_content.as_str(),
            lookup_product,
            &secrets_snapshot,
            &mut yaml,
            &mut missing_keys,
            &mut hidden_keys,
            &mut seen_missing,
            &mut seen_hidden,
            &mut HashSet::new(),
        );

        Ok(CompileTemplateYamlResponse {
            has_missing_keys: !missing_keys.is_empty(),
            missing_keys,
            hidden_keys,
            yaml,
        })
    }
}

/// Expand a chunk of content for the MCP tool call.
///
/// - `${secret_id}` where the secret is `visible_for_mcp` → injected value
///   (recursively expanded with the same rules).
/// - `${secret_id}` where the secret exists but is hidden from MCP →
///   `SECRET_<id>_VALUE` marker.
/// - `${secret_id}` where the secret is missing → `SECRET_<id>_NOT_FOUND`.
/// - `$${...}` escape → kept as a literal `${...}`.
///
/// `expanding` guards against secret-cycle loops.
fn expand_for_mcp(
    content: &str,
    product_id: ProductId<'_>,
    snapshot: &SecretsSnapshot,
    out: &mut String,
    missing_keys: &mut Vec<String>,
    hidden_keys: &mut Vec<String>,
    seen_missing: &mut HashSet<String>,
    seen_hidden: &mut HashSet<String>,
    expanding: &mut HashSet<String>,
) {
    for token in PlaceholdersIterator::new(
        content,
        crate::consts::PLACEHOLDER_OPEN,
        crate::consts::PLACEHOLDER_CLOSE,
    ) {
        match token {
            ContentToken::Text(text) => out.push_str(text),
            ContentToken::Placeholder(secret_id) => {
                if let Some(stripped) = secret_id.strip_prefix('$') {
                    out.push_str(crate::consts::PLACEHOLDER_OPEN);
                    out.push_str(stripped);
                    out.push_str(crate::consts::PLACEHOLDER_CLOSE);
                    continue;
                }

                let Some(secret) = snapshot.consume_secret(product_id, secret_id) else {
                    out.push_str("SECRET_");
                    out.push_str(secret_id);
                    out.push_str("_NOT_FOUND");
                    if seen_missing.insert(secret_id.to_string()) {
                        missing_keys.push(secret_id.to_string());
                    }
                    continue;
                };

                if !secret.visible_for_mcp {
                    out.push_str("SECRET_");
                    out.push_str(secret_id);
                    out.push_str("_VALUE");
                    if seen_hidden.insert(secret_id.to_string()) {
                        hidden_keys.push(secret_id.to_string());
                    }
                    continue;
                }

                if !expanding.insert(secret_id.to_string()) {
                    // Cycle: bail to marker rather than overflow stack.
                    out.push_str("SECRET_");
                    out.push_str(secret_id);
                    out.push_str("_VALUE");
                    continue;
                }

                let value = secret.content.as_str().to_string();
                if value.contains(crate::consts::PLACEHOLDER_OPEN) {
                    expand_for_mcp(
                        &value,
                        product_id,
                        snapshot,
                        out,
                        missing_keys,
                        hidden_keys,
                        seen_missing,
                        seen_hidden,
                        expanding,
                    );
                } else {
                    out.push_str(&value);
                }
                expanding.remove(secret_id);
            }
        }
    }
}
