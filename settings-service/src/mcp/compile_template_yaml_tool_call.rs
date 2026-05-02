use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
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
    #[property(description: "Compiled YAML for the local (root) datacenter. Missing secrets appear inline as `/*Secret <id> is not found*/` so they can be detected and reported.")]
    pub yaml: String,

    #[property(description: "Compiled YAML for the remote datacenter when it differs from local; omitted otherwise.")]
    pub remote_yaml: Option<String>,

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
    const DESCRIPTION: &'static str = "Render a YAML template for a given product_id + template_id with all referenced secrets substituted. Missing secrets are kept inline as `/*Secret <id> is not found*/` markers AND returned in `missing_keys` so an AI can clearly report which keys are absent.";
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
            return Err("`product_id` must not be empty (use \"Shared\" for shared templates)".to_string());
        }
        if template_id.is_empty() {
            return Err("`template_id` must not be empty".to_string());
        }

        let compiled = crate::flows::compile_yaml(self.app.as_ref(), product_id, template_id)
            .await
            .ok_or_else(|| {
                format!(
                    "Template {}/{} not found",
                    product_id, template_id
                )
            })?;

        let remote_yaml = if compiled.remote == compiled.local {
            None
        } else {
            Some(compiled.remote)
        };

        let secrets_snapshot = self.app.secrets.get_snapshot().await;
        let template_content = self
            .app
            .templates
            .get_by_id(product_id, template_id, |itm| itm.content.clone())
            .await;

        let missing_keys: Vec<String> = match template_content {
            Some(content) => {
                let lookup_product: ProductId = product_id.into();
                let mut seen = std::collections::HashSet::new();
                let mut out = Vec::new();
                for key in content.get_secrets() {
                    if !secrets_snapshot.has_secret_to_consume(lookup_product, key)
                        && seen.insert(key.to_string())
                    {
                        out.push(key.to_string());
                    }
                }
                out
            }
            None => Vec::new(),
        };

        Ok(CompileTemplateYamlResponse {
            yaml: compiled.local,
            remote_yaml,
            has_missing_keys: !missing_keys.is_empty(),
            missing_keys,
        })
    }
}
