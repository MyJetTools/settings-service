use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::sync::Mutex;

use crate::my_no_sql::TemplateMyNoSqlEntity;

pub struct SecretUsage {
    pub env: String,
    pub name: String,
    pub yaml: String,
}

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

    pub async fn save(&self, entity: TemplateMyNoSqlEntity) {
        let mut write_access = self.items.lock().await;

        let index = get_no(
            write_access.as_ref().unwrap(),
            &entity.partition_key,
            &entity.row_key,
        );

        if let Some(index) = index {
            write_access.as_mut().unwrap().remove(index);
            write_access
                .as_mut()
                .unwrap()
                .insert(index, Arc::new(entity));
        } else {
            write_access.as_mut().unwrap().push(Arc::new(entity));
        }
    }

    pub async fn delete(&self, env: &str, name: &str) {
        let mut write_access = self.items.lock().await;

        let index = get_no(write_access.as_ref().unwrap(), env, name);

        if let Some(index) = index {
            write_access.as_mut().unwrap().remove(index);
        }
    }

    pub async fn get_used_secret_amount(&self, contains: &str) -> usize {
        let read_access = self.items.lock().await;

        let mut result = 0;
        for itm in read_access.as_ref().unwrap() {
            if itm.yaml_template.contains(contains) {
                result += 1;
            }
        }

        result
    }
}

fn get_no(src: &Vec<Arc<TemplateMyNoSqlEntity>>, env: &str, name: &str) -> Option<usize> {
    let mut result = 0;

    for itm in src {
        if itm.partition_key == env && itm.row_key == name {
            return Some(result);
        }
        result += 1;
    }

    None
}
