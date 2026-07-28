use std::collections::BTreeMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

use crate::models::Product;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ProductFileItem {
    pub description: String,
    pub prompt: String,
    pub created: i64,
    pub updated: i64,
}

impl ProductFileItem {
    pub fn into_product(self, id: String) -> Product {
        Product {
            id,
            description: self.description,
            prompt: self.prompt,
            created: DateTimeAsMicroseconds::new(self.created),
            updated: DateTimeAsMicroseconds::new(self.updated),
        }
    }

    pub fn from_product(src: &Product) -> Self {
        Self {
            description: src.description.clone(),
            prompt: src.prompt.clone(),
            created: src.created.unix_microseconds,
            updated: src.updated.unix_microseconds,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ProductsFileModel {
    #[serde(default)]
    pub products: BTreeMap<String, ProductFileItem>,
}

impl ProductsFileModel {
    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec_pretty(self).unwrap()
    }

    pub fn from_slice(src: &[u8]) -> Self {
        serde_json::from_slice(src).unwrap()
    }
}
