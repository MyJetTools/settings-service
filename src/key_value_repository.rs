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

pub enum SecretValue {
    Unknown,
    None,
    Some(String),
}

pub struct KeyValueRepository {
    cache: Mutex<Option<BTreeMap<String, SecretValue>>>,
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

    pub async fn get_secret(&self, secret_name: &str) -> Option<String> {
        {
            let mut cache = self.cache.lock().await;

            if cache.is_none() {
                *cache = Some(self.init_all().await);
            }

            if let Some(cache) = cache.as_ref() {
                if let Some(value) = cache.get(secret_name) {
                    if let SecretValue::Some(value) = value {
                        return Some(value.to_string());
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

        let result = match &self.storage {
            KeyValueRepositoryStorage::KeyValue(vault) => {
                vault.get_secret(secret_name).await.unwrap()
            }
            KeyValueRepositoryStorage::EncodingKey(aes_key) => {
                if let Some(value) = &entity.value {
                    let bytes = AesEncryptedData::from_base_64(value);
                    if bytes.is_err() {
                        return Some("".to_string());
                    }

                    let bytes = bytes.unwrap();
                    let result = aes_key.decrypt(&bytes);
                    match result {
                        Ok(result) => return Some(result.into_string()),
                        Err(_) => return Some("".to_string()),
                    }
                } else {
                    return Some("".to_string());
                }
            }
        };

        if let Some(secret_value) = &result {
            self.update_cache(secret_name, secret_value).await;
        }

        result
    }

    async fn update_cache(&self, secret_name: &str, secret_value: &str) {
        let mut cache = self.cache.lock().await;

        if cache.is_none() {
            *cache = Some(self.init_all().await);
        }

        if let Some(cache) = cache.as_mut() {
            cache.insert(
                secret_name.to_string(),
                SecretValue::Some(secret_value.to_string()),
            );
        }
    }
    pub async fn set_secret(&self, secret_name: &str, secret_value: &str, level: u8) {
        let now = DateTimeAsMicroseconds::now().to_rfc3339();

        let mut entity = SecretMyNoSqlEntity {
            partition_key: SecretMyNoSqlEntity::generate_partition_key().to_string(),
            row_key: secret_name.to_string(),
            time_stamp: now.clone(),
            create_date: now.clone(),
            last_update_date: now,
            value: None,
            level: Some(level),
        };

        match &self.storage {
            KeyValueRepositoryStorage::KeyValue(vault) => {
                vault.set_secret(secret_name, secret_value).await.unwrap()
            }
            KeyValueRepositoryStorage::EncodingKey(aes_key) => {
                let encrypted = aes_key.encrypt(secret_value.as_bytes());
                entity.value = Some(encrypted.as_base_64());
            }
        };

        self.secrets_storage
            .insert_or_replace_entity(&entity)
            .await
            .unwrap();

        self.update_cache(secret_name, secret_value).await;
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

    async fn init_all(&self) -> BTreeMap<String, SecretValue> {
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
                        result.insert(entity.row_key.clone(), SecretValue::Unknown);
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

fn decode_value(entity: &SecretMyNoSqlEntity, aes_key: &AesKey) -> SecretValue {
    let value = entity.value.as_ref();

    match value {
        Some(value) => {
            let encrypted_data = AesEncryptedData::from_base_64(value);

            if encrypted_data.is_err() {
                return SecretValue::None;
            }

            let encrypted_data = encrypted_data.unwrap();
            let result = aes_key.decrypt(&encrypted_data);
            match result {
                Ok(result) => SecretValue::Some(result.into_string()),
                Err(_) => SecretValue::None,
            }
        }
        None => SecretValue::Unknown,
    }
}
