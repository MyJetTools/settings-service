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
    let secrets = crate::scripts::secrets::get_all(app).await;

    if secrets.is_none() {
        return 0;
    }

    let secret_name = format!("${{{}}}", secret_name);

    let mut amount = 0;

    for secret in secrets.unwrap() {
        let value = crate::scripts::secrets::decode(&secret, &app.aes_key);

        if let Some(value) = value {
            if value.content.contains(secret_name.as_str()) {
                amount += 1;
            }
        } else {
        }
    }

    amount
}
