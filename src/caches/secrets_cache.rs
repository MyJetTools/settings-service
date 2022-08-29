use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::sync::Mutex;

use crate::my_no_sql::SecretMyNoSqlEntity;

pub struct SecretsCache {
    initialized: AtomicBool,
    items: Mutex<BTreeMap<String, Arc<SecretMyNoSqlEntity>>>,
}

impl SecretsCache {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(BTreeMap::new()),
            initialized: AtomicBool::new(false),
        }
    }

    pub async fn init(&self, items: Option<Vec<SecretMyNoSqlEntity>>) {
        let mut write_access = self.items.lock().await;

        if let Some(items) = items {
            for itm in items {
                write_access.insert(itm.row_key.clone(), Arc::new(itm));
            }
        }
        self.initialized.store(true, Ordering::SeqCst);
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }

    pub async fn has_value(&self, key: &str) -> bool {
        let read_access = self.items.lock().await;
        read_access.contains_key(key)
    }

    pub async fn get_all(&self) -> Vec<Arc<SecretMyNoSqlEntity>> {
        let read_access = self.items.lock().await;

        let mut result = Vec::new();

        for itm in read_access.values() {
            result.push(itm.clone());
        }

        result
    }
}
