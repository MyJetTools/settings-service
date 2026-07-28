use std::sync::Arc;

use rust_extensions::sorted_vec::*;
use tokio::sync::RwLock;

use crate::{caches::SecretsSnapshot, models::*};

#[derive(Default)]
pub struct SecretsCache {
    inner: RwLock<(SecretsSnapshot, Arc<SecretsSnapshot>)>,
}

impl SecretsCache {
    pub async fn init(&self, mut items: SecretsSnapshot) {
        items.calc_usage();
        let mut write_access = self.inner.write().await;

        let snapshot = Arc::new(items.clone());

        write_access.0 = items;
        write_access.1 = snapshot;
    }

    pub async fn get_snapshot(&self) -> Arc<SecretsSnapshot> {
        let read_access = self.inner.read().await;
        read_access.1.clone()
    }

    pub async fn insert_or_update(
        &self,
        product_id: ProductId<'_>,
        items: impl Iterator<Item = SecretItem>,
    ) -> Arc<SecretsSnapshot> {
        let mut write_access = self.inner.write().await;

        for item in items {
            match product_id {
                ProductId::Shared => {
                    write_access.0.shared.insert_or_replace(item);
                }
                ProductId::Id(product_id) => match write_access.0.by_product.get_mut(product_id) {
                    Some(items) => {
                        items.insert_or_replace(item);
                    }
                    None => {
                        let mut items = SortedVecWithStrKey::new();
                        items.insert_or_replace(item);
                        write_access
                            .0
                            .by_product
                            .insert(product_id.to_string(), items);
                    }
                },
            };
        }

        write_access.0.calc_usage();

        let snapshot = write_access.0.clone();

        write_access.1 = Arc::new(snapshot);

        write_access.1.clone()
    }

    /// Relocates a single secret from one scope to another atomically: the
    /// insert into the target scope and the removal from the source scope happen
    /// under a single write-lock acquisition and republish exactly one snapshot,
    /// so no observer (and no concurrent persist) can ever see the secret in both
    /// scopes at once. Emptied source product buckets are dropped from the map.
    pub async fn move_secret(
        &self,
        from: ProductId<'_>,
        to: ProductId<'_>,
        item: SecretItem,
    ) -> Arc<SecretsSnapshot> {
        let mut write_access = self.inner.write().await;

        let secret_id = item.id.clone();

        // Insert into the target scope.
        match to {
            ProductId::Shared => {
                write_access.0.shared.insert_or_replace(item);
            }
            ProductId::Id(product_id) => match write_access.0.by_product.get_mut(product_id) {
                Some(items) => {
                    items.insert_or_replace(item);
                }
                None => {
                    let mut items = SortedVecWithStrKey::new();
                    items.insert_or_replace(item);
                    write_access
                        .0
                        .by_product
                        .insert(product_id.to_string(), items);
                }
            },
        }

        // Remove from the source scope.
        match from {
            ProductId::Shared => {
                write_access.0.shared.remove(secret_id.as_str());
            }
            ProductId::Id(product_id) => {
                let now_empty = match write_access.0.by_product.get_mut(product_id) {
                    Some(items) => {
                        items.remove(secret_id.as_str());
                        items.is_empty()
                    }
                    None => false,
                };

                if now_empty {
                    write_access.0.by_product.remove(product_id);
                }
            }
        }

        write_access.0.calc_usage();

        let snapshot = write_access.0.clone();
        write_access.1 = Arc::new(snapshot);

        write_access.1.clone()
    }

    pub async fn remove(&self, product_id: ProductId<'_>, secret_id: &str) -> Option<SecretItem> {
        let mut write_access = self.inner.write().await;

        let removed_item = match product_id {
            ProductId::Shared => write_access.0.shared.remove(secret_id),
            ProductId::Id(product_id) => match write_access.0.by_product.get_mut(product_id) {
                Some(by_product) => by_product.remove(secret_id),
                None => None,
            },
        };

        write_access.0.calc_usage();

        if removed_item.is_some() {
            let snapshot = write_access.0.clone();
            write_access.1 = Arc::new(snapshot);
        }

        removed_item
    }
}
