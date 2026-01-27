use encryption::*;
use rust_extensions::{file_utils::FilePath, sorted_vec::*};

use crate::caches::SecretsSnapshot;

pub struct SecretsPersistence {
    aes_key: encryption::aes::AesKey,
    path: FilePath,
}

impl SecretsPersistence {
    pub fn new(mut path: FilePath, aes_key: encryption::aes::AesKey) -> Self {
        path.append_segment("secrets.dat");
        Self { path, aes_key }
    }

    pub async fn save(&self, snapshot: &SecretsSnapshot) {
        let file_content = super::models::SecretFileModel::from_snapshot(snapshot).to_vec();
        let encrypted = self.aes_key.encrypt(&file_content);

        tokio::fs::write(self.path.as_str(), encrypted.as_slice())
            .await
            .unwrap();
    }

    pub async fn get_all(&self) -> SecretsSnapshot {
        let content = match tokio::fs::read(self.path.as_str()).await {
            Ok(content) => content,
            Err(err) => {
                eprintln!(
                    "Can not open secrets file `{}`. Err:{:?}. Creating new secrets snapshot",
                    self.path.as_str(),
                    err
                );
                return SecretsSnapshot::default();
            }
        };

        let encrypted_data = AesEncryptedDataOwned::new(content);

        let decrypted_data = match self.aes_key.decrypt(&encrypted_data) {
            Ok(decrypted_data) => decrypted_data,
            Err(_) => {
                panic!(
                    "Can not decrypt secrets binary file '{}'. File is wrong or corrupted",
                    self.path.as_str()
                )
            }
        };

        let Some(snapshot) = super::models::SecretFileModel::from_slice(decrypted_data.as_slice())
        else {
            panic!("Secrets binary file '{}' is corrupted. It decrypted but can not be deserialized. File is wrong or corrupt", self.path.as_str())
        };

        let mut result = SecretsSnapshot::default();

        for file_secret in snapshot.secrets {
            let (product_id, secret_item) = file_secret.into_secret_item();

            if product_id.len() == 0 {
                result.shared.insert_or_replace(secret_item.into());
                continue;
            }

            match result.by_product.get_mut(product_id.as_str()) {
                Some(items) => {
                    items.insert_or_replace(secret_item);
                }
                None => {
                    let mut items = SortedVecWithStrKey::new();
                    items.insert_or_replace(secret_item);
                    result.by_product.insert(product_id.to_string(), items);
                }
            }
        }

        result
    }
}
