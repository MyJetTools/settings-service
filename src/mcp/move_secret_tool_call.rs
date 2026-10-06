use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, flows::MoveSecretError, models::ProductId};

const SHARED_LITERAL: &str = "Shared";

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct MoveSecretInputData {
    #[property(description: "Identifier of the secret to move. The id is preserved across the move; only the scope changes.")]
    pub secret_id: String,

    #[property(description: "Scope the secret currently lives in. Use \"Shared\" for the shared scope, or a product id for a product-scoped secret.")]
    pub from_product_id: String,

    #[property(description: "Scope to move the secret into. Use \"Shared\" to promote it to the shared scope, or a product id to scope it to a single product. Must differ from `from_product_id`.")]
    pub to_product_id: String,

    #[property(description: "When false (default) the move is REFUSED if it would break any `${...}` reference — either a dependency the moved secret itself needs, or another secret/template that consumes it. Set true to move anyway, knowingly leaving those references unresolved.")]
    pub force: Option<bool>,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct MoveConsumerEntry {
    #[property(description: "Scope of the consumer that references the moved secret (\"Shared\" or a product id).")]
    pub scope: String,

    #[property(description: "Kind of the consumer: \"Secret\" or \"Template\".")]
    pub kind: String,

    #[property(description: "Identifier of the consuming secret or template.")]
    pub id: String,
}

#[derive(ApplyJsonSchema, Debug, Serialize, Deserialize)]
pub struct MoveSecretResponse {
    #[property(description: "Echoes the moved secret_id.")]
    pub secret_id: String,

    #[property(description: "Scope the secret was moved from.")]
    pub from_product_id: String,

    #[property(description: "Scope the secret was moved to.")]
    pub to_product_id: String,

    #[property(description: "True when the secret now lives in `to_product_id` and was removed from `from_product_id`.")]
    pub moved: bool,

    #[property(description: "True when the move was performed despite breaking references because `force` was set.")]
    pub forced: bool,

    #[property(description: "Dependencies of the moved secret (`${...}` placeholders inside its value) that resolved in the source scope but no longer resolve in the target scope after the move.")]
    pub broken_dependencies: Vec<String>,

    #[property(description: "Other secrets/templates that referenced this secret and can no longer resolve it after the move.")]
    pub broken_consumers: Vec<MoveConsumerEntry>,
}

pub struct MoveSecretHandler {
    app: Arc<AppContext>,
}

impl MoveSecretHandler {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

impl ToolDefinition for MoveSecretHandler {
    const FUNC_NAME: &'static str = "move_secret";
    const DESCRIPTION: &'static str = "Move a secret between scopes (e.g. promote a product-scoped secret to \"Shared\", or pull a shared secret down into a single product). The secret keeps its id and all of its fields (value, remote value, level, description, visibility, timestamps) — only the scope changes. Fails if the source secret does not exist, if the source and target scope are the same, or if the target scope already holds a secret with that id (this call never overwrites). By default the move is also REFUSED when it would break a `${...}` reference — either a dependency the moved secret needs, or another secret/template that consumes it; pass `force: true` to override. Use \"Shared\" as a scope to mean the shared scope.";
}

#[async_trait::async_trait]
impl McpToolCall<MoveSecretInputData, MoveSecretResponse> for MoveSecretHandler {
    async fn execute_tool_call(
        &self,
        model: MoveSecretInputData,
    ) -> Result<MoveSecretResponse, String> {
        let secret_id = model.secret_id.trim();
        let from_input = model.from_product_id.trim();
        let to_input = model.to_product_id.trim();
        let force = model.force.unwrap_or(false);

        if secret_id.is_empty() {
            return Err("`secret_id` must not be empty".to_string());
        }
        if from_input.is_empty() {
            return Err(
                "`from_product_id` must not be empty (use \"Shared\" for the shared scope)"
                    .to_string(),
            );
        }
        if to_input.is_empty() {
            return Err(
                "`to_product_id` must not be empty (use \"Shared\" for the shared scope)".to_string(),
            );
        }

        let from_is_shared = from_input.eq_ignore_ascii_case(SHARED_LITERAL);
        let to_is_shared = to_input.eq_ignore_ascii_case(SHARED_LITERAL);

        let from_product: ProductId = if from_is_shared {
            ProductId::Shared
        } else {
            ProductId::Id(from_input)
        };
        let to_product: ProductId = if to_is_shared {
            ProductId::Shared
        } else {
            ProductId::Id(to_input)
        };

        let result = crate::flows::try_move_secret(
            self.app.as_ref(),
            secret_id,
            from_product,
            to_product,
            force,
        )
        .await;

        let (impact, moved) = match result {
            Ok(impact) => (impact, true),
            Err(MoveSecretError::BreaksReferences(impact)) => (impact, false),
            Err(MoveSecretError::SameScope) => {
                return Err(
                    "`from_product_id` and `to_product_id` resolve to the same scope — nothing to move"
                        .to_string(),
                );
            }
            Err(MoveSecretError::NotFound) => {
                return Err(format!(
                    "Secret {}/{} not found in the source scope",
                    from_input, secret_id
                ));
            }
            Err(MoveSecretError::AlreadyExists) => {
                return Err(format!(
                    "Secret {}/{} already exists in the target scope — move refused (this call never overwrites). Resolve the conflict first.",
                    to_input, secret_id
                ));
            }
        };

        let would_break = impact.breaks_references();
        let broken_dependencies = impact.broken_dependencies;
        let broken_consumers: Vec<MoveConsumerEntry> = impact
            .broken_consumers
            .into_iter()
            .map(|consumer| MoveConsumerEntry {
                scope: consumer
                    .product_id
                    .unwrap_or_else(|| SHARED_LITERAL.to_string()),
                kind: consumer.kind.as_str().to_string(),
                id: consumer.id,
            })
            .collect();

        if !moved {
            let mut msg = format!(
                "Refusing to move secret {}/{} to {} because it would break references:\n",
                from_input, secret_id, to_input
            );
            if !broken_dependencies.is_empty() {
                msg.push_str(&format!(
                    "- depends on {} secret(s) that will no longer resolve in the target scope: {}\n",
                    broken_dependencies.len(),
                    broken_dependencies.join(", ")
                ));
            }
            if !broken_consumers.is_empty() {
                let list = broken_consumers
                    .iter()
                    .map(|c| format!("{} {}/{}", c.kind, c.scope, c.id))
                    .collect::<Vec<_>>()
                    .join(", ");
                msg.push_str(&format!(
                    "- {} consumer(s) will no longer resolve this secret: {}\n",
                    broken_consumers.len(),
                    list
                ));
            }
            msg.push_str(
                "Re-call with `force: true` to move anyway (the broken references will be left unresolved).",
            );
            return Err(msg);
        }

        Ok(MoveSecretResponse {
            secret_id: secret_id.to_string(),
            from_product_id: if from_is_shared {
                SHARED_LITERAL.to_string()
            } else {
                from_input.to_string()
            },
            to_product_id: if to_is_shared {
                SHARED_LITERAL.to_string()
            } else {
                to_input.to_string()
            },
            moved: true,
            forced: would_break,
            broken_dependencies,
            broken_consumers,
        })
    }
}
