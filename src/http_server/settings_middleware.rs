use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpServerMiddleware};
use rust_extensions::date_time::DateTimeAsMicroseconds;

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

        let mut env = None;
        let mut name = None;

        let mut no = 0;

        for segment in path.as_str().split('/') {
            if no == 1 {
                if rust_extensions::str_utils::compare_strings_case_insensitive(segment, "settings")
                {
                    return None;
                }
            }
            if no == 2 {
                env = Some(segment);
            } else if no == 3 {
                name = Some(segment);
            }
            no += 1;
        }

        if no != 4 {
            return None;
        }

        let env = env.unwrap();
        let name = name.unwrap();

        let yaml = crate::flows::templates::get_populated_template(&self.app, env, name).await;

        if yaml.is_none() {
            return None;
        }

        self.app
            .last_request
            .update(env, name, DateTimeAsMicroseconds::now())
            .await;

        Some(HttpOutput::as_text(yaml.unwrap()).into_ok_result(false))
    }
}
