use std::collections::BTreeMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct LastRequestTimeCache {
    pub items: BTreeMap<String, BTreeMap<String, DateTimeAsMicroseconds>>,
}

impl LastRequestTimeCache {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn update(&mut self, product_id: &str, template_id: &str, now: DateTimeAsMicroseconds) {
        if !self.items.contains_key(product_id) {
            self.items.insert(product_id.to_string(), BTreeMap::new());
        }

        let sub_access = self.items.get_mut(product_id).unwrap();

        sub_access.insert(template_id.to_string(), now);
    }

    pub fn get(&self, product_id: &str, template_id: &str) -> Option<DateTimeAsMicroseconds> {
        let by_product = self.items.get(product_id)?;

        let result = by_product.get(template_id)?;

        Some(*result)
    }
}
