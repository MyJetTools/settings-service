use std::collections::HashMap;

use rust_extensions::sorted_vec::*;

use crate::models::{ProductId, SecretItem};

use super::*;

#[derive(Default, Clone)]
pub struct SecretsSnapshot {
    pub shared: SortedVecWithStrKey<SecretItem>,
    pub by_product: HashMap<String, SortedVecWithStrKey<SecretItem>>,

    shared_usage: Usage,
    usage_by_product: HashMap<String, Usage>,
}

impl SecretsSnapshot {
    pub fn get_all_by_product_id<TResult>(
        &self,
        product_id: ProductId<'_>,
        transform: impl Fn(&SecretItem) -> TResult,
    ) -> Vec<TResult> {
        let product_id = match product_id {
            ProductId::Shared => {
                let mut result = Vec::with_capacity(self.shared.capacity());

                for itm in self.shared.iter() {
                    result.push(transform(itm));
                }

                return result;
            }
            ProductId::Id(product_id) => product_id,
        };

        let Some(by_product) = self.by_product.get(product_id) else {
            return vec![];
        };

        let mut result = Vec::with_capacity(by_product.len());

        for itm in by_product.iter() {
            result.push(transform(itm));
        }

        result
    }

    pub fn get_by_id(&self, product_id: ProductId<'_>, secret_id: &str) -> Option<&SecretItem> {
        let product_id = match product_id {
            ProductId::Shared => {
                return self.shared.get(secret_id);
            }
            ProductId::Id(product_id) => product_id,
        };

        let by_product = self.by_product.get(product_id)?;

        by_product.get(secret_id)
    }
    pub fn get_secreted_amount(&self) -> usize {
        let mut result = self.shared.len();

        for by_product in self.by_product.values() {
            result += by_product.len();
        }

        result
    }

    pub fn consume_secret(
        &self,
        product_id: ProductId<'_>,
        secret_id: &str,
    ) -> Option<&SecretItem> {
        let product_id = match product_id {
            ProductId::Shared => {
                return self.shared.get(secret_id);
            }
            ProductId::Id(product_id) => product_id,
        };

        if let Some(by_product) = self.by_product.get(product_id) {
            if let Some(result) = by_product.get(secret_id) {
                return Some(result);
            }
        }

        self.shared.get(secret_id)
    }

    pub fn get_slice<'s>(&'s self, product_id: ProductId<'_>) -> &'s [SecretItem] {
        match product_id {
            ProductId::Shared => self.shared.as_slice(),
            ProductId::Id(product_id) => match self.by_product.get(product_id) {
                Some(items) => items.as_slice(),
                None => &self.shared.as_slice()[0..0],
            },
        }
    }

    pub fn get_count(
        &self,
        product_id: ProductId<'_>,
        predicate: impl Fn(&SecretItem) -> bool,
    ) -> usize {
        let mut result = 0;

        match product_id {
            ProductId::Shared => {
                for itm in self.shared.as_slice() {
                    if predicate(itm) {
                        result += 1;
                    }
                }
            }
            ProductId::Id(product_id) => {
                if let Some(items_by_product) = self.by_product.get(product_id) {
                    for itm in items_by_product.iter() {
                        if predicate(itm) {
                            result += 1;
                        }
                    }
                }
            }
        }

        result
    }

    pub fn has_secret(&self, product_id: ProductId<'_>, secret_id: &str) -> bool {
        let product_id = match product_id {
            ProductId::Shared => {
                return self.shared.contains(secret_id);
            }
            ProductId::Id(product_id) => product_id,
        };

        let Some(by_product) = self.by_product.get(product_id) else {
            return false;
        };

        by_product.contains(secret_id)
    }

    pub fn has_secret_to_consume(&self, product_id: ProductId<'_>, secret_id: &str) -> bool {
        let product_id = match product_id {
            ProductId::Shared => {
                return self.shared.contains(secret_id);
            }
            ProductId::Id(product_id) => product_id,
        };

        if let Some(by_product) = self.by_product.get(product_id) {
            if by_product.contains(secret_id) {
                return true;
            }
        }

        self.shared.contains(secret_id)
    }

    pub fn find_into_vec_by_product<TResult>(
        &self,
        product_id: ProductId<'_>,
        callback: impl Fn(&SecretItem) -> Option<TResult>,
    ) -> Vec<TResult> {
        let mut result = Vec::new();
        match product_id {
            ProductId::Shared => {
                for itm in self.shared.iter() {
                    if let Some(item) = callback(itm) {
                        result.push(item);
                    }
                }
            }
            ProductId::Id(product_id) => {
                if let Some(by_product) = self.by_product.get(product_id) {
                    for itm in by_product.iter() {
                        if let Some(item) = callback(itm) {
                            result.push(item);
                        }
                    }
                }
            }
        }

        result
    }

    pub fn find_all_into_vec<TResult>(
        &self,
        callback: impl Fn(&SecretItem) -> Option<TResult>,
    ) -> Vec<TResult> {
        let mut result = Vec::new();

        for itm in self.shared.iter() {
            if let Some(item) = callback(itm) {
                result.push(item);
            }
        }

        for by_product in self.by_product.values() {
            for itm in by_product.iter() {
                if let Some(item) = callback(itm) {
                    result.push(item);
                }
            }
        }

        result
    }

    pub fn calc_usage(&mut self) {
        self.shared_usage.reset();
        self.usage_by_product.clear();

        for itm in self.shared.iter() {
            for secret_id in itm.content.get_secrets() {
                println!("Calculating shared secret: {}", secret_id);
                if self.shared.contains(secret_id) {
                    println!("Has Shared secret");
                    self.shared_usage.inc(secret_id);
                }

                for by_product in self.by_product.values() {
                    if by_product.contains(secret_id) {
                        self.shared_usage.inc(secret_id);
                        continue;
                    }
                }
            }
        }

        for (product_id, by_product) in self.by_product.iter() {
            for itm in by_product.iter() {
                for secret_id in itm.content.get_secrets() {
                    println!(
                        "Calculating shared secret: {} for product: {}",
                        secret_id, product_id
                    );
                    if by_product.contains(secret_id) {
                        if !self.usage_by_product.contains_key(product_id) {
                            self.usage_by_product
                                .insert(product_id.to_string(), Default::default());
                        }

                        let by_product = self.usage_by_product.get_mut(product_id).unwrap();

                        by_product.inc(secret_id);
                    }
                }
            }
        }

        println!("Shared: {:?}", self.shared_usage);
        println!("ByProduct: {:?}", self.by_product);
    }

    pub fn get_usage(&self, product_id: ProductId<'_>, secret_id: &str) -> usize {
        match product_id {
            ProductId::Shared => return self.shared_usage.get(secret_id),
            ProductId::Id(product_id) => {
                if let Some(by_product) = self.usage_by_product.get(product_id) {
                    return by_product.get(secret_id);
                }
            }
        }

        0
    }
}
