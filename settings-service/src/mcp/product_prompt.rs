use std::{collections::HashMap, sync::Arc};

use mcp_server_middleware::*;

use crate::app_ctx::AppContext;

const ARG_PRODUCT_ID: &str = "product_id";

pub struct ProductPromptHandler {
    app: Arc<AppContext>,
}

impl ProductPromptHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl PromptDefinition for ProductPromptHandler {
    const PROMPT_NAME: &'static str = "product_prompt";
    const DESCRIPTION: &'static str = "Loads the description and free-form prompt of a product (the same content `get_product_prompt` returns) as an MCP prompt. Pass `product_id` as the only argument. Use this prompt when the AI client wants to inject the product's context directly into the conversation rather than calling a tool.";

    fn get_argument_descriptions() -> Vec<PromptArgumentDescription> {
        vec![PromptArgumentDescription {
            name: ARG_PRODUCT_ID.to_string(),
            description: "Product id to load. Must match the id used in `list_products` / `get_product_prompt`.".to_string(),
            required: true,
        }]
    }
}

#[async_trait::async_trait]
impl McpPromptService for ProductPromptHandler {
    async fn execute_prompt(
        &self,
        arguments: &HashMap<String, String>,
    ) -> Result<PromptExecutionResult, String> {
        let product_id = arguments
            .get(ARG_PRODUCT_ID)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| format!("`{}` argument is required", ARG_PRODUCT_ID))?;

        let snapshot = self.app.products.get_snapshot().await;

        if let Some(product) = snapshot.get(product_id) {
            let mut message = String::new();
            message.push_str("Product: ");
            message.push_str(product.id.as_str());
            message.push_str("\n\nDescription: ");
            if product.description.trim().is_empty() {
                message.push_str("(no description set)");
            } else {
                message.push_str(product.description.as_str());
            }
            message.push_str("\n\nPrompt:\n");
            if product.prompt.trim().is_empty() {
                message.push_str("(no prompt set)");
            } else {
                message.push_str(product.prompt.as_str());
            }

            return Ok(PromptExecutionResult {
                description: format!("Context for product '{}'.", product.id),
                message,
            });
        }

        Ok(PromptExecutionResult {
            description: format!("No explicit context recorded for product '{}'.", product_id),
            message: format!(
                "Product '{}' has no explicit description or prompt recorded. \
                 It exists implicitly through its secrets/templates only. \
                 Ask the user for context, or call `upsert_product` once you know what to record.",
                product_id
            ),
        })
    }
}
