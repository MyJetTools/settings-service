use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::app_ctx::AppContext;

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct UpsertProductInputData {
    #[property(description: "Product identifier to create or update.")]
    pub product_id: String,

    #[property(description: "Short product description shown in product lists.")]
    pub description: String,

    #[property(description: "Free-form prompt that explains the product context and how its settings are organised. Read by AI agents via `get_product_prompt` before they touch the product's secrets/templates.")]
    pub prompt: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct UpsertProductResponse {
    #[property(description: "Echoes the saved product id.")]
    pub product_id: String,
}

pub struct UpsertProductHandler {
    app: Arc<AppContext>,
}

impl UpsertProductHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for UpsertProductHandler {
    const FUNC_NAME: &'static str = "upsert_product";
    const DESCRIPTION: &'static str = "Create or update the explicit description and prompt for a product. The product becomes a first-class entity that other AI agents can discover via `list_products` and load via `get_product_prompt`. This call does not affect the product's secrets or templates.";
}

#[async_trait::async_trait]
impl McpToolCall<UpsertProductInputData, UpsertProductResponse> for UpsertProductHandler {
    async fn execute_tool_call(
        &self,
        model: UpsertProductInputData,
    ) -> Result<UpsertProductResponse, String> {
        let id = model.product_id.trim().to_string();
        if id.is_empty() {
            return Err("`product_id` must not be empty".to_string());
        }

        crate::flows::save_product(
            self.app.as_ref(),
            id.clone(),
            model.description,
            model.prompt,
        )
        .await;

        Ok(UpsertProductResponse { product_id: id })
    }
}
