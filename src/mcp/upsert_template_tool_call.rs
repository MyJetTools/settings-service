use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::app_ctx::AppContext;

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct UpsertTemplateInputData {
    #[property(description: "Product identifier the template belongs to. Use \"Shared\" for shared templates.")]
    pub product_id: String,

    #[property(description: "Template identifier within the product. Created if missing, overwritten if it already exists.")]
    pub template_id: String,

    #[property(description: "Full YAML body of the template. May contain ${secret_id} placeholders that will be resolved at compile time.")]
    pub yaml: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct UpsertTemplateResponse {
    #[property(description: "Echoes the product_id that was written.")]
    pub product_id: String,

    #[property(description: "Echoes the template_id that was written.")]
    pub template_id: String,
}

pub struct UpsertTemplateHandler {
    app: Arc<AppContext>,
}

impl UpsertTemplateHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for UpsertTemplateHandler {
    const FUNC_NAME: &'static str = "upsert_template";
    const DESCRIPTION: &'static str = "Create or overwrite a template's YAML body for a given product_id + template_id. YAML syntax is not validated by this call; bad YAML surfaces only when the template is later compiled with `compile_template_yaml`.";
}

#[async_trait::async_trait]
impl McpToolCall<UpsertTemplateInputData, UpsertTemplateResponse> for UpsertTemplateHandler {
    async fn execute_tool_call(
        &self,
        model: UpsertTemplateInputData,
    ) -> Result<UpsertTemplateResponse, String> {
        let product_id = model.product_id.trim().to_string();
        let template_id = model.template_id.trim().to_string();

        if product_id.is_empty() {
            return Err("`product_id` must not be empty (use \"Shared\" for shared templates)".to_string());
        }
        if template_id.is_empty() {
            return Err("`template_id` must not be empty".to_string());
        }

        crate::flows::save_template(
            self.app.as_ref(),
            &product_id,
            template_id.clone(),
            model.yaml,
        )
        .await;

        Ok(UpsertTemplateResponse {
            product_id,
            template_id,
        })
    }
}
