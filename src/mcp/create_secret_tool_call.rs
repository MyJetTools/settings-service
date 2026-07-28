use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, models::ProductId};

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct CreateSecretInputData {
    #[property(description: "Product identifier the new secret will belong to. Use \"Shared\" to create a shared secret.")]
    pub product_id: String,

    #[property(description: "Identifier for the new secret. Must not already exist in the given product (the call fails otherwise).")]
    pub secret_id: String,

    #[property(description: "Initial value of the secret. Must not be empty. May contain `${other_secret}` placeholders referencing other existing secrets.")]
    pub secret_value: String,

    #[property(description: "Alternative value used when rendering for a non-root (remote) datacenter. Pass empty string to use the same value in all datacenters.")]
    pub remote_value: String,

    #[property(description: "Human-readable description of what the secret is for. Pass empty string if you really have nothing to say (strongly discouraged — descriptions help future agents find the right secret without guessing from the name).")]
    pub description: String,

    #[property(description: "If true, AI can read this secret's value back via `get_secret_value`. Default behaviour (false) keeps the value private; the AI can confirm existence but not read it. Set to true only for non-sensitive configuration values (feature flags, environment names, public URLs), NEVER for credentials.")]
    pub visible_for_mcp: bool,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct CreateSecretResponse {
    #[property(description: "Echoes the product_id of the created secret.")]
    pub product_id: String,

    #[property(description: "Echoes the secret_id of the created secret.")]
    pub secret_id: String,

    #[property(description: "Stored description (None when no description was provided).")]
    pub description: Option<String>,

    #[property(description: "Permission level the secret was created with (defaults to 0; can be changed in the UI).")]
    pub level: i32,

    #[property(description: "True when a non-empty remote-datacenter variant was stored.")]
    pub has_remote_value: bool,

    #[property(description: "Whether the secret is readable via `get_secret_value` (mirrors the input flag).")]
    pub visible_for_mcp: bool,
}

pub struct CreateSecretHandler {
    app: Arc<AppContext>,
}

impl CreateSecretHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for CreateSecretHandler {
    const FUNC_NAME: &'static str = "create_secret";
    const DESCRIPTION: &'static str = "Create a new secret identified by (product_id, secret_id). Fails with an error if a secret with that id already exists in the given product — this call never overwrites an existing secret. To change description of an existing secret use `upsert_secret_description`. The new secret is created with `level = 0` (a human can raise it via the UI). When `visible_for_mcp` is true, future MCP calls can read the value via `get_secret_value`; otherwise the value is private even though existence is observable.";
}

#[async_trait::async_trait]
impl McpToolCall<CreateSecretInputData, CreateSecretResponse> for CreateSecretHandler {
    async fn execute_tool_call(
        &self,
        model: CreateSecretInputData,
    ) -> Result<CreateSecretResponse, String> {
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
        if model.secret_value.is_empty() {
            return Err("`secret_value` must not be empty".to_string());
        }

        let lookup_product: ProductId = if product_id_input.eq_ignore_ascii_case("Shared") {
            ProductId::Shared
        } else {
            ProductId::Id(product_id_input)
        };

        let snapshot = self.app.secrets.get_snapshot().await;
        if snapshot.has_secret(lookup_product, secret_id) {
            return Err(format!(
                "Secret {}/{} already exists. Use `upsert_secret_description` to change its description or edit it via the UI.",
                product_id_input, secret_id
            ));
        }
        drop(snapshot);

        let level = 0u8;
        let remote_value = {
            let trimmed = model.remote_value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        };

        crate::flows::save_secret(
            self.app.as_ref(),
            lookup_product,
            secret_id.to_string(),
            model.secret_value,
            remote_value,
            level,
            Some(model.description),
            Some(model.visible_for_mcp),
        )
        .await;

        let snapshot = self.app.secrets.get_snapshot().await;
        let stored = snapshot
            .get_by_id(lookup_product, secret_id)
            .ok_or_else(|| {
                format!(
                    "Secret {}/{} was not stored — internal error.",
                    product_id_input, secret_id
                )
            })?;

        let has_remote_value = stored
            .remote_value
            .as_ref()
            .map(|c| !c.as_str().is_empty())
            .unwrap_or(false);

        Ok(CreateSecretResponse {
            product_id: product_id_input.to_string(),
            secret_id: secret_id.to_string(),
            description: stored.description.clone(),
            level: stored.level as i32,
            has_remote_value,
            visible_for_mcp: stored.visible_for_mcp,
        })
    }
}
