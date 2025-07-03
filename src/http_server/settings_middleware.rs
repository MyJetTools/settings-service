use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpServerMiddleware};
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

        let mut env = None;
        let mut name = None;

        for (no, segment) in path.as_str().split('/').enumerate() {
            match no {
                0 => {}
                1 => {
                    if !segment.eq_case_insensitive("settings") {
                        return None;
                    }
                }
                2 => {
                    env = Some(segment);
                }
                3 => {
                    name = Some(segment);
                }
                _ => {
                    return None;
                }
            }
        }

        if env.is_none() {
            return None;
        }

        if name.is_none() {
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
