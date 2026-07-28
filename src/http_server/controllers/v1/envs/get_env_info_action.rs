use std::sync::Arc;

use my_http_server::{
    macros::{http_route, MyHttpObjectStructure},
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput,
};
use serde::{Deserialize, Serialize};

use crate::app_ctx::AppContext;

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct EnvInfoHttpModel {
    pub name: String,
    pub color: String,
}

#[http_route(
    method: "GET",
    route: "/api/v1/env",
    description: "Returns environment metadata (name, color) of this settings-service instance",
    summary: "Get env info",
    controller: "V1::Envs",
    result: [
        {status_code: 200, description: "Ok response", model: "EnvInfoHttpModel"},
    ]
)]
pub struct GetEnvInfoAction {
    app: Arc<AppContext>,
}

impl GetEnvInfoAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetEnvInfoAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let color = match action.app.settings.get_favicon_suffix() {
        crate::settings::FaviconColor::Default => "orange",
        crate::settings::FaviconColor::Green => "green",
        crate::settings::FaviconColor::Pink => "pink",
        crate::settings::FaviconColor::Black => "gray",
        crate::settings::FaviconColor::Yellow => "yellow",
    };

    let model = EnvInfoHttpModel {
        name: action.app.settings.env.clone(),
        color: color.to_string(),
    };

    HttpOutput::as_json(model).into_ok_result(false)
}
