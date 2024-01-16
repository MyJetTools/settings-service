use rust_extensions::{base64::FromBase64, date_time::DateTimeAsMicroseconds};
use serde::*;

use crate::{app_ctx::AppContext, caches::SecretValue, my_no_sql::TemplateMyNoSqlEntity};

#[derive(Serialize, Deserialize)]
pub struct SnapshotExportModel {
    pub templates: Vec<TemplateExportModel>,
    pub secrets: Vec<SecretExportModel>,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateExportModel {
    pub env: String,
    pub name: String,
    pub yaml: String,
}

#[derive(Serialize, Deserialize)]
pub struct SecretExportModel {
    pub name: String,
    pub value: String,
    pub level: u8,
}

pub async fn import_snapshot(app: &AppContext, snapshot: &[u8]) {
    let model: SnapshotExportModel = serde_json::from_slice(snapshot).unwrap();

    for template in model.templates {
        let template = TemplateMyNoSqlEntity {
            partition_key: template.env,
            row_key: template.name,
            time_stamp: "".to_string(),
            create_date: DateTimeAsMicroseconds::now().to_rfc3339(),
            last_update_date: DateTimeAsMicroseconds::now().to_rfc3339(),
            yaml_template: String::from_utf8(template.yaml.from_base64().unwrap()).unwrap(),
        };

        app.templates_storage
            .insert_or_replace_entity(&template)
            .await
            .unwrap();
    }

    for secret in model.secrets {
        crate::operations::update_secret(
            app,
            secret.name,
            SecretValue {
                content: secret.value,
                level: secret.level,
            },
        )
        .await;
    }
}
