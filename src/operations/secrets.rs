use crate::app_ctx::AppContext;

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
    let secrets = app.key_value_repository.get_all().await;

    let secret_name = format!("${{{}}}", secret_name);

    let mut amount = 0;

    for secret in secrets {
        let value = app
            .key_value_repository
            .get_secret(&secret.get_secret_name())
            .await;

        if let Some(value) = value {
            if value.value.contains(secret_name.as_str()) {
                amount += 1;
            }
        } else {
        }
    }

    amount
}
