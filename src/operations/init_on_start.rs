use crate::app_ctx::AppContext;

pub async fn init_on_start(app: &AppContext, file_init_from: &str) {
    let templates = crate::operations::get_all_templates(app).await;

    if templates.len() > 0 {
        println!(
            "Found {} templates. Skipping snapshot initialization",
            templates.len()
        );
        return;
    }

    let file_init = if file_init_from.starts_with("~") {
        let home_path = std::env::var("HOME").unwrap();
        file_init_from.replace("~", home_path.as_str())
    } else {
        file_init_from.to_string()
    };

    let snapshot = tokio::fs::read(file_init_from).await;
    if snapshot.is_err() {
        panic!(
            "Failed to read snapshot file {}. Error:{}",
            file_init,
            snapshot.err().unwrap()
        );
    }
    crate::operations::import_snapshot(&app, snapshot.as_ref().unwrap()).await;
}
