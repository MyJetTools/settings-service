use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, models::ProductId};

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct UpsertSecretDescriptionInputData {
    #[property(description: "Product identifier the secret belongs to. Use \"Shared\" for shared secrets.")]
    pub product_id: String,

    #[property(description: "Secret identifier within the product. The secret must already exist; this call only updates its description, not its value.")]
    pub secret_id: String,

    #[property(description: "New human-readable description for the secret. Pass an empty string to clear the existing description.")]
    pub description: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct UpsertSecretDescriptionResponse {
    #[property(description: "Echoes the product_id of the updated secret.")]
    pub product_id: String,

    #[property(description: "Echoes the secret_id of the updated secret.")]
    pub secret_id: String,

    #[property(description: "The description that is now stored. None when the description has been cleared.")]
    pub description: Option<String>,
}

pub struct UpsertSecretDescriptionHandler {
    app: Arc<AppContext>,
}

impl UpsertSecretDescriptionHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for UpsertSecretDescriptionHandler {
    const FUNC_NAME: &'static str = "upsert_secret_description";
    const DESCRIPTION: &'static str = "Update the human-readable description of an existing secret identified by product_id + secret_id. The secret's value, remote-datacenter variant, and permission level are preserved unchanged. Pass an empty string in `description` to clear the description. Fails if the secret does not exist — this call never creates a new secret.";
}

#[async_trait::async_trait]
impl McpToolCall<UpsertSecretDescriptionInputData, UpsertSecretDescriptionResponse>
    for UpsertSecretDescriptionHandler
{
    async fn execute_tool_call(
        &self,
        model: UpsertSecretDescriptionInputData,
    ) -> Result<UpsertSecretDescriptionResponse, String> {
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
        let existing = snapshot
            .get_by_id(lookup_product, secret_id)
            .ok_or_else(|| format!("Secret {}/{} not found", product_id_input, secret_id))?;

        let secret_value = existing.content.to_string();
        let remote_value = existing
            .remote_value
            .as_ref()
            .map(|c| c.to_string())
            .filter(|v| !v.is_empty());
        let level = existing.level;
        drop(snapshot);

        crate::flows::save_secret(
            self.app.as_ref(),
            lookup_product,
            secret_id.to_string(),
            secret_value,
            remote_value,
            level,
            Some(model.description),
            None,
        )
        .await;

        let snapshot = self.app.secrets.get_snapshot().await;
        let stored_description = snapshot
            .get_by_id(lookup_product, secret_id)
            .and_then(|item| item.description.clone());

        Ok(UpsertSecretDescriptionResponse {
            product_id: product_id_input.to_string(),
            secret_id: secret_id.to_string(),
            description: stored_description,
        })
    }
}
