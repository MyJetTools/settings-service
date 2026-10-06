use crate::{
    app_ctx::AppContext,
    caches::SecretsSnapshot,
    models::{Content, ProductId, SecretItem},
};

/// Moves a secret from one scope to another, preserving every field of the
/// secret (value, remote value, level, description, mcp visibility and the
/// original `created`/`updated` timestamps).
///
/// `from` and `to` MUST be different scopes. The caller is responsible for the
/// pre-flight checks (the source secret exists and the target scope does not
/// already hold a secret with the same id).
pub async fn move_secret(
    app: &AppContext,
    from: ProductId<'_>,
    to: ProductId<'_>,
    item: SecretItem,
) {
    // Relocate under a single write lock so the secret is never observable in
    // both scopes, then persist the resulting snapshot once.
    let snapshot = app.secrets.move_secret(from, to, item).await;

    app.secrets_persistence.save(&snapshot).await;
}

#[derive(Debug, Clone, Copy)]
pub enum MoveSecretConsumerKind {
    Secret,
    Template,
}

impl MoveSecretConsumerKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Secret => "Secret",
            Self::Template => "Template",
        }
    }
}

/// A secret or a template that references the moved secret and can no longer
/// resolve it after the move.
#[derive(Debug)]
pub struct MoveSecretBrokenConsumer {
    /// `None` when the consumer lives in the Shared scope.
    pub product_id: Option<String>,
    pub kind: MoveSecretConsumerKind,
    pub id: String,
}

/// The `${...}` references a move leaves unresolved.
#[derive(Debug, Default)]
pub struct MoveSecretImpact {
    /// Dependencies of the moved secret (`${...}` placeholders inside its value)
    /// that resolved in the source scope but no longer resolve in the target scope.
    pub broken_dependencies: Vec<String>,
    /// Other secrets/templates that referenced the moved secret and can no longer
    /// resolve it.
    pub broken_consumers: Vec<MoveSecretBrokenConsumer>,
}

impl MoveSecretImpact {
    pub fn breaks_references(&self) -> bool {
        !self.broken_dependencies.is_empty() || !self.broken_consumers.is_empty()
    }
}

#[derive(Debug)]
pub enum MoveSecretError {
    /// `from` and `to` are the same scope - nothing to move.
    SameScope,
    /// The secret does not exist in the source scope.
    NotFound,
    /// The target scope already holds a secret with this id.
    AlreadyExists,
    /// The move would break `${...}` references and `force` was not set.
    BreaksReferences(MoveSecretImpact),
}

/// Runs the pre-flight checks and moves the secret from `from` to `to`.
///
/// Never overwrites: the move is refused when the target scope already holds a
/// secret with the same id. Unless `force` is set, it is also refused when it
/// would break a `${...}` reference - either a dependency the moved secret
/// itself needs, or another secret/template that consumes it. The returned
/// impact lists the references a forced move has left unresolved.
pub async fn try_move_secret(
    app: &AppContext,
    secret_id: &str,
    from: ProductId<'_>,
    to: ProductId<'_>,
    force: bool,
) -> Result<MoveSecretImpact, MoveSecretError> {
    if is_same_scope(from, to) {
        return Err(MoveSecretError::SameScope);
    }

    let snapshot = app.secrets.get_snapshot().await;

    // The source secret must exist; clone it so we can keep using the snapshot.
    let Some(item) = snapshot.get_by_id(from, secret_id) else {
        return Err(MoveSecretError::NotFound);
    };
    let item = item.clone();

    // Never overwrite an existing secret in the target scope.
    if snapshot.has_secret(to, secret_id) {
        return Err(MoveSecretError::AlreadyExists);
    }

    let impact = get_move_impact(app, snapshot.as_ref(), &item, from, to).await;

    if impact.breaks_references() && !force {
        return Err(MoveSecretError::BreaksReferences(impact));
    }

    drop(snapshot);

    move_secret(app, from, to, item).await;

    Ok(impact)
}

