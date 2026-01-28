use rust_extensions::sorted_vec::*;
use tokio::sync::RwLock;

use crate::models::*;

use super::*;

pub struct TemplatesCache {
    inner: RwLock<TemplatesCacheInner>,
}

impl TemplatesCache {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub async fn get_all<TResult>(
        &self,
        transform: impl Fn(&str, &TemplateItem) -> TResult,
    ) -> Vec<TResult> {
        let read_access = self.inner.read().await;
        let mut result = Vec::new();

        for (product_id, items) in read_access.items.iter() {
            for itm in items.iter() {
                result.push(transform(product_id.as_str(), itm));
            }
        }

        result
    }

    pub async fn get_products(&self) -> Vec<String> {
        let read_access = self.inner.read().await;
        read_access.items.keys().cloned().collect()
    }

    pub async fn get_by_product_id<TResult>(
        &self,
        product_id: &str,
        transform: impl Fn(&TemplateItem) -> TResult,
    ) -> Vec<TResult> {
        let mut result = Vec::new();
        let read_access = self.inner.read().await;

        if let Some(by_product) = read_access.items.get(product_id) {
            for itm in by_product.iter() {
                let item = transform(itm);
                result.push(item);
            }
        }

        result
    }

    pub async fn get_by_id<TResult>(
        &self,
        product_id: &str,
        template_id: &str,
        transform: impl Fn(&TemplateItem) -> TResult,
    ) -> Option<TResult> {
        let read_access = self.inner.read().await;

        let by_product = read_access.items.get(product_id)?;

        let item = by_product.get(template_id)?;

        let result = transform(item);

        Some(result)
    }

    pub async fn remove(&self, product_id: &str, template_id: &str) -> Option<TemplateItem> {
        let mut write_access = self.inner.write().await;
        let by_product = write_access.items.get_mut(product_id)?;
        by_product.remove(template_id)
    }

    pub async fn insert(&self, product_id: &str, items: impl Iterator<Item = TemplateItem>) {
        let mut write_access = self.inner.write().await;

        for item in items {
            match write_access.items.get_mut(product_id) {
                Some(items) => {
                    items.insert_or_replace(item);
                }
                None => {
                    let mut items = SortedVecWithStrKey::new();
                    items.insert_or_replace(item);
                    write_access.items.insert(product_id.to_string(), items);
                }
            }
        }
    }

    pub async fn find_into_vec_by_product<TResult>(
        &self,
        product_id: &str,
        callback: impl Fn(&TemplateItem) -> Option<TResult>,
    ) -> Vec<TResult> {
        let read_access = self.inner.read().await;

        let mut result = Vec::new();
        let Some(by_product) = read_access.items.get(product_id) else {
            return result;
        };

        for itm in by_product.iter() {
            if let Some(item) = callback(itm) {
                result.push(item);
            }
        }

        result
    }

    /*
       pub async fn get_count(
           &self,
           product_id: &str,
           predicate: impl Fn(&TemplateItem) -> bool,
       ) -> usize {
           let read_access = self.inner.read().await;
           let Some(by_product) = read_access.items.get(product_id) else {
               return 0;
           };

           let mut result = 0;
           for itm in by_product.iter() {
               if predicate(itm.as_ref()) {
                   result += 1;
               }
           }

           result
       }
    */
    pub async fn get_count_from_all(&self, predicate: impl Fn(&TemplateItem) -> bool) -> usize {
        let read_access = self.inner.read().await;

        let mut result = 0;
        for itm in read_access.items.values() {
            for itm in itm.iter() {
                if predicate(itm) {
                    result += 1;
                }
            }
        }

        result
    }
}
