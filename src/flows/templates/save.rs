use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{app_ctx::AppContext, my_no_sql::TemplateMyNoSqlEntity};

pub async fn save(app: &AppContext, env: String, name: String, yaml: String) {
    let new_entity =
        if let Some(entity_to_update) = super::get(app, env.as_str(), name.as_str()).await {
            entity_to_update.update_yaml(yaml)
        } else {
            TemplateMyNoSqlEntity {
                partition_key: env,
                row_key: name,
                create_date: DateTimeAsMicroseconds::now().to_rfc3339(),
                last_update_date: DateTimeAsMicroseconds::now().to_rfc3339(),
                yaml_template: yaml,
                time_stamp: Default::default(),
            }
        };

    app.templates_storage
        .insert_or_replace_entity(&new_entity)
        .await
        .unwrap();
}
