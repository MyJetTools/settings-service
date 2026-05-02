use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, models::ProductId};

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct GetSecretInfoInputData {
    #[property(description: "Product identifier the secret belongs to. Use \"Shared\" for shared secrets.")]
    pub product_id: String,

    #[property(description: "Secret identifier within the product.")]
    pub secret_id: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct GetSecretInfoResponse {
    #[property(description: "Echoes the product_id of the located secret.")]
    pub product_id: String,

    #[property(description: "Secret identifier.")]
    pub secret_id: String,

    #[property(description: "The secret value as stored for the root datacenter.")]
    pub secret_value: String,

    #[property(description: "Human-readable description of the secret. None when no description has been set.")]
    pub secret_description: Option<String>,

    #[property(description: "True when the secret has a non-empty remote-datacenter variant (i.e. a value that overrides the root one when rendered for non-root datacenters). The remote value itself is intentionally not returned.")]
    pub has_remote_value: bool,

    #[property(description: "Permission level of the secret.")]
    pub level: u8,
}

pub struct GetSecretInfoHandler {
    app: Arc<AppContext>,
}

impl GetSecretInfoHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for GetSecretInfoHandler {
    const FUNC_NAME: &'static str = "get_secret_info";
    const DESCRIPTION: &'static str = "Read a single secret by product_id + secret_id. Returns the value, the human-readable description, the permission level, and a boolean flag indicating whether a remote-datacenter variant exists (without exposing the remote value itself).";
}

#[async_trait::async_trait]
impl McpToolCall<GetSecretInfoInputData, GetSecretInfoResponse> for GetSecretInfoHandler {
    async fn execute_tool_call(
        &self,
        model: GetSecretInfoInputData,
    ) -> Result<GetSecretInfoResponse, String> {
        let product_id_input = model.product_id.trim();
        let secret_id = model.secret_id.trim();

        if product_id_input.is_empty() {
            return Err("`product_id` must not be empty (use \"Shared\" for shared secrets)".to_string());
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
        let item = snapshot.get_by_id(lookup_product, secret_id).ok_or_else(|| {
            format!(
                "Secret {}/{} not found",
                product_id_input, secret_id
            )
        })?;

        let has_remote_value = item
            .remote_value
            .as_ref()
            .map(|c| !c.as_str().is_empty())
            .unwrap_or(false);

        Ok(GetSecretInfoResponse {
            product_id: product_id_input.to_string(),
            secret_id: item.id.clone(),
            secret_value: item.content.to_string(),
            secret_description: item.description.clone(),
            has_remote_value,
            level: item.level,
        })
    }
}
