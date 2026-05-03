use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{app_ctx::AppContext, models::Product};

pub async fn save_product(
    app: &AppContext,
    id: String,
    description: String,
    prompt: String,
) {
    let snapshot = app.products.get_snapshot().await;
    let now = DateTimeAsMicroseconds::now();

    let created = snapshot
        .get(id.as_str())
        .map(|existing| existing.created)
        .unwrap_or(now);

    let product = Product {
        id,
        description,
        prompt,
        created,
        updated: now,
    };

    let new_snapshot = app.products.upsert(product).await;
    app.products_persistence
        .save_all(&new_snapshot.as_vec())
        .await;
}
