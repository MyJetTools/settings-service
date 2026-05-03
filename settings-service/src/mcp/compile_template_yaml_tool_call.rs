use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use rust_common::placeholders::{ContentToken, PlaceholdersIterator};
use serde::{Deserialize, Serialize};

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
    #[property(description: "YAML body of the template with each `${secret_id}` placeholder REWRITTEN — never substituted with the real value. Each placeholder becomes either `SECRET_<id>_VALUE` (when the secret exists in scope) or `SECRET_<id>_NOT_FOUND` (when it does not). This lets the AI inspect template structure and spot missing secrets without ever reading actual secret values.")]
    pub yaml: String,

    #[property(description: "True when at least one referenced secret is missing in the rendered YAML.")]
    pub has_missing_keys: bool,

    #[property(description: "List of secret ids referenced by the template but not found in the secret store.")]
    pub missing_keys: Vec<String>,
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
    const DESCRIPTION: &'static str = "Read a YAML template for a given product_id + template_id and rewrite each `${secret_id}` placeholder as a marker — `SECRET_<id>_VALUE` when the secret exists, `SECRET_<id>_NOT_FOUND` when it does not. Real secret values are never returned by this tool. Missing ids are also collected in `missing_keys` for easy reporting.";
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
        let mut seen_missing = std::collections::HashSet::new();

        for token in PlaceholdersIterator::new(
            template_content.as_str(),
            crate::consts::PLACEHOLDER_OPEN,
            crate::consts::PLACEHOLDER_CLOSE,
        ) {
            match token {
                ContentToken::Text(text) => yaml.push_str(text),
                ContentToken::Placeholder(secret_id) => {
                    if secret_id.starts_with('$') {
                        yaml.push_str(crate::consts::PLACEHOLDER_OPEN);
                        yaml.push_str(&secret_id[1..]);
                        yaml.push_str(crate::consts::PLACEHOLDER_CLOSE);
                    } else if secrets_snapshot.has_secret_to_consume(lookup_product, secret_id) {
                        yaml.push_str("SECRET_");
                        yaml.push_str(secret_id);
                        yaml.push_str("_VALUE");
                    } else {
                        yaml.push_str("SECRET_");
                        yaml.push_str(secret_id);
                        yaml.push_str("_NOT_FOUND");
                        if seen_missing.insert(secret_id.to_string()) {
                            missing_keys.push(secret_id.to_string());
                        }
                    }
                }
            }
        }

        Ok(CompileTemplateYamlResponse {
            has_missing_keys: !missing_keys.is_empty(),
            missing_keys,
            yaml,
        })
    }
}
