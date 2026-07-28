use rust_extensions::file_utils::FilePath;

use crate::{models::Product, persistence::models::*};

pub struct ProductsPersistence {
    path: FilePath,
}

impl ProductsPersistence {
    pub fn new(mut path: FilePath) -> Self {
        path.append_segment("product-descriptions.json");
        Self { path }
    }

    pub async fn load_all(&self) -> Vec<Product> {
        let content = match tokio::fs::read(self.path.as_str()).await {
            Ok(content) => content,
            Err(_) => return Vec::new(),
        };

        let file_model = ProductsFileModel::from_slice(&content);

        file_model
            .products
            .into_iter()
            .map(|(id, item)| item.into_product(id))
            .collect()
    }

    pub async fn save_all(&self, products: &[Product]) {
        let mut file_model = ProductsFileModel::default();

        for product in products {
            file_model
                .products
                .insert(product.id.clone(), ProductFileItem::from_product(product));
        }

        let bytes = file_model.to_vec();
        tokio::fs::write(self.path.as_str(), bytes.as_slice())
            .await
            .unwrap();
    }
}
