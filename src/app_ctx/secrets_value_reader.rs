use crate::caches::SecretValue;

use super::AppContext;

#[async_trait::async_trait]
pub trait SecretsValueReader {
    async fn get_secret_value(&self, secret_name: &str) -> Option<SecretValue>;
}

#[async_trait::async_trait]
impl SecretsValueReader for AppContext {
    async fn get_secret_value(&self, secret_name: &str) -> Option<SecretValue> {
        if let Some(value) = self.secret_values_cache.get(secret_name).await {
            return Some(value);
        }

        let value = self.secrets_repository.get_secret(secret_name).await?;

        self.secret_values_cache
            .save(secret_name, value.clone())
            .await;

        Some(value)
    }
}
