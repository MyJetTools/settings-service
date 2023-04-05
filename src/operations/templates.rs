use std::sync::Arc;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{app_ctx::AppContext, my_no_sql::TemplateMyNoSqlEntity};

pub async fn get_all(app: &AppContext) -> Vec<Arc<TemplateMyNoSqlEntity>> {
    if !app.templates_cache.is_initialized() {
        let templates = app.templates_storage.get_all().await.unwrap();
        app.templates_cache.init(templates).await;
    }

    app.templates_cache.get_all().await
}

pub async fn get(app: &AppContext, evn: &str, name: &str) -> Option<Arc<TemplateMyNoSqlEntity>> {
    if !app.templates_cache.is_initialized() {
        let templates = app.templates_storage.get_all().await.unwrap();
        app.templates_cache.init(templates).await;
    }

    app.templates_cache.get(evn, name).await
}

pub async fn post(app: &AppContext, evn: String, name: String, yaml: String) {
    if !app.templates_cache.is_initialized() {
        let templates = app.templates_storage.get_all().await.unwrap();
        app.templates_cache.init(templates).await;
    }

    let new_entity = if let Some(entity_to_update) =
        app.templates_cache.get(evn.as_str(), name.as_str()).await
    {
        entity_to_update.update_yaml(yaml)
    } else {
        TemplateMyNoSqlEntity {
            partition_key: evn,
            row_key: name,
            create_date: DateTimeAsMicroseconds::now().to_rfc3339(),
            last_update_date: DateTimeAsMicroseconds::now().to_rfc3339(),
            yaml_template: yaml,
            time_stamp: DateTimeAsMicroseconds::now().to_rfc3339(),
        }
    };

    app.templates_storage
        .insert_or_replace_entity(&new_entity)
        .await
        .unwrap();

    app.templates_cache.save(new_entity).await;
}

pub async fn delete(app: &AppContext, evn: String, name: String) {
    if !app.templates_cache.is_initialized() {
        let templates = app.templates_storage.get_all().await.unwrap();
        app.templates_cache.init(templates).await;
    }

    app.templates_storage
        .delete_row(evn.as_str(), name.as_str())
        .await
        .unwrap();

    app.templates_cache.delete(evn.as_str(), &name).await;
}

pub async fn get_populated_template(app: &AppContext, evn: &str, name: &str) -> Option<String> {
    let template = get(app, evn, name).await?;
    super::populate_with_secrets(app, template.yaml_template.as_str(), 0)
        .await
        .into()
}
