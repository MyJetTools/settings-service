use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, models::ProductId};

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct GetSecretValueLenInputData {
    #[property(description: "Product identifier the secret belongs to. Use \"Shared\" for shared secrets.")]
    pub product_id: String,

    #[property(description: "Secret identifier to measure.")]
    pub secret_id: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct GetSecretValueLenResponse {
    #[property(description: "Echoes the product_id of the inspected secret.")]
    pub product_id: String,

    #[property(description: "Echoes the secret_id of the inspected secret.")]
    pub secret_id: String,

    #[property(description: "Number of characters of the stored root value. This is the raw stored length: when the value contains `${other_secret}` placeholders it counts the placeholders themselves, not what they resolve to.")]
    pub value_len: i64,

    #[property(description: "True when a non-empty remote-datacenter variant exists.")]
    pub has_remote_value: bool,

    #[property(description: "Number of characters of the remote-datacenter variant. None when the secret has no remote variant.")]
    pub remote_value_len: Option<i64>,

    #[property(description: "Whether the value itself can be read via `get_secret_value`. False means you can only see this length, never the content.")]
    pub visible_for_mcp: bool,
}

pub struct GetSecretValueLenHandler {
    app: Arc<AppContext>,
}

impl GetSecretValueLenHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for GetSecretValueLenHandler {
    const FUNC_NAME: &'static str = "get_secret_value_len";
    const DESCRIPTION: &'static str = "Measure the length of a secret's value without reading it. Unlike `get_secret_value`, this works for EVERY existing secret, including the private ones (`visible_for_mcp = false`) — a character count reveals nothing about the content. Use it to verify that a secret actually holds something plausible: that a generated token has the expected number of characters, that a value is not a one-character placeholder left behind by a human, or that a rotation really replaced a short value with a long one. Fails only when the secret does not exist.";
}

#[async_trait::async_trait]
impl McpToolCall<GetSecretValueLenInputData, GetSecretValueLenResponse>
    for GetSecretValueLenHandler
{
    async fn execute_tool_call(
        &self,
        model: GetSecretValueLenInputData,
    ) -> Result<GetSecretValueLenResponse, String> {
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

        let value_len = item.content.as_str().chars().count() as i64;

        let remote_value_len = item
            .remote_value
            .as_ref()
            .map(|c| c.as_str().chars().count() as i64)
            .filter(|len| *len > 0);

        Ok(GetSecretValueLenResponse {
            product_id: product_id_input.to_string(),
            secret_id: secret_id.to_string(),
            value_len,
            has_remote_value: remote_value_len.is_some(),
            remote_value_len,
            visible_for_mcp: item.visible_for_mcp,
        })
    }
}
