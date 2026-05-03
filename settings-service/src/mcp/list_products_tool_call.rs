use std::{collections::BTreeMap, sync::Arc};

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
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct ListProductsResponse {
    #[property(description: "Total number of products returned.")]
    pub count: i64,

    #[property(description: "Products discovered from the union of templates and secrets, ordered alphabetically (with \"Shared\" first when present).")]
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
    const DESCRIPTION: &'static str = "List all known products in the SettingsService — every product_id that owns at least one template or one secret. Each entry also reports how many templates and secrets it has, so the AI can pick a product before drilling into `list_secrets` or `compile_template_yaml`. The special \"Shared\" scope is included by default.";
}

#[async_trait::async_trait]
impl McpToolCall<ListProductsInputData, ListProductsResponse> for ListProductsHandler {
    async fn execute_tool_call(
        &self,
        model: ListProductsInputData,
    ) -> Result<ListProductsResponse, String> {
        let include_shared = model.include_shared.unwrap_or(true);

        let mut counts: BTreeMap<String, (i64, i64)> = BTreeMap::new();

        let templates_per_product = self
            .app
            .templates
            .find_into_vec(|product_id, _| Some(product_id.to_string()))
            .await;
        for product_id in templates_per_product {
            counts.entry(product_id).or_default().0 += 1;
        }

        let secrets_snapshot = self.app.secrets.get_snapshot().await;
        for (product_id, items) in secrets_snapshot.by_product.iter() {
            counts.entry(product_id.clone()).or_default().1 += items.len() as i64;
        }

        let shared_secrets_count = secrets_snapshot.shared.len() as i64;

        let mut products: Vec<ProductListEntry> = Vec::new();

        if include_shared {
            products.push(ProductListEntry {
                product_id: SHARED_LITERAL.to_string(),
                templates_count: 0,
                secrets_count: shared_secrets_count,
            });
        }

        for (product_id, (templates_count, secrets_count)) in counts {
            products.push(ProductListEntry {
                product_id,
                templates_count,
                secrets_count,
            });
        }

        let count = products.len() as i64;
        Ok(ListProductsResponse { count, products })
    }
}
