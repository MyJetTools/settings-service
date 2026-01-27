use std::collections::BTreeMap;

use rust_extensions::file_utils::FilePath;
use tokio::sync::Mutex;

use crate::{
    models::*,
    persistence::models::{TemplateFileData, TemplatesFileModel},
};

pub struct TemplatesPersistence {
    path: FilePath,
    pub content: Mutex<Option<TemplatesFileModel>>,
}

impl TemplatesPersistence {
    pub fn new(mut path: FilePath) -> Self {
        path.append_segment("templates.json");
        Self {
            path,
            content: Default::default(),
        }
    }

    pub async fn get_file_content(&self) -> TemplatesFileModel {
        let mut write_access = self.content.lock().await;

        if let Some(result) = write_access.take() {
            return result;
        }

        if let Ok(content) = tokio::fs::read(self.path.as_str()).await {
            return TemplatesFileModel::from_slice(&content);
        };

        TemplatesFileModel::default()
    }

    async fn set_content(&self, content: TemplatesFileModel) {
        let mut write_access = self.content.lock().await;
        *write_access = Some(content);
    }

    pub async fn save(&self, product_id: &str, items: &[TemplateItem]) {
        let mut file_content = self.get_file_content().await;

        for item in items {
            let id = item.id.to_string();

            let item = TemplateFileData {
                content: item.content.to_base_64(),
                created: item.created.unix_microseconds,
                updated: item.last_update.unix_microseconds,
            };

            match file_content.items.get_mut(product_id) {
                Some(items) => {
                    items.insert(id, item);
                }
                None => {
                    let mut items = BTreeMap::new();
                    items.insert(id, item);
                    file_content.items.insert(product_id.to_string(), items);
                }
            }
        }

        let as_vec = file_content.to_vec();
        tokio::fs::write(self.path.as_str(), as_vec.as_slice())
            .await
            .unwrap();

        self.set_content(file_content).await;
    }

    pub async fn delete(&self, product_id: &str, template_id: &str) {
        let mut file_content = self.get_file_content().await;

        let delete_product = if let Some(items) = file_content.items.get_mut(product_id) {
            items.remove(template_id);
            items.len() == 0
        } else {
            return;
        };

        if delete_product {
            file_content.items.remove(product_id);
        }

        let as_vec = file_content.to_vec();
        tokio::fs::write(self.path.as_str(), as_vec.as_slice())
            .await
            .unwrap();

        self.set_content(file_content).await;
    }
}
