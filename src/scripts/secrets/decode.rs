use encryption::{aes::AesKey, AesEncryptedDataOwned};

use crate::my_no_sql::SecretMyNoSqlEntity;

use crate::models::*;

pub fn decode(entity: &SecretMyNoSqlEntity, aes_key: &AesKey) -> Option<SecretValue> {
    let value = entity.value.as_ref();

    match value {
        Some(value) => {
            let encrypted_data = AesEncryptedDataOwned::from_base_64(value);

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
