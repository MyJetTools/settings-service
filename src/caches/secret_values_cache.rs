use std::collections::HashMap;

use tokio::sync::RwLock;

use super::SecretValue;

pub struct SecretsValuesCache {
    cache: RwLock<HashMap<String, SecretValue>>,
}

impl SecretsValuesCache {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(&self, secret_name: &str) -> Option<SecretValue> {
        let read_access = self.cache.read().await;

        let result = read_access.get(secret_name)?;
        Some(result.clone())
    }

    pub async fn save(&self, secret_name: &str, value: SecretValue) {
        let mut write_access = self.cache.write().await;

        write_access.insert(secret_name.to_string(), value);
    }

    pub async fn delete(&self, secret_name: &str) {
        let mut write_access = self.cache.write().await;
        write_access.remove(secret_name);
    }
}
