use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::app_ctx::AppContext;

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct GetProductPromptInputData {
    #[property(description: "Product id whose prompt and description should be loaded.")]
    pub product_id: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct GetProductPromptResponse {
    #[property(description: "Product id (echoed back).")]
    pub product_id: String,

    #[property(description: "Short product description. None when no explicit product record exists.")]
    pub description: Option<String>,

    #[property(description: "Free-form prompt that explains the product context and how its settings are organised. Read this in full before working with secrets/templates of this product.")]
    pub prompt: Option<String>,

    #[property(description: "False when this product has no explicit record (it was created implicitly by saving a secret or template). In that case description and prompt will be None.")]
    pub has_metadata: bool,
}

pub struct GetProductPromptHandler {
    app: Arc<AppContext>,
}

impl GetProductPromptHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for GetProductPromptHandler {
    const FUNC_NAME: &'static str = "get_product_prompt";
    const DESCRIPTION: &'static str = "Load the description and prompt that explain a product's purpose and how its settings are organised. Call this BEFORE working with secrets or templates of a specific product (e.g. before `list_secrets`, `compile_template_yaml`, or `upsert_template`) so that you understand what the product actually is. Returns has_metadata=false when the product exists only implicitly (created by saving a secret) and no explicit prompt has been set.";
}

#[async_trait::async_trait]
impl McpToolCall<GetProductPromptInputData, GetProductPromptResponse> for GetProductPromptHandler {
    async fn execute_tool_call(
        &self,
        model: GetProductPromptInputData,
    ) -> Result<GetProductPromptResponse, String> {
        let snapshot = self.app.products.get_snapshot().await;

        if let Some(product) = snapshot.get(model.product_id.as_str()) {
            return Ok(GetProductPromptResponse {
                product_id: product.id.clone(),
                description: Some(product.description.clone()),
                prompt: Some(product.prompt.clone()),
                has_metadata: true,
            });
        }

        Ok(GetProductPromptResponse {
            product_id: model.product_id,
            description: None,
            prompt: None,
            has_metadata: false,
        })
    }
}
