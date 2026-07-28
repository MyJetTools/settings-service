use crate::{
    app_ctx::AppContext,
    models::{ProductId, SecretItem},
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
