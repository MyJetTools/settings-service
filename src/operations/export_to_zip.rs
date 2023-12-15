use std::io::Write;

use crate::app_ctx::AppContext;

pub async fn export_to_zip(app: &AppContext, max_level: u8) -> Vec<u8> {
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));

    let templates = crate::operations::get_all_templates(app).await;

    for template in templates {
        let file_name = format!(
            "templates/{}/{}.json",
            template.partition_key, template.row_key
        );
        zip.start_file(file_name, Default::default()).unwrap();
        let json = serde_json::to_string(template.as_ref()).unwrap();
        zip.write_all(json.as_bytes()).unwrap();
    }

    let secrets = crate::operations::get_all_secrets(app).await;

    if let Some(secrets) = secrets {
        for secret in secrets {
            if secret.get_level() > max_level {
                continue;
            }

            let file_name = format!("secrets/{}.json", secret.row_key);
            zip.start_file(file_name, Default::default()).unwrap();
            let json = serde_json::to_string(&secret).unwrap();
            zip.write_all(json.as_bytes()).unwrap();
        }
    }

    let result = zip.finish().unwrap();
    result.into_inner()
}
