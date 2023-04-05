use std::collections::BTreeMap;

use my_azure_key_vault::MyAzureKeyVault;
use my_no_sql_data_writer::MyNoSqlDataWriter;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::Mutex;

use crate::my_no_sql::SecretMyNoSqlEntity;
use encryption::aes::{AesEncryptedData, AesKey};

pub enum KeyValueRepositoryStorage {
    KeyValue(MyAzureKeyVault),
    EncodingKey(AesKey),
}

#[derive(Debug, Clone)]
pub struct SecretValue {
    pub value: String,
    pub level: u8,
}

pub enum SecretCacheValue {
    Unknown,
    None,
    Some(SecretValue),
}

pub struct KeyValueRepository {
    cache: Mutex<Option<BTreeMap<String, SecretCacheValue>>>,
    storage: KeyValueRepositoryStorage,
    pub secrets_storage: MyNoSqlDataWriter<SecretMyNoSqlEntity>,
}

impl KeyValueRepository {
    pub fn new(
        storage: KeyValueRepositoryStorage,
        secrets_storage: MyNoSqlDataWriter<SecretMyNoSqlEntity>,
    ) -> Self {
        Self {
            storage,
            secrets_storage,
            cache: Mutex::new(None),
        }
    }

    pub async fn get_secret(&self, secret_name: &str) -> Option<SecretValue> {
        {
            let mut cache = self.cache.lock().await;

            if cache.is_none() {
                *cache = Some(self.init_all().await);
            }

            if let Some(cache) = cache.as_ref() {
                if let Some(value) = cache.get(secret_name) {
                    if let SecretCacheValue::Some(value) = value {
                        return Some(value.clone());
                    }
                }
            }
        }

        let entity = self
            .secrets_storage
            .get_entity(
                SecretMyNoSqlEntity::generate_partition_key(),
                secret_name,
                None,
            )
            .await
            .unwrap()?;

        match &self.storage {
            KeyValueRepositoryStorage::KeyValue(vault) => {
                let result = vault.get_secret(secret_name).await.unwrap();

                if result.is_none() {
                    panic!("Secret {} not found in azure vault", secret_name);
                }

                let result = result.unwrap();

                let result = SecretValue {
                    value: result,
                    level: entity.get_level(),
                };

                self.update_cache(secret_name, result.clone()).await;

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
                        SecretCacheValue::Some(result) => {
                            self.update_cache(secret_name, result.clone()).await;
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

    async fn update_cache(&self, secret_name: &str, value: SecretValue) {
        let mut cache = self.cache.lock().await;

        if cache.is_none() {
            *cache = Some(self.init_all().await);
        }

        if let Some(cache) = cache.as_mut() {
            cache.insert(secret_name.to_string(), SecretCacheValue::Some(value));
        }
    }
    pub async fn set_secret(&self, secret_name: String, secret_value: SecretValue) {
        let now = DateTimeAsMicroseconds::now().to_rfc3339();

        let mut entity = SecretMyNoSqlEntity {
            partition_key: SecretMyNoSqlEntity::generate_partition_key().to_string(),
            row_key: secret_name.to_string(),
            time_stamp: now.clone(),
            create_date: now.clone(),
            last_update_date: now,
            value: None,
            level: Some(secret_value.level),
        };

        match &self.storage {
            KeyValueRepositoryStorage::KeyValue(vault) => vault
                .set_secret(secret_name.as_str(), &secret_value.value)
                .await
                .unwrap(),
            KeyValueRepositoryStorage::EncodingKey(aes_key) => {
                let encrypted = aes_key.encrypt(secret_value.value.as_bytes());
                entity.value = Some(encrypted.as_base_64());
            }
        };

        self.secrets_storage
            .insert_or_replace_entity(&entity)
            .await
            .unwrap();

        self.update_cache(secret_name.as_str(), secret_value).await;
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

        let mut cache = self.cache.lock().await;
        if cache.is_none() {
            *cache = Some(self.init_all().await);
        }

        if let Some(cache) = cache.as_mut() {
            cache.remove(secret_name);
        }
    }

    pub async fn get_all(&self) -> Vec<SecretMyNoSqlEntity> {
        let secrets = self
            .secrets_storage
            .get_by_partition_key(SecretMyNoSqlEntity::generate_partition_key(), None)
            .await
            .unwrap();

        match secrets {
            Some(result) => result,
            None => Vec::new(),
        }
    }

    async fn init_all(&self) -> BTreeMap<String, SecretCacheValue> {
        let secrets = self
            .secrets_storage
            .get_by_partition_key(SecretMyNoSqlEntity::generate_partition_key(), None)
            .await
            .unwrap();

        let mut result = BTreeMap::new();

        if let Some(secrets) = secrets {
            for entity in secrets {
                match &self.storage {
                    KeyValueRepositoryStorage::KeyValue(_) => {
                        result.insert(entity.row_key.clone(), SecretCacheValue::Unknown);
                    }
                    KeyValueRepositoryStorage::EncodingKey(aes_key) => {
                        result.insert(entity.row_key.clone(), decode_value(&entity, aes_key));
                    }
                }
            }
        }

        return result;
    }
}

fn decode_value(entity: &SecretMyNoSqlEntity, aes_key: &AesKey) -> SecretCacheValue {
    let value = entity.value.as_ref();

    match value {
        Some(value) => {
            let encrypted_data = AesEncryptedData::from_base_64(value);

            if encrypted_data.is_err() {
                return SecretCacheValue::None;
            }

            let encrypted_data = encrypted_data.unwrap();
            let result = aes_key.decrypt(&encrypted_data);
            match result {
                Ok(result) => SecretCacheValue::Some(SecretValue {
                    value: result.into_string(),
                    level: entity.get_level(),
                }),
                Err(_) => SecretCacheValue::None,
            }
        }
        None => SecretCacheValue::Unknown,
    }
}
