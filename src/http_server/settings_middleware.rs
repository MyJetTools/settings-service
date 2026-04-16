use std::sync::Arc;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpRequestHeaders, HttpServerMiddleware,
};
use rust_extensions::{date_time::DateTimeAsMicroseconds, str_utils::StrUtils};

use crate::app_ctx::AppContext;

pub struct SettingsMiddleware {
    app: Arc<AppContext>,
}

impl SettingsMiddleware {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for SettingsMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        let path = ctx.request.get_path();

        let mut product_id = None;
        let mut template_id = None;

        for (no, segment) in path.as_str().split('/').enumerate() {
            match no {
                0 => {}
                1 => {
                    if !segment.eq_case_insensitive("settings") {
                        return None;
                    }
                }
                2 => {
                    product_id = Some(segment);
                }
                3 => {
                    template_id = Some(segment);
                }
                _ => {
                    return None;
                }
            }
        }

        let Some(product_id) = product_id else {
            return None;
        };

        let Some(template_id) = template_id else {
            return None;
        };

        let content = self
            .app
            .templates
            .get_by_id(product_id, template_id, |itm| itm.content.clone())
            .await;

        let Some(content) = content else {
            return None;
        };

        let now = DateTimeAsMicroseconds::now();

        self.app
            .last_time_access
            .lock()
            .await
            .update(product_id, template_id, now);

        let env_info = ctx
            .request
            .get_headers()
            .try_get_case_insensitive_as_str("env-info")
            .ok()
            .flatten()
            .map(|v| v.to_string());

        let is_remote = !self.app.settings.is_local_env(env_info.as_deref());

        let secrets_snapshot = self.app.secrets.get_snapshot().await;

        let populated_content = crate::scripts::populate_secrets(
            &self.app,
            product_id.into(),
            &content,
            &secrets_snapshot,
            0,
            is_remote,
        );

        Some(HttpOutput::as_text(populated_content.into_string()).into_ok_result(false))
    }
}
