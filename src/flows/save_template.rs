use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{app_ctx::AppContext, models::*};

pub async fn save_template(app: &AppContext, product_id: &str, template_id: String, yaml: String) {
    let now = DateTimeAsMicroseconds::now();

    let mut new_entity = TemplateItem {
        id: template_id,
        content: yaml.into(),
        created: now,
        last_update: now,
    };

    if let Some(removed_entity) = app.templates.remove(product_id, &new_entity.id).await {
        new_entity.created = removed_entity.created;
    };

    let to_insert = [new_entity];

    app.templates_persistence.save(product_id, &to_insert).await;

    app.templates
        .insert(product_id, to_insert.into_iter())
        .await
}
