use rust_extensions::file_utils::FilePath;

use crate::caches::SecretsSnapshot;

pub struct SecretsPersistence {
    aes_key: encryption::aes::AesKey,
    path: FilePath,
}

impl SecretsPersistence {
    pub fn new(path: FilePath, aes_key: encryption::aes::AesKey) -> Self {
        Self { path, aes_key }
    }

    pub async fn save(&self, snapshot: &SecretsSnapshot) {
        let file_content = super::models::SecretFileModel::from_snapshot(snapshot).to_vec();
        let encrypted = self.aes_key.encrypt(&file_content);

        tokio::fs::write(self.path.as_str(), encrypted.as_slice())
            .await
            .unwrap();
    }
}
