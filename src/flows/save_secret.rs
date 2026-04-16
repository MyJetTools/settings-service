use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_ctx::AppContext;

use crate::models::*;

pub async fn save_secret(
    app: &AppContext,
    product_id: ProductId<'_>,
    secret_id: String,
    secret_value: String,
    remote_value: Option<String>,
    level: u8,
) -> Option<SecretItem> {
    let mut secret = SecretItem {
        id: secret_id,
        content: secret_value.into(),
        remote_value: remote_value
            .filter(|v| !v.is_empty())
            .map(|v| v.into()),
        level,
        created: DateTimeAsMicroseconds::now(),
        updated: DateTimeAsMicroseconds::now(),
    };
    let removed = if let Some(removed) = app.secrets.remove(product_id, &secret.id).await {
        secret.created = removed.created;
        Some(removed)
    } else {
        None
    };

    let snapshot = app
        .secrets
        .insert_or_update(product_id, [secret].into_iter())
        .await;

    app.secrets_persistence.save(&snapshot).await;

    removed
}
