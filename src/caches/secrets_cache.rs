use std::{collections::BTreeMap, sync::atomic::AtomicBool};

use tokio::sync::RwLock;

use crate::my_no_sql::SecretMyNoSqlEntity;

#[derive(Debug, Clone)]
pub struct SecretValue {
    pub content: String,
    pub level: u8,
}

impl SecretValue {
    pub fn get_usages(&self) -> Vec<&str> {
        let mut result = Vec::new();
        for token in crate::operations::get_tokens_with_placeholders(&self.content) {
            match token {
                crate::operations::ContentToken::Text(_) => {}
                crate::operations::ContentToken::Placeholder(secret_name) => {
                    result.push(secret_name)
                }
            }
        }

        result
    }
}

pub struct SecretsCache {
    cache: RwLock<Option<BTreeMap<String, SecretMyNoSqlEntity>>>,
    initialized: AtomicBool,
}

impl SecretsCache {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(None),
            initialized: AtomicBool::new(false),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub async fn get(&self, secret_name: &str) -> Option<SecretMyNoSqlEntity> {
        let read_access = self.cache.read().await;

        if read_access.is_none() {
            return None;
        }

        let cache = read_access.as_ref().unwrap();
        let result = cache.get(secret_name)?;
        Some(result.clone())
    }

    pub async fn get_all(&self) -> Option<Vec<SecretMyNoSqlEntity>> {
        let read_access = self.cache.read().await;

        if read_access.is_none() {
            return None;
        }

        let cache = read_access.as_ref().unwrap();

        Some(cache.values().map(|itm| itm.clone()).collect())
    }

    pub async fn save(&self, value: SecretMyNoSqlEntity) {
        let mut write_access = self.cache.write().await;

        if write_access.is_none() {
            *write_access = Some(BTreeMap::new());
        }

        write_access
            .as_mut()
            .unwrap()
            .insert(value.get_secret_name().to_string(), value);
    }

    pub async fn delete(&self, secret_name: &str) {
        let mut write_access = self.cache.write().await;
        write_access.as_mut().unwrap().remove(secret_name);
    }

    pub async fn init(&self, secrets: Option<Vec<SecretMyNoSqlEntity>>) {
        if secrets.is_none() {
            return;
        }

        let secrets = secrets.unwrap();
        let mut write_access = self.cache.write().await;
        let mut cache = BTreeMap::new();
        for secret in secrets {
            cache.insert(secret.get_secret_name().to_string(), secret);
        }

        *write_access = Some(cache);

        self.initialized
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
