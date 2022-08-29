use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput, WebContentType};
use my_http_server_controllers::controllers::{
    actions::GetAction, documentation::HttpActionDescription,
};

use crate::app_ctx::AppContext;

pub struct IndexAction {
    pub app: Arc<AppContext>,
}

impl IndexAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl GetAction for IndexAction {
    fn get_route(&self) -> &str {
        "/"
    }

    fn get_description(&self) -> Option<HttpActionDescription> {
        None
    }

    async fn handle_request(&self, _: &mut HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        if cfg!(debug_assertions) {
            let content = format!(
                r###"<html><head><title>{} traffic-forwarder-a</title>
                <link href="/css/bootstrap.css" rel="stylesheet" type="text/css" />
                <link href="/css/site.css" rel="stylesheet" type="text/css" />
                <script src="/lib/jquery.js"></script>
                <script src="/js/HtmlStaticElement.js"></script>
                <script src="/js/AppContext.js"></script>
                <script src="/js/Dialog.js"></script>
                <script src="/js/dialogs/EditTemplate.js"></script>
                <script src="/js/dialogs/EditSecret.js"></script>
                <script src="/js/dialogs/ConfirmDeleteTemplate.js"></script>
                <script src="/js/Actions.js"></script>
                <script src="/js/HtmlMain.js"></script>
                <script src="/js/main.js"></script>
                </head><body></body></html>"###,
                ver = crate::app_ctx::APP_VERSION,
            );

            HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Html),
                content: content.into_bytes(),
            }
            .into_ok_result(true)
            .into()
        } else {
            let content = format!(
                r###"<html><head><title>{} traffic-forwarder-a</title>
                <link href="/css/bootstrap.css" rel="stylesheet" type="text/css" />
                <link href="/css/site.css" rel="stylesheet" type="text/css" />
                <script src="/lib/jquery.js"></script><script src="/js/app.js?ver={rnd}"></script>
                </head><body></body></html>"###,
                ver = crate::app_ctx::APP_VERSION,
                rnd = self.app.process_id
            );
            HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Html),
                content: content.into_bytes(),
            }
            .into_ok_result(true)
            .into()
        }
    }
}
