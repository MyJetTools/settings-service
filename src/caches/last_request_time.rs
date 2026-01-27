use std::collections::BTreeMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::Mutex;

pub struct LastRequestTimeCache {
    pub items: Mutex<BTreeMap<String, BTreeMap<String, DateTimeAsMicroseconds>>>,
}

impl LastRequestTimeCache {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(BTreeMap::new()),
        }
    }

    pub async fn update(&self, product_id: &str, template_id: &str, now: DateTimeAsMicroseconds) {
        let mut write_access = self.items.lock().await;

        if !write_access.contains_key(product_id) {
            write_access.insert(product_id.to_string(), BTreeMap::new());
        }

        let sub_access = write_access.get_mut(product_id).unwrap();

        sub_access.insert(template_id.to_string(), now);
    }

    pub async fn get(&self, product_id: &str, template_id: &str) -> Option<DateTimeAsMicroseconds> {
        let read_access = self.items.lock().await;

        let by_product = read_access.get(product_id)?;

        let result = by_product.get(template_id)?;

        Some(*result)
    }
}
