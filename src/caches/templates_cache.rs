use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::sync::Mutex;

use crate::my_no_sql::TemplateMyNoSqlEntity;

pub struct TemplatesCache {
    initialized: AtomicBool,
    items: Mutex<Option<Vec<Arc<TemplateMyNoSqlEntity>>>>,
}

impl TemplatesCache {
    pub fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            items: Mutex::new(None),
        }
    }

    pub async fn init(&self, src: Option<Vec<TemplateMyNoSqlEntity>>) {
        let mut result = Vec::new();

        if let Some(src) = src {
            for itm in src {
                result.push(Arc::new(itm));
            }
        }

        let mut write_access = self.items.lock().await;

        write_access.replace(result);
        self.initialized.store(true, Ordering::SeqCst);
    }

    pub async fn get_all(&self) -> Vec<Arc<TemplateMyNoSqlEntity>> {
        let read_access = self.items.lock().await;
        let result = read_access.as_ref().unwrap();
        return result.clone();
    }

    pub async fn get(&self, env: &str, name: &str) -> Option<Arc<TemplateMyNoSqlEntity>> {
        let read_access = self.items.lock().await;
        let result = read_access.as_ref().unwrap();

        for itm in result {
            if itm.partition_key == env && itm.row_key == name {
                return Some(itm.clone());
            }
        }

        None
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }
}
