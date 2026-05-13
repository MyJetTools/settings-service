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
    // None = keep existing description on update; Some("") = clear it; Some(text) = set/replace.
    description: Option<String>,
    // None = keep existing visibility on update (false on create); Some(b) = set explicitly.
    visible_for_mcp: Option<bool>,
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
        description: description.clone().and_then(|d| {
            let trimmed = d.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }),
        visible_for_mcp: visible_for_mcp.unwrap_or(false),
    };
    let removed = if let Some(removed) = app.secrets.remove(product_id, &secret.id).await {
        secret.created = removed.created;
        if description.is_none() {
            secret.description = removed.description.clone();
        }
        if visible_for_mcp.is_none() {
            secret.visible_for_mcp = removed.visible_for_mcp;
        }
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
