use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::app_ctx::AppContext;

const SHARED_LITERAL: &str = "Shared";

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct ListProductsInputData {
    #[property(description: "When true, include the special \"Shared\" scope alongside real products. Defaults to true.")]
    pub include_shared: Option<bool>,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct ProductListEntry {
    #[property(description: "Product identifier. \"Shared\" denotes the shared scope (templates and secrets visible to every product).")]
    pub product_id: String,

    #[property(description: "Number of templates owned by this product.")]
    pub templates_count: i64,

    #[property(description: "Number of secrets owned by this product.")]
    pub secrets_count: i64,

    #[property(description: "Short product description (only present when an explicit product record exists).")]
    pub description: Option<String>,

    #[property(description: "True when this product has an explicit prompt the AI should read via `get_product_prompt` before working with its secrets/templates.")]
    pub has_prompt: bool,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct ListProductsResponse {
    #[property(description: "Total number of products returned.")]
    pub count: i64,

    #[property(description: "Products discovered from the union of explicit product records, templates and secrets, ordered alphabetically (with \"Shared\" first when present).")]
    pub products: Vec<ProductListEntry>,
}

pub struct ListProductsHandler {
    app: Arc<AppContext>,
}

impl ListProductsHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for ListProductsHandler {
    const FUNC_NAME: &'static str = "list_products";
    const DESCRIPTION: &'static str = "List all known products in the SettingsService — every product_id that owns at least one template, one secret, or has an explicit description/prompt record. Each entry reports how many templates and secrets it has, an optional description, and `has_prompt` to signal that you should call `get_product_prompt` next to load the product context. The special \"Shared\" scope is included by default.";
}

#[async_trait::async_trait]
impl McpToolCall<ListProductsInputData, ListProductsResponse> for ListProductsHandler {
    async fn execute_tool_call(
        &self,
        model: ListProductsInputData,
    ) -> Result<ListProductsResponse, String> {
        let include_shared = model.include_shared.unwrap_or(true);

        let aggregated = crate::flows::get_all_products(&self.app).await;

        let secrets_snapshot = self.app.secrets.get_snapshot().await;
        let shared_secrets_count = secrets_snapshot.shared.len() as i64;

        let mut products: Vec<ProductListEntry> = Vec::new();

        if include_shared {
            products.push(ProductListEntry {
                product_id: SHARED_LITERAL.to_string(),
                templates_count: 0,
                secrets_count: shared_secrets_count,
                description: None,
                has_prompt: false,
            });
        }

        for item in aggregated {
            let has_prompt = item
                .prompt
                .as_ref()
                .map(|p| !p.trim().is_empty())
                .unwrap_or(false);
            products.push(ProductListEntry {
                product_id: item.id,
                templates_count: item.templates_count as i64,
                secrets_count: item.secrets_count as i64,
                description: item.description.filter(|d| !d.trim().is_empty()),
                has_prompt,
            });
        }

        let count = products.len() as i64;
        Ok(ListProductsResponse { count, products })
    }
}
