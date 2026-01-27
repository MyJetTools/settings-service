use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::app_ctx::AppContext;
use crate::flows::SnapshotExportModel;
use crate::models::{SecretItem, TemplateItem};

pub async fn import_snapshot(
    app: &AppContext,
    product_id: &str,
    snapshot: &[u8],
    templates_only: bool,
) {
    let mut model: SnapshotExportModel = serde_json::from_slice(snapshot).unwrap();

    for template in model.templates.iter_mut() {
        template.content = get_content_from_base64(&template.content);
    }

    for secret in model.secrets.iter_mut() {
        secret.value = get_content_from_base64(&secret.value);
    }

    let now = DateTimeAsMicroseconds::now();

    let templates: Vec<_> = model
        .templates
        .into_iter()
        .map(|itm| TemplateItem {
            id: itm.id,
            content: itm.content.into(),
            created: now,
            last_update: now,
        })
        .collect();

    app.templates_persistence
        .save(product_id, templates.as_slice())
        .await;

    app.templates
        .insert(product_id, templates.into_iter())
        .await;

    if templates_only {
        return;
    }
    let mut shared_secrets = Vec::new();
    let mut not_shared = Vec::new();

    for secret in model.secrets {
        let shared = secret.shared.is_some();

        let item = SecretItem {
            id: secret.id,
            content: secret.value.into(),
            level: secret.level,
            created: now,
            updated: now,
        };

        if shared {
            shared_secrets.push(item);
        } else {
            not_shared.push(item);
        }
    }

    app.secrets
        .insert_or_update(crate::models::ProductId::Shared, shared_secrets.into_iter())
        .await;

    let snapshot = app
        .secrets
        .insert_or_update(
            crate::models::ProductId::Id(product_id),
            not_shared.into_iter(),
        )
        .await;

    app.secrets_persistence.save(snapshot.as_ref()).await;
}

fn get_content_from_base64(src: &str) -> String {
    use rust_extensions::base64::FromBase64;
    let content = src.from_base64().unwrap();
    String::from_utf8(content).unwrap()
}
