use std::{net::SocketAddr, sync::Arc};

use my_http_server::{MyHttpServer, StaticFilesMiddleware};
use my_http_server_controllers::swagger::SwaggerMiddleware;

use crate::app_ctx::AppContext;

use super::SettingsMiddleware;

pub fn start(app: &Arc<AppContext>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8000)));

    http_server.add_middleware(Arc::new(SettingsMiddleware::new(app.clone())));

    let controllers = super::build_controllers(app);

    let swagger_middleware = SwaggerMiddleware::new(
        controllers.clone(),
        crate::app_ctx::APP_NAME.to_string(),
        crate::app_ctx::APP_VERSION.to_string(),
    );

    http_server.add_middleware(Arc::new(swagger_middleware));

    http_server.add_middleware(controllers);

    if cfg!(debug_assertions) {
        http_server.add_middleware(Arc::new(StaticFilesMiddleware::new(
            Some(vec![my_http_server::FilesMapping::new(
                "/typescript",
                "./typescript",
            )]),
            None,
        )));
    } else {
        http_server.add_middleware(Arc::new(StaticFilesMiddleware::new(None, None)));
    }

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());
}
