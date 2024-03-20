use crate::{app_ctx::AppContext, caches::SecretUsage};

pub async fn get_secret_usage_in_templates(
    app: &AppContext,
    secret_name: &str,
) -> Vec<SecretUsage> {
    super::initialize_templates(app, false).await;

    let templates = app.templates_cache.get_all().await;

    let mut result = Vec::new();
    for template in templates {
        if rust_extensions::placeholders::has_placeholder(&template.yaml_template, secret_name) {
            result.push(SecretUsage {
                env: template.partition_key.clone(),
                name: template.row_key.clone(),
                yaml: template.yaml_template.clone(),
            });
        }
    }

    result
}
