use crate::app_ctx::AppContext;

pub async fn show_populated_secret(app: &AppContext, secret_name: &str) -> Option<String> {
    let result = app.key_value_repository.get_secret(secret_name).await?;

    if result.value.contains("${") {
        return Some(super::populate_with_secrets(app, &result.value, result.level + 1).await);
    } else {
        return Some(result.value);
    }
}
