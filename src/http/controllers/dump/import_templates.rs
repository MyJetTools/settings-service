use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use crate::{app_ctx::AppContext, my_no_sql::TemplateMyNoSqlEntity};

use super::contracts::{ImportSettingsTemplateAction, SettingTemplateDumpModel};

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/dump/import/templates",
    description: "Import Templates",
    summary: "Import Templates",
    controller: "Dump",
    input_data: "ImportSettingsTemplateAction",
    result:[
        {status_code: 202, description: "Ok response"},
    ]
)]

pub struct ImportTemplatesAction {
    app: Arc<AppContext>,
}

impl ImportTemplatesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ImportTemplatesAction,
    input_data: ImportSettingsTemplateAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let dump_data: Vec<TemplateMyNoSqlEntity> =
        serde_json::from_slice::<Vec<SettingTemplateDumpModel>>(&input_data.dump.content)
            .unwrap()
            .iter()
            .map(|x| {
                let entity: SettingTemplateDumpModel = x.to_owned();
                let entity = entity.into();
                return entity;
            })
            .collect();

    action
        .app
        .templates_storage
        .bulk_insert_or_replace(dump_data.as_slice())
        .await
        .unwrap();

    crate::operations::initialize_templates(&action.app, true).await;
    HttpOutput::Empty.into_ok_result(false)
}
