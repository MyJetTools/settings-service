use std::sync::Arc;

use mcp_server_middleware::McpToolCall;
use my_ai_agent::{macros::ApplyJsonSchema, ToolDefinition};
use serde::{Deserialize, Serialize};

use crate::{app_ctx::AppContext, caches::SecretsSnapshot, models::ProductId};

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

        let same_scope = if from_is_shared && to_is_shared {
            true
        } else if !from_is_shared && !to_is_shared {
            from_input == to_input
        } else {
            false
        };
        if same_scope {
            return Err(
                "`from_product_id` and `to_product_id` resolve to the same scope — nothing to move"
                    .to_string(),
            );
        }

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

        let snapshot = self.app.secrets.get_snapshot().await;

        // The source secret must exist; clone it so we can keep using the snapshot.
        let item = snapshot
            .get_by_id(from_product, secret_id)
            .ok_or_else(|| {
                format!(
                    "Secret {}/{} not found in the source scope",
                    from_input, secret_id
                )
            })?
            .clone();

        // Never overwrite an existing secret in the target scope.
        if snapshot.has_secret(to_product, secret_id) {
            return Err(format!(
                "Secret {}/{} already exists in the target scope — move refused (this call never overwrites). Resolve the conflict first.",
                to_input, secret_id
            ));
        }

        let ctx = MoveImpactCtx {
            snapshot: snapshot.as_ref(),
            secret_id,
            from_is_shared,
            from_input,
            to_is_shared,
            to_input,
        };

        // 1. Dependencies of the moved secret that stop resolving after the move.
        let mut broken_dependencies: Vec<String> = Vec::new();
        for dep in item.content.get_secrets() {
            if dep == secret_id {
                continue;
            }
            if broken_dependencies.iter().any(|d| d == dep) {
                continue;
            }
            let resolved_before = ctx.dep_reachable(dep, from_product);
            let resolved_after = ctx.dep_reachable(dep, to_product);
            if resolved_before && !resolved_after {
                broken_dependencies.push(dep.to_string());
            }
        }

        // 2. Consumers (secrets + templates) that stop resolving this secret after the move.
        let mut broken_consumers: Vec<MoveConsumerEntry> = Vec::new();

        for shared_item in snapshot.shared.iter() {
            if from_is_shared && shared_item.id == secret_id {
                continue; // the secret being moved is not a consumer of itself
            }
            if shared_item.content.has_the_secret_inside(secret_id)
                && ctx.consumer_breaks(ProductId::Shared)
            {
                broken_consumers.push(MoveConsumerEntry {
                    scope: SHARED_LITERAL.to_string(),
                    kind: "Secret".to_string(),
                    id: shared_item.id.clone(),
                });
            }
        }

        for (product_id, items) in snapshot.by_product.iter() {
            let consumer_scope = ProductId::Id(product_id.as_str());
            let breaks = ctx.consumer_breaks(consumer_scope);
            for product_item in items.iter() {
                if !from_is_shared && product_id == from_input && product_item.id == secret_id {
                    continue; // the secret being moved is not a consumer of itself
                }
                if breaks && product_item.content.has_the_secret_inside(secret_id) {
                    broken_consumers.push(MoveConsumerEntry {
                        scope: product_id.clone(),
                        kind: "Secret".to_string(),
                        id: product_item.id.clone(),
                    });
                }
            }
        }

        let template_consumers = self
            .app
            .templates
            .find_into_vec(|product_id, template| {
                if template.content.has_the_secret_inside(secret_id) {
                    Some((product_id.to_string(), template.id.clone()))
                } else {
                    None
                }
            })
            .await;

        for (template_product, template_id) in template_consumers {
            let consumer_scope: ProductId = template_product.as_str().into();
            if ctx.consumer_breaks(consumer_scope) {
                let scope_display = if template_product.is_empty() {
                    SHARED_LITERAL.to_string()
                } else {
                    template_product
                };
                broken_consumers.push(MoveConsumerEntry {
                    scope: scope_display,
                    kind: "Template".to_string(),
                    id: template_id,
                });
            }
        }

        let would_break = !broken_dependencies.is_empty() || !broken_consumers.is_empty();

        if would_break && !force {
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

        drop(snapshot);

        crate::flows::move_secret(self.app.as_ref(), from_product, to_product, item).await;

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

/// Helper that answers reachability questions about the move without mutating
/// anything. The move only ever relocates the single subject secret, so the
/// "after" state is the current snapshot with that one id removed from the
/// source scope and added to the target scope.
struct MoveImpactCtx<'a> {
    snapshot: &'a SecretsSnapshot,
    secret_id: &'a str,
    from_is_shared: bool,
    from_input: &'a str,
    to_is_shared: bool,
    to_input: &'a str,
}

impl<'a> MoveImpactCtx<'a> {
    fn is_target(&self, scope: ProductId<'_>) -> bool {
        match scope {
            ProductId::Shared => self.to_is_shared,
            ProductId::Id(p) => !self.to_is_shared && p == self.to_input,
        }
    }

    fn is_source(&self, scope: ProductId<'_>) -> bool {
        match scope {
            ProductId::Shared => self.from_is_shared,
            ProductId::Id(p) => !self.from_is_shared && p == self.from_input,
        }
    }

    /// Is the subject secret present in `scope`, either now (`after = false`) or
    /// once the move has happened (`after = true`)?
    fn subject_present(&self, scope: ProductId<'_>, after: bool) -> bool {
        if after {
            if self.is_target(scope) {
                return true;
            }
            if self.is_source(scope) {
                return false;
            }
        }
        self.snapshot.has_secret(scope, self.secret_id)
    }

    /// Can a consumer living in `consumer` scope resolve the subject secret
    /// (its own scope first, then the Shared fallback for product scopes)?
    fn subject_reachable(&self, consumer: ProductId<'_>, after: bool) -> bool {
        match consumer {
            ProductId::Shared => self.subject_present(ProductId::Shared, after),
            ProductId::Id(_) => {
                self.subject_present(consumer, after)
                    || self.subject_present(ProductId::Shared, after)
            }
        }
    }

    fn consumer_breaks(&self, consumer: ProductId<'_>) -> bool {
        self.subject_reachable(consumer, false) && !self.subject_reachable(consumer, true)
    }

    /// Can a dependency `dep` (a different secret, unaffected by the move) be
    /// resolved from `scope`? Used to check the moved secret's own `${...}`
    /// placeholders against the source vs. the target scope.
    fn dep_reachable(&self, dep: &str, scope: ProductId<'_>) -> bool {
        match scope {
            ProductId::Shared => self.snapshot.has_secret(ProductId::Shared, dep),
            ProductId::Id(_) => {
                self.snapshot.has_secret(scope, dep)
                    || self.snapshot.has_secret(ProductId::Shared, dep)
            }
        }
    }
}
