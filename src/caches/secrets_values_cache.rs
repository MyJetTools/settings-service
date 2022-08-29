use std::collections::HashMap;

use tokio::sync::Mutex;

pub struct SecretsValuesCache {
    cache: Mutex<HashMap<String, String>>,
}

impl SecretsValuesCache {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get(&self, name: &str) -> Option<String> {
        self.cache.lock().await.get(name).cloned()
    }

    pub async fn set(&self, name: String, secret: String) {
        self.cache.lock().await.insert(name, secret);
    }
}
