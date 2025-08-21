use crate::app_ctx::AppContext;

use crate::models::*;

pub async fn get_secret_usage_by_templates(
    app: &AppContext,
    secret_name: &str,
) -> Vec<SecretUsage> {
    let templates = crate::scripts::templates::get_all(app).await;

    let mut result = Vec::new();
    for template in templates {
        if rust_common::placeholders::has_placeholder(
            &template.yaml_template,
            secret_name,
            super::super::PLACEHOLDER_OPEN,
            super::super::PLACEHOLDER_CLOSE,
        ) {
            result.push(SecretUsage {
                env: template.partition_key.clone(),
                name: template.row_key.clone(),
                yaml: template.yaml_template.clone(),
            });
        }
    }

    result
}
