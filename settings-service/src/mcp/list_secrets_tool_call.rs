use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, models::SecretItem};

const SHARED_LITERAL: &str = "Shared";

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct ListSecretsInputData {
    #[property(description: "Product identifier to list secrets for. Use \"Shared\" to list shared secrets only.")]
    pub product_id: String,

    #[property(description: "When true and product_id is not \"Shared\", shared secrets are merged into the listing as well. Ignored for \"Shared\".")]
    pub include_shared: Option<bool>,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct SecretListEntry {
    #[property(description: "Product the secret belongs to. \"Shared\" indicates a shared secret.")]
    pub product_id: String,

    #[property(description: "Secret identifier.")]
    pub secret_id: String,

    #[property(description: "Human-readable description of the secret. None when no description has been set.")]
    pub secret_description: Option<String>,

    #[property(description: "True when the secret has a non-empty remote-datacenter variant.")]
    pub has_remote_value: bool,

    #[property(description: "Permission level of the secret.")]
    pub level: u8,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct ListSecretsResponse {
    #[property(description: "Total number of secrets returned.")]
    pub count: i64,

    #[property(description: "Secrets matching the query. Values are intentionally not included; use `get_secret_info` to read a specific value.")]
    pub secrets: Vec<SecretListEntry>,
}

pub struct ListSecretsHandler {
    app: Arc<AppContext>,
}

impl ListSecretsHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for ListSecretsHandler {
    const FUNC_NAME: &'static str = "list_secrets";
    const DESCRIPTION: &'static str = "Browse the directory of secrets for a product (with their human-readable descriptions, levels, and a flag for whether a remote-datacenter variant exists). Secret VALUES are not returned by this call — read a specific value with `get_secret_info`. Use \"Shared\" as `product_id` to list shared secrets, or pass `include_shared: true` to merge shared secrets into a per-product listing.";
}

#[async_trait::async_trait]
impl McpToolCall<ListSecretsInputData, ListSecretsResponse> for ListSecretsHandler {
    async fn execute_tool_call(
        &self,
        model: ListSecretsInputData,
    ) -> Result<ListSecretsResponse, String> {
        let product_id_input = model.product_id.trim();
        if product_id_input.is_empty() {
            return Err("`product_id` must not be empty (use \"Shared\" to list shared secrets)".to_string());
        }
        let is_shared_query = product_id_input.eq_ignore_ascii_case(SHARED_LITERAL);
        let include_shared = model.include_shared.unwrap_or(false);

        let snapshot = self.app.secrets.get_snapshot().await;

        let mut secrets: Vec<SecretListEntry> = Vec::new();

        if is_shared_query {
            for item in snapshot.shared.iter() {
                secrets.push(to_entry(SHARED_LITERAL.to_string(), item));
            }
        } else {
            if let Some(by_product) = snapshot.by_product.get(product_id_input) {
                for item in by_product.iter() {
                    secrets.push(to_entry(product_id_input.to_string(), item));
                }
            }
            if include_shared {
                for item in snapshot.shared.iter() {
                    secrets.push(to_entry(SHARED_LITERAL.to_string(), item));
                }
            }
        }

        let count = secrets.len() as i64;
        Ok(ListSecretsResponse { count, secrets })
    }
}

fn to_entry(product_id: String, item: &SecretItem) -> SecretListEntry {
    let has_remote_value = item
        .remote_value
        .as_ref()
        .map(|c| !c.as_str().is_empty())
        .unwrap_or(false);

    SecretListEntry {
        product_id,
        secret_id: item.id.clone(),
        secret_description: item.description.clone(),
        has_remote_value,
        level: item.level,
    }
}
