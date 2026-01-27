use crate::caches::SecretsSnapshot;

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
            secrets.push(SecretValue {
                product_id: Default::default(),
                secret_id: itm.id.to_string(),
                value: itm.content.to_string(),
                level: itm.level as i32,
            });
        }

        for (product_id, items) in snapshot.by_product.iter() {
            for itm in items.iter() {
                secrets.push(SecretValue {
                    product_id: product_id.to_string(),
                    secret_id: itm.id.to_string(),
                    value: itm.content.to_string(),
                    level: itm.level as i32,
                });
            }
        }

        Self { secrets }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result).unwrap();
        result
    }
}
