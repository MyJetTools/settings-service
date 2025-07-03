use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

use crate::models::*;

pub async fn update(app: &AppContext, secret_name: String, secret_value: SecretValue) {
    let now = DateTimeAsMicroseconds::now().to_rfc3339();

    let mut entity = SecretMyNoSqlEntity {
        partition_key: SecretMyNoSqlEntity::generate_partition_key().to_string(),
        row_key: secret_name.to_string(),
        time_stamp: Default::default(),
        create_date: now.clone(),
        last_update_date: now,
        value: None,
        level: Some(secret_value.level),
        secret_usages: serde_json::to_string(&secret_value.get_usages())
            .unwrap()
            .into(),
    };
    let encrypted = app.aes_key.encrypt(secret_value.content.as_bytes());
    entity.value = Some(encrypted.as_base_64());

    app.secrets_storage
        .insert_or_replace_entity(&entity)
        .await
        .unwrap();
}
