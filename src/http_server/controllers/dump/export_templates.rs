use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::app_ctx::AppContext;

use super::contracts::SettingTemplateDumpModel;

#[http_route(
    method: "GET",
    route: "/api/dump/export/templates",
    description: "Export Templates",
    summary: "Export Templates",

    controller: "Dump",
    result:[
        {status_code: 202, description: "Ok response"},
    ]
)]

pub struct ExportTemplatesAction {
    app: Arc<AppContext>,
}

impl ExportTemplatesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ExportTemplatesAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let templates = crate::scripts::templates::get_all(&action.app).await;

    let dump_data = templates
        .iter()
        .map(|x: &Arc<crate::my_no_sql::TemplateMyNoSqlEntity>| {
            SettingTemplateDumpModel::new(x.as_ref())
        })
        .collect::<Vec<SettingTemplateDumpModel>>();

    let json = serde_json::to_string(&dump_data).unwrap();

    HttpOutput::File {
        file_name: "templates_dump.json".to_string(),
        content: json.as_bytes().to_vec(),
    }
    .into_ok_result(false)
}
