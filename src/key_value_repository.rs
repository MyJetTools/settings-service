use std::collections::HashMap;

use my_azure_key_vault::MyAzureKeyVault;
use my_no_sql_data_writer::MyNoSqlDataWriter;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    caches::{SecretValue, SecretsCache},
    my_no_sql::SecretMyNoSqlEntity,
};
use encryption::aes::{AesEncryptedData, AesKey};

pub enum KeyValueRepositoryStorage {
    KeyValue(MyAzureKeyVault),
    EncodingKey(AesKey),
}

pub struct SecretsRepository {
    secrets_cache: SecretsCache,
    storage: KeyValueRepositoryStorage,
    pub secrets_storage: MyNoSqlDataWriter<SecretMyNoSqlEntity>,
}

impl SecretsRepository {
    pub fn new(
        storage: KeyValueRepositoryStorage,
        secrets_storage: MyNoSqlDataWriter<SecretMyNoSqlEntity>,
    ) -> Self {
        Self {
            storage,
            secrets_storage,
            secrets_cache: SecretsCache::new(),
        }
    }

    async fn initialize(&self) {
        if self.secrets_cache.is_initialized() {
            return;
        }
        let secrets = self
            .secrets_storage
            .get_by_partition_key(SecretMyNoSqlEntity::generate_partition_key(), None)
            .await
            .unwrap();

        self.secrets_cache.init(secrets).await;
    }

    pub async fn get_secret(&self, secret_name: &str) -> Option<SecretValue> {
        self.initialize().await;
        let entity = self.secrets_cache.get(secret_name).await?;

        match &self.storage {
            KeyValueRepositoryStorage::KeyValue(vault) => {
                let result = vault.get_secret(secret_name).await.unwrap();

                if result.is_none() {
                    panic!("Secret {} not found in azure vault", secret_name);
                }

                let result = result.unwrap();

                let result = SecretValue {
                    content: result,
                    level: entity.get_level(),
                };

                return Some(result);
            }
            KeyValueRepositoryStorage::EncodingKey(aes_key) => {
                if let Some(value) = &entity.value {
                    let bytes = AesEncryptedData::from_base_64(value);
                    if bytes.is_err() {
                        return Some(entity.to_empty_value());
                    }

                    let value = decode_value(&entity, aes_key);

                    match value {
                        Some(result) => {
                            return Some(result);
                        }
                        _ => return Some(entity.to_empty_value()),
                    }
                } else {
                    return Some(entity.to_empty_value());
                }
            }
        }
    }

    pub async fn set_secret(&self, secret_name: String, secret_value: &SecretValue) {
        self.initialize().await;
        let now = DateTimeAsMicroseconds::now().to_rfc3339();

        let mut entity = SecretMyNoSqlEntity {
            partition_key: SecretMyNoSqlEntity::generate_partition_key().to_string(),
            row_key: secret_name.to_string(),
            time_stamp: now.clone(),
            create_date: now.clone(),
            last_update_date: now,
            value: None,
            level: Some(secret_value.level),
            secret_usages: serde_json::to_string(&secret_value.get_usages())
                .unwrap()
                .into(),
        };

        match &self.storage {
            KeyValueRepositoryStorage::KeyValue(vault) => vault
                .set_secret(secret_name.as_str(), &secret_value.content)
                .await
                .unwrap(),
            KeyValueRepositoryStorage::EncodingKey(aes_key) => {
                let encrypted = aes_key.encrypt(secret_value.content.as_bytes());
                entity.value = Some(encrypted.as_base_64());
            }
        };

        self.secrets_storage
            .insert_or_replace_entity(&entity)
            .await
            .unwrap();
        self.secrets_cache.save(entity).await;
    }

    pub async fn delete_secret(&self, secret_name: &str) {
        self.secrets_storage
            .delete_row(SecretMyNoSqlEntity::generate_partition_key(), secret_name)
            .await
            .unwrap();

        match &self.storage {
            KeyValueRepositoryStorage::KeyValue(vault) => {
                vault.delete_secret(secret_name).await.unwrap();
            }
            KeyValueRepositoryStorage::EncodingKey(_) => {}
        }

        self.secrets_cache.delete(secret_name).await;
    }

    pub async fn get_all(&self) -> Option<Vec<SecretMyNoSqlEntity>> {
        self.initialize().await;
        self.secrets_cache.get_all().await
    }

    pub async fn get_as_hash_map(&self) -> Option<HashMap<String, SecretMyNoSqlEntity>> {
        self.initialize().await;

        let all = self.secrets_cache.get_all().await?;

        let mut result = HashMap::new();

        for entity in all {
            result.insert(entity.get_secret_name().to_string(), entity);
        }

        Some(result)
    }
}

fn decode_value(entity: &SecretMyNoSqlEntity, aes_key: &AesKey) -> Option<SecretValue> {
    let value = entity.value.as_ref();

    match value {
        Some(value) => {
            let encrypted_data = AesEncryptedData::from_base_64(value);

            if encrypted_data.is_err() {
                return None;
            }

            let encrypted_data = encrypted_data.unwrap();
            let result = aes_key.decrypt(&encrypted_data);
            match result {
                Ok(result) => SecretValue {
                    content: result.into_string(),
                    level: entity.get_level(),
                }
                .into(),
                Err(_) => None,
            }
        }
        None => None,
    }
}
