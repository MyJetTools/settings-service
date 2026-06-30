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
    let secret_id = item.id.clone();

    // Insert into the target scope first (keeping the exact same item) ...
    app.secrets.insert_or_update(to, [item].into_iter()).await;

    // ... then drop it from the source scope.
    app.secrets.remove(from, &secret_id).await;

    let snapshot = app.secrets.get_snapshot().await;
    app.secrets_persistence.save(&snapshot).await;
}
