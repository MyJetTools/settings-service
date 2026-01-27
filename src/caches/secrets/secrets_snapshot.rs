use std::collections::HashMap;

use rust_extensions::sorted_vec::*;

use crate::models::{ProductId, SecretItem};

#[derive(Default, Clone)]
pub struct SecretsSnapshot {
    pub shared: SortedVecWithStrKey<SecretItem>,
    pub by_product: HashMap<String, SortedVecWithStrKey<SecretItem>>,
}

impl SecretsSnapshot {
    pub async fn get_all_by_product_id<TResult>(
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

    pub async fn find_into_vec_by_product<TResult>(
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
}
