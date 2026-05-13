use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, models::ProductId};

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct GetSecretValueInputData {
    #[property(description: "Product identifier the secret belongs to. Use \"Shared\" for shared secrets.")]
    pub product_id: String,

    #[property(description: "Secret identifier to read.")]
    pub secret_id: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct GetSecretValueResponse {
    #[property(description: "Echoes the product_id of the inspected secret.")]
    pub product_id: String,

    #[property(description: "Echoes the secret_id of the inspected secret.")]
    pub secret_id: String,

    #[property(description: "The secret's stored value, returned as plain text.")]
    pub secret_value: String,

    #[property(description: "Human-readable description of the secret. None when no description has been set.")]
    pub description: Option<String>,

    #[property(description: "True when a non-empty remote-datacenter variant exists. The remote variant itself is never returned by this tool — it is considered more sensitive than the root value.")]
    pub has_remote_value: bool,
}

pub struct GetSecretValueHandler {
    app: Arc<AppContext>,
}

impl GetSecretValueHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for GetSecretValueHandler {
    const FUNC_NAME: &'static str = "get_secret_value";
    const DESCRIPTION: &'static str = "Read the stored value of a secret. Only works for secrets that a human has explicitly marked as `visible_for_mcp = true` — for any other secret this call returns an error. Inspect `list_secrets` first and pick the `visible_for_mcp: true` entries. Even when readable, the remote-datacenter variant is never returned, only the root value.";
}

#[async_trait::async_trait]
impl McpToolCall<GetSecretValueInputData, GetSecretValueResponse> for GetSecretValueHandler {
    async fn execute_tool_call(
        &self,
        model: GetSecretValueInputData,
    ) -> Result<GetSecretValueResponse, String> {
        let product_id_input = model.product_id.trim();
        let secret_id = model.secret_id.trim();

        if product_id_input.is_empty() {
            return Err(
                "`product_id` must not be empty (use \"Shared\" for shared secrets)".to_string(),
            );
        }
        if secret_id.is_empty() {
            return Err("`secret_id` must not be empty".to_string());
        }

        let lookup_product: ProductId = if product_id_input.eq_ignore_ascii_case("Shared") {
            ProductId::Shared
        } else {
            ProductId::Id(product_id_input)
        };

        let snapshot = self.app.secrets.get_snapshot().await;
        let item = snapshot
            .get_by_id(lookup_product, secret_id)
            .ok_or_else(|| format!("Secret {}/{} not found", product_id_input, secret_id))?;

        if !item.visible_for_mcp {
            return Err(format!(
                "Secret {}/{} is not marked visible for MCP. A human must enable `visible_for_mcp` on it (via the UI) before its value can be read.",
                product_id_input, secret_id
            ));
        }

        let has_remote_value = item
            .remote_value
            .as_ref()
            .map(|c| !c.as_str().is_empty())
            .unwrap_or(false);

        Ok(GetSecretValueResponse {
            product_id: product_id_input.to_string(),
            secret_id: secret_id.to_string(),
            secret_value: item.content.to_string(),
            description: item.description.clone(),
            has_remote_value,
        })
    }
}
