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

    fn get_favicon_file_name(&self) -> &str {
        match self.app.settings.get_favicon_suffix() {
            crate::settings::FaviconColor::Default => "favicon.png",
            crate::settings::FaviconColor::Green => "favicon-green.png",
            crate::settings::FaviconColor::Pink => "favicon-pink.png",
            crate::settings::FaviconColor::Black => "favicon-black.png",
            crate::settings::FaviconColor::Yellow => "favicon-yellow.png",
        }
    }

    fn get_env_color(&self) -> &str {
        match self.app.settings.get_favicon_suffix() {
            crate::settings::FaviconColor::Default => "orange",
            crate::settings::FaviconColor::Green => "green",
            crate::settings::FaviconColor::Pink => "pink",
            crate::settings::FaviconColor::Black => "gray",
            crate::settings::FaviconColor::Yellow => "yellow",
        }
    }
}

async fn handle_request(
    action: &IndexAction,
    _: &mut HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    if cfg!(debug_assertions) {
        let content = format!(
            r###"<html><head><title>{ver} settings-service</title>
            <link rel="icon" type="image/x-icon" href="/img/{favicon_file_name}">
            <link href="/css/bootstrap.css" rel="stylesheet" type="text/css" />
            <link href="/css/site.css" rel="stylesheet" type="text/css" />
            <script src="/lib/jquery.js"></script>
            <script src="/js/HtmlStaticElement.js"></script>
            <script src="/js/AppContext.js"></script>
            <script src="/js/Utils.js"></script>
            <script src="/js/Dialog.js"></script>
            <script src="/js/dialogs/ShowYamlDialog.js"></script>
            <script src="/js/dialogs/EditTemplate.js"></script>
            <script src="/js/dialogs/ShowSecretUsage.js"></script>     
            <script src="/js/dialogs/ShowUsageSecretOfSecrets.js"></script>                
            <script src="/js/dialogs/EditSecret.js"></script>
            <script src="/js/dialogs/ConfirmDeleteTemplate.js"></script>
            <script src="/js/dialogs/ConfirmDeleteSecret.js"></script>
            <script src="/js/Actions.js"></script>
            <script src="/js/HtmlMain.js"></script>
            <script src="/js/main.js"></script>
            </head><body data-env="debug"></body></html>"###,
            ver = crate::app_ctx::APP_VERSION,
            favicon_file_name = action.get_favicon_file_name()
        );

        HttpOutput::Content {
            status_code: 200,
            headers: None,
            content_type: Some(WebContentType::Html),
            content: content.into_bytes(),
            set_cookies: None,
        }
        .into_ok_result(true)
        .into()
    } else {
        let content = format!(
            r###"<html><head><title>{app_version} settings-service-{env}</title>
            <link rel="icon" type="image/x-icon" href="/img/{favicon_file_name}">
            <link href="/css/bootstrap.css" rel="stylesheet" type="text/css" />
            <link href="/css/site.css" rel="stylesheet" type="text/css" />
            <script src="/lib/jquery.js"></script><script src="/js/app.js?ver={rnd}"></script>
            </head><body data-env="{env}" data-env-color="{env_color}"></body></html>"###,
            app_version = crate::app_ctx::APP_VERSION,
            env = action.app.settings.env,
            rnd = action.app.process_id,
            env_color = action.get_env_color(),
            favicon_file_name = action.get_favicon_file_name()
        );
        HttpOutput::Content {
            status_code: 200,
            headers: None,
            content_type: Some(WebContentType::Html),
            content: content.into_bytes(),
            set_cookies: None,
        }
        .into_ok_result(true)
        .into()
    }
}
