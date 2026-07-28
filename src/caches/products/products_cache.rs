use std::sync::Arc;

use tokio::sync::RwLock;

use crate::models::Product;

use super::ProductsSnapshot;

#[derive(Default)]
pub struct ProductsCache {
    inner: RwLock<(ProductsSnapshot, Arc<ProductsSnapshot>)>,
}

impl ProductsCache {
    pub async fn init(&self, items: Vec<Product>) {
        let mut snapshot = ProductsSnapshot::default();
        for item in items {
            snapshot.by_id.insert(item.id.clone(), item);
        }

        let shared = Arc::new(snapshot.clone());
        let mut write_access = self.inner.write().await;
        write_access.0 = snapshot;
        write_access.1 = shared;
    }

    pub async fn get_snapshot(&self) -> Arc<ProductsSnapshot> {
        let read_access = self.inner.read().await;
        read_access.1.clone()
    }

    pub async fn upsert(&self, product: Product) -> Arc<ProductsSnapshot> {
        let mut write_access = self.inner.write().await;
        write_access.0.by_id.insert(product.id.clone(), product);
        let snapshot = Arc::new(write_access.0.clone());
        write_access.1 = snapshot.clone();
        snapshot
    }

    pub async fn delete(&self, id: &str) -> Arc<ProductsSnapshot> {
        let mut write_access = self.inner.write().await;
        write_access.0.by_id.remove(id);
        let snapshot = Arc::new(write_access.0.clone());
        write_access.1 = snapshot.clone();
        snapshot
    }
}
