use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, models::ProductId};

const SHARED_LITERAL: &str = "Shared";

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct GetSecretDependenciesInputData {
    #[property(description: "Product identifier the secret belongs to. Use \"Shared\" for shared secrets.")]
    pub product_id: String,

    #[property(description: "Secret identifier to inspect.")]
    pub secret_id: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct SecretDependencyEntry {
    #[property(description: "The referenced secret_id (the placeholder name found inside the value).")]
    pub secret_id: String,

    #[property(description: "Where the dependency was resolved: \"Product\" — found in the same product, \"Shared\" — found in the Shared scope, \"Missing\" — not found anywhere.")]
    pub resolved_from: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct GetSecretDependenciesResponse {
    #[property(description: "Echoes the product_id of the inspected secret.")]
    pub product_id: String,

    #[property(description: "Echoes the secret_id of the inspected secret.")]
    pub secret_id: String,

    #[property(description: "Total number of dependencies (deduplicated).")]
    pub count: i64,

    #[property(description: "List of secrets the inspected secret depends on (i.e. `${...}` placeholders inside its value, deduplicated, in order of first appearance). The actual values are intentionally NOT returned — only the names and where each name was resolved from.")]
    pub dependencies: Vec<SecretDependencyEntry>,

    #[property(description: "True when at least one dependency could not be resolved from the product or from Shared.")]
    pub has_missing: bool,
}

pub struct GetSecretDependenciesHandler {
    app: Arc<AppContext>,
}

impl GetSecretDependenciesHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for GetSecretDependenciesHandler {
    const FUNC_NAME: &'static str = "get_secret_dependencies";
    const DESCRIPTION: &'static str = "Inspect a single secret and return the list of OTHER secrets it depends on (the `${other_secret}` placeholders inside its value). Each dependency is reported with where it was resolved from: the same product, the Shared scope, or Missing if it does not exist. The secret's actual value is never returned — this tool only exposes the dependency graph, not contents.";
}

#[async_trait::async_trait]
impl McpToolCall<GetSecretDependenciesInputData, GetSecretDependenciesResponse>
    for GetSecretDependenciesHandler
{
    async fn execute_tool_call(
        &self,
        model: GetSecretDependenciesInputData,
    ) -> Result<GetSecretDependenciesResponse, String> {
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

        let lookup_product: ProductId = if product_id_input.eq_ignore_ascii_case(SHARED_LITERAL) {
            ProductId::Shared
        } else {
            ProductId::Id(product_id_input)
        };

        let snapshot = self.app.secrets.get_snapshot().await;
        let item = snapshot
            .get_by_id(lookup_product, secret_id)
            .ok_or_else(|| format!("Secret {}/{} not found", product_id_input, secret_id))?;

        let mut seen: Vec<String> = Vec::new();
        let mut dependencies: Vec<SecretDependencyEntry> = Vec::new();
        let mut has_missing = false;

        for placeholder in item.content.get_secrets() {
            if seen.iter().any(|s| s == placeholder) {
                continue;
            }
            seen.push(placeholder.to_string());

            let in_product = match lookup_product {
                ProductId::Shared => false,
                ProductId::Id(_) => snapshot.has_secret(lookup_product, placeholder),
            };
            let in_shared = snapshot.has_secret(ProductId::Shared, placeholder);

            let resolved_from = if in_product {
                "Product"
            } else if in_shared {
                "Shared"
            } else {
                has_missing = true;
                "Missing"
            };

            dependencies.push(SecretDependencyEntry {
                secret_id: placeholder.to_string(),
                resolved_from: resolved_from.to_string(),
            });
        }

        Ok(GetSecretDependenciesResponse {
            product_id: product_id_input.to_string(),
            secret_id: secret_id.to_string(),
            count: dependencies.len() as i64,
            dependencies,
            has_missing,
        })
    }
}
