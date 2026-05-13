use std::sync::Arc;

use my_http_server::{
    macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput, WebContentType,
};

use crate::app_ctx::AppContext;

#[http_route(
    method: "GET",
    route: "/",
)]
pub struct IndexAction {
    pub app: Arc<AppContext>,
}

impl IndexAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    _action: &IndexAction,
    _: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    match tokio::fs::read("./wwwroot/index.html").await {
        Ok(bytes) => HttpOutput::Content {
            status_code: 200,
            headers: WebContentType::Html.into(),
            content: bytes,
        }
        .into_ok_result(false)
        .into(),
        Err(err) => {
            let placeholder = format!(
                "<html><head><title>settings-service</title></head><body>\
                <h1>UI is not built yet</h1>\
                <p>Run <code>./build.sh</code> inside <code>my-settings-ui/</code> to populate <code>wwwroot/</code>.</p>\
                <p>Reason: {err}</p>\
                </body></html>"
            );
            HttpOutput::Content {
                status_code: 200,
                headers: WebContentType::Html.into(),
                content: placeholder.into_bytes(),
            }
            .into_ok_result(false)
            .into()
        }
    }
}
