use crate::app_ctx::{AppContext, SecretsValueReader};

pub async fn get_used_secret_amount_by_template(app: &AppContext, secret_name: &str) -> usize {
    if !app.templates_cache.is_initialized() {
        let templates = app.templates_storage.get_all().await.unwrap();
        app.templates_cache.init(templates).await;
    }

    let secret_name = format!("${{{}}}", secret_name);

    app.templates_cache
        .get_used_secret_amount(secret_name.as_str())
        .await
}

pub async fn get_used_secret_amount_by_secret(app: &AppContext, secret_name: &str) -> usize {
    let secrets = super::get_all_secrets(app).await;

    if secrets.is_none() {
        return 0;
    }

    let secret_name = format!("${{{}}}", secret_name);

    let mut amount = 0;

    for secret in secrets.unwrap() {
        let value = app.get_secret_value(secret.get_secret_name()).await;

        if let Some(value) = value {
            if value.content.contains(secret_name.as_str()) {
                amount += 1;
            }
        } else {
        }
    }

    amount
}

pub async fn has_secret(app: &AppContext, secret_name: &str) -> bool {
    app.secrets_repository
        .get_secret(secret_name)
        .await
        .is_some()
}
