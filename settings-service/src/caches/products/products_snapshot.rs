use std::collections::BTreeMap;

use crate::models::Product;

#[derive(Default, Clone)]
pub struct ProductsSnapshot {
    pub by_id: BTreeMap<String, Product>,
}

impl ProductsSnapshot {
    pub fn get(&self, id: &str) -> Option<&Product> {
        self.by_id.get(id)
    }

    pub fn as_vec(&self) -> Vec<Product> {
        self.by_id.values().cloned().collect()
    }
}
