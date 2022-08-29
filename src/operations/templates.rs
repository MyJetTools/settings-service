use std::sync::Arc;

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

pub async fn get_populated_template(app: &AppContext, evn: &str, name: &str) -> Option<String> {
    let template = get(app, evn, name).await?;
    populate_template(app, template.yaml_template.as_str())
        .await
        .into()
}

pub async fn populate_template(app: &AppContext, template: &str) -> String {
    let template = template.as_bytes();

    let mut result = Vec::new();

    let mut first = false;
    let mut second = false;
    let mut start = 0;

    for i in 0..template.len() {
        let b = template[i];

        if b == b'$' {
            first = true;
            continue;
        }

        if first && b == b'{' {
            second = true;
            start = i + 1;
            continue;
        }

        if first && second {
            if b == b'}' {
                let key = std::str::from_utf8(&template[start..i]).unwrap();

                if let Some(value) = super::secrets_values::get(app, key).await {
                    result.extend_from_slice(value.as_bytes());
                }

                first = false;
                second = false;
            }
            continue;
        }

        result.push(b);
        first = false;
        second = false;
    }
    String::from_utf8(result).unwrap()
}