async fn get_move_impact(
    app: &AppContext,
    snapshot: &SecretsSnapshot,
    item: &SecretItem,
    from: ProductId<'_>,
    to: ProductId<'_>,
) -> MoveSecretImpact {
    let secret_id = item.id.as_str();

    let ctx = MoveImpactCtx {
        snapshot,
        secret_id,
        from,
        to,
    };

    let mut result = MoveSecretImpact::default();

    // 1. Dependencies of the moved secret that stop resolving after the move.
    for dep in item.content.get_secrets() {
        if dep == secret_id {
            continue;
        }
        if result.broken_dependencies.iter().any(|d| d == dep) {
            continue;
        }
        let resolved_before = ctx.dep_reachable(dep, from);
        let resolved_after = ctx.dep_reachable(dep, to);
        if resolved_before && !resolved_after {
            result.broken_dependencies.push(dep.to_string());
        }
    }

    // 2. Consumers (secrets + templates) that stop resolving this secret after the move.
    for shared_item in snapshot.shared.iter() {
        if is_same_scope(ProductId::Shared, from) && shared_item.id == secret_id {
            continue; // the secret being moved is not a consumer of itself
        }
        if references_secret(&shared_item.content, secret_id)
            && ctx.consumer_breaks(ProductId::Shared)
        {
            result.broken_consumers.push(MoveSecretBrokenConsumer {
                product_id: None,
                kind: MoveSecretConsumerKind::Secret,
                id: shared_item.id.clone(),
            });
        }
    }

    for (product_id, items) in snapshot.by_product.iter() {
        let consumer_scope = ProductId::Id(product_id.as_str());
        let breaks = ctx.consumer_breaks(consumer_scope);
        for product_item in items.iter() {
            if is_same_scope(consumer_scope, from) && product_item.id == secret_id {
                continue; // the secret being moved is not a consumer of itself
            }
            if breaks && references_secret(&product_item.content, secret_id) {
                result.broken_consumers.push(MoveSecretBrokenConsumer {
                    product_id: Some(product_id.clone()),
                    kind: MoveSecretConsumerKind::Secret,
                    id: product_item.id.clone(),
                });
            }
        }
    }

    let template_consumers = app
        .templates
        .find_into_vec(|product_id, template| {
            if references_secret(&template.content, secret_id) {
                Some((product_id.to_string(), template.id.clone()))
            } else {
                None
            }
        })
        .await;

    for (template_product, template_id) in template_consumers {
        let consumer_scope: ProductId = template_product.as_str().into();
        if ctx.consumer_breaks(consumer_scope) {
            result.broken_consumers.push(MoveSecretBrokenConsumer {
                product_id: if template_product.is_empty() {
                    None
                } else {
                    Some(template_product)
                },
                kind: MoveSecretConsumerKind::Template,
                id: template_id,
            });
        }
    }

    result
}

/// Does `content` actually reference `secret_id` as a `${secret_id}` placeholder?
///
/// Uses the SAME placeholder parser as secret resolution (`Content::get_secrets`),
/// not a naive substring test — so adversarial/nested content such as `${a${b}`
/// (which parses as a single placeholder `a${b`, not a reference to `b`) is not
/// mis-detected as a consumer.
fn references_secret(content: &Content, secret_id: &str) -> bool {
    content.get_secrets().iter().any(|name| *name == secret_id)
}

fn is_same_scope(left: ProductId<'_>, right: ProductId<'_>) -> bool {
    match (left, right) {
        (ProductId::Shared, ProductId::Shared) => true,
        (ProductId::Id(left), ProductId::Id(right)) => left == right,
        _ => false,
    }
}

/// Helper that answers reachability questions about the move without mutating
/// anything. The move only ever relocates the single subject secret, so the
/// "after" state is the current snapshot with that one id removed from the
/// source scope and added to the target scope.
struct MoveImpactCtx<'a> {
    snapshot: &'a SecretsSnapshot,
    secret_id: &'a str,
    from: ProductId<'a>,
    to: ProductId<'a>,
}

impl<'a> MoveImpactCtx<'a> {
    /// Is the subject secret present in `scope`, either now (`after = false`) or
    /// once the move has happened (`after = true`)?
    fn subject_present(&self, scope: ProductId<'_>, after: bool) -> bool {
        if after {
            if is_same_scope(scope, self.to) {
                return true;
            }
            if is_same_scope(scope, self.from) {
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
