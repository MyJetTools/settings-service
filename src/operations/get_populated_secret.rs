use crate::{app_ctx::AppContext, key_value_repository::SecretValue};

pub async fn get_populated_secret(app: &AppContext, secret_name: &str) -> Option<SecretValue> {
    let secret_value = app.key_value_repository.get_secret(secret_name).await;

    if let Some(secret_value) = secret_value {
        if secret_value.value.contains("${") {
            let value =
                super::populate_with_secrets(app, &secret_value.value, secret_value.level + 1)
                    .await;
            return Some(SecretValue {
                level: secret_value.level + 1,
                value,
            });
        } else {
            return Some(secret_value);
        }
    } else {
        Some(SecretValue {
            value: format!("Secret {} not found", secret_name),
            level: 0,
        })
    }
}
