use rust_extensions::base64::IntoBase64;
use serde::*;

use crate::app_ctx::AppContext;

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

pub async fn export_snapshot(app: &AppContext, env: Option<&str>, max_level: u8) -> Vec<u8> {
    let templates = crate::scripts::templates::get_all(app).await;

    let secrets = crate::scripts::secrets::get_all(app, env).await;

    let mut result = SnapshotExportModel {
        templates: Vec::with_capacity(templates.len()),
        secrets: match &secrets {
            Some(secrets) => Vec::with_capacity(secrets.len()),
            None => Vec::new(),
        },
    };

    for template in templates {
        result.templates.push(TemplateExportModel {
            env: template.partition_key.to_string(),
            name: template.row_key.to_string(),
            yaml: template.yaml_template.as_bytes().into_base64(),
        })
    }

    if let Some(secrets) = secrets {
        for secret in secrets {
            if secret.get_level() > max_level {
                continue;
            }

            let secret_name = secret.get_secret_name();

            if let Some(value) = crate::scripts::secrets::decode(&secret, &app.aes_key) {
                result.secrets.push(SecretExportModel {
                    name: secret_name.to_string(),
                    value: value.content,
                    level: secret.get_level(),
                });
            }
        }
    }

    serde_json::to_vec(&result).unwrap()
}
