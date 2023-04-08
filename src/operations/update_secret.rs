use crate::{app_ctx::AppContext, caches::SecretValue};

pub async fn update_secret(app: &AppContext, secret_name: String, secret_value: SecretValue) {
    app.secrets_repository
        .set_secret(secret_name.to_string(), &secret_value)
        .await;

    app.secret_values_cache
        .save(&secret_name, secret_value)
        .await;
}
