use std::sync::Arc;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpServerMiddleware,
    HttpServerRequestFlow,
};
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
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let path = ctx.request.get_path_lower_case();

        let mut env = None;
        let mut name = None;

        let mut no = 0;

        for segment in path.split('/') {
            if no == 1 {
                if segment != "settings" {
                    return get_next.next(ctx).await;
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
            return get_next.next(ctx).await;
        }

        let env = env.unwrap();
        let name = name.unwrap();

        let yaml = crate::operations::templates::get_populated_template(&self.app, env, name).await;

        if yaml.is_none() {
            return get_next.next(ctx).await;
        }

        self.app
            .last_request
            .update(env, name, DateTimeAsMicroseconds::now())
            .await;

        HttpOutput::as_text(yaml.unwrap()).into_ok_result(false)
    }
}
