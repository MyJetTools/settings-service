use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{caches::SecretsSnapshot, models::SecretItem};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretValue {
    #[prost(string, tag = "1")]
    pub product_id: String,
    #[prost(string, tag = "2")]
    pub secret_id: String,
    #[prost(string, tag = "3")]
    pub value: String,
    #[prost(int32, tag = "4")]
    pub level: i32,
    #[prost(int64, tag = "5")]
    pub created: i64,
    #[prost(int64, tag = "6")]
    pub updated: i64,
}
impl SecretValue {
    pub fn from_item(product_id: String, src: &SecretItem) -> Self {
        Self {
            product_id,
            secret_id: src.id.to_string(),
            value: src.content.to_string(),
            level: src.level as i32,
            created: src.created.unix_microseconds,
            updated: src.updated.unix_microseconds,
        }
    }

    pub fn into_secret_item(self) -> (String, SecretItem) {
        let result = SecretItem {
            id: self.secret_id,
            content: self.value.into(),
            level: self.level as u8,
            created: DateTimeAsMicroseconds::new(self.created),
            updated: DateTimeAsMicroseconds::new(self.updated),
        };

        (self.product_id, result)
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretFileModel {
    #[prost(message, repeated, tag = "1")]
    pub secrets: Vec<SecretValue>,
}

impl SecretFileModel {
    pub fn from_snapshot(snapshot: &SecretsSnapshot) -> Self {
        let mut secrets = Vec::new();
        for itm in snapshot.shared.iter() {
            secrets.push(SecretValue::from_item(Default::default(), itm));
        }

        for (product_id, items) in snapshot.by_product.iter() {
            for itm in items.iter() {
                secrets.push(SecretValue::from_item(product_id.to_string(), itm));
            }
        }

        Self { secrets }
    }

    pub fn from_slice(src: &[u8]) -> Option<Self> {
        match prost::Message::decode(src) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result).unwrap();
        result
    }
}
