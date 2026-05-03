use std::collections::BTreeMap;

use crate::{app_ctx::AppContext, models::Product};

pub struct ProductListItem {
    pub id: String,
    pub description: Option<String>,
    pub prompt: Option<String>,
    pub templates_count: i32,
    pub secrets_count: i32,
    pub has_metadata: bool,
}

pub async fn get_all_products(app: &AppContext) -> Vec<ProductListItem> {
    let mut by_id: BTreeMap<String, ProductListItem> = BTreeMap::new();

    let explicit = app.products.get_snapshot().await;
    for product in explicit.by_id.values() {
        by_id.insert(product.id.clone(), entry_from_product(product));
    }

    let secrets = app.secrets.get_snapshot().await;
    for (product_id, items) in secrets.by_product.iter() {
        let entry = by_id
            .entry(product_id.clone())
            .or_insert_with(|| empty_entry(product_id));
        entry.secrets_count += items.len() as i32;
    }

    let templates_per_product = app
        .templates
        .find_into_vec(|product_id, _| Some(product_id.to_string()))
        .await;
    for product_id in templates_per_product {
        let entry = by_id
            .entry(product_id.clone())
            .or_insert_with(|| empty_entry(&product_id));
        entry.templates_count += 1;
    }

    by_id.into_values().collect()
}

fn entry_from_product(product: &Product) -> ProductListItem {
    ProductListItem {
        id: product.id.clone(),
        description: Some(product.description.clone()),
        prompt: Some(product.prompt.clone()),
        templates_count: 0,
        secrets_count: 0,
        has_metadata: true,
    }
}

fn empty_entry(id: &str) -> ProductListItem {
    ProductListItem {
        id: id.to_string(),
        description: None,
        prompt: None,
        templates_count: 0,
        secrets_count: 0,
        has_metadata: false,
    }
}
