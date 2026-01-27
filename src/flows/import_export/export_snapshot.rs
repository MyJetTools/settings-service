use crate::app_ctx::AppContext;

use super::models::*;

pub async fn export_snapshot(
    app: &AppContext,
    product_id: &str,
    templates_only: bool,
) -> SnapshotExportModel {
    let max_level = app.settings.max_level_of_secrets_to_export;
    let templates = app
        .templates
        .get_by_product_id(product_id, |itm| TemplateExportModel::from_cache_item(itm))
        .await;

    let secrets = app.secrets.get_snapshot().await;

    let mut result = SnapshotExportModel {
        templates,
        secrets: Vec::new(),
    };

    if !templates_only {
        for secret in secrets.shared.iter() {
            result.secrets.push(SecretExportModel {
                shared: Some(1),
                id: secret.id.to_string(),
                value: secret.content.to_base_64(),
                level: secret.level,
            });
        }

        if let Some(by_product) = secrets.by_product.get(product_id) {
            for secret in by_product.iter() {
                if secret.level <= max_level {
                    result.secrets.push(SecretExportModel {
                        shared: None,
                        id: secret.id.to_string(),
                        value: secret.content.to_base_64(),
                        level: secret.level,
                    });
                }
            }
        }
    }

    result
}
