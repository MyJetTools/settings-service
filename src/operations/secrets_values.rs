use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

pub async fn get(app: &AppContext, secret_name: &str) -> Option<String> {
    if !app.secrets_cache.is_initialized() {
        let secrets = app.secrets_storage.get_all().await.unwrap();
        app.secrets_cache.init(secrets).await;
    }

    if !app.secrets_cache.has_value(secret_name).await {
        return None;
    }

    let result = app.secrets_values_cache.get(secret_name).await;

    if result.is_some() {
        return result;
    }

    let result = app.key_vault_client.get_secret(secret_name).await.unwrap();

    if let Some(result) = result.as_ref() {
        app.secrets_values_cache
            .set(secret_name.to_string(), result.clone())
            .await;
    }

    result
}

pub async fn save(app: &AppContext, secret_name: &str, value: &str) {
    if !app.secrets_cache.is_initialized() {
        let secrets = app.secrets_storage.get_all().await.unwrap();
        app.secrets_cache.init(secrets).await;
    }

    let entity = app
        .secrets_storage
        .get_entity(SecretMyNoSqlEntity::generate_partition_key(), secret_name)
        .await
        .unwrap();

    let entity = if let Some(mut entity) = entity {
        entity.last_update_date = DateTimeAsMicroseconds::now().to_rfc3339();
        entity
    } else {
        let now = DateTimeAsMicroseconds::now().to_rfc3339();
        SecretMyNoSqlEntity {
            partition_key: SecretMyNoSqlEntity::generate_partition_key().to_string(),
            row_key: secret_name.to_string(),
            time_stamp: now.clone(),
            create_date: now.clone(),
            last_update_date: now,
        }
    };

    app.key_vault_client
        .set_secret(secret_name, value)
        .await
        .unwrap();

    app.secrets_storage
        .insert_or_replace_entity(&entity)
        .await
        .unwrap();

    app.secrets_values_cache
        .set(secret_name.to_string(), value.to_string())
        .await;

    app.secrets_cache.insert(entity).await;
}
