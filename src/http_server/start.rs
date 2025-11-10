use std::{net::SocketAddr, sync::Arc};

use my_http_server::controllers::swagger::SwaggerMiddleware;
use my_http_server::{MyHttpServer, StaticFilesMiddleware};

use crate::app_ctx::AppContext;

use super::SettingsMiddleware;

pub fn start(app: &Arc<AppContext>) {
    let mut http_server =
        MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], app.settings.http_port)));

    let unix_socket = std::env::var("UNIX_SOCKET");

    let mut unix_socket = if let Ok(unix_socket) = unix_socket {
        Some(MyHttpServer::new_as_unix_socket(unix_socket))
    } else {
        None
    };

    let settings_middleware = Arc::new(SettingsMiddleware::new(app.clone()));

    if let Some(unix_socket) = unix_socket.as_mut() {
        unix_socket.add_middleware(settings_middleware.clone());
    }

    http_server.add_middleware(settings_middleware);

    let controllers = super::build_controllers(app);

    let swagger_middleware = SwaggerMiddleware::new(
        controllers.clone(),
        crate::app_ctx::APP_NAME.to_string(),
        crate::app_ctx::APP_VERSION.to_string(),
    );

    let swagger_middleware = Arc::new(swagger_middleware);

    if let Some(unix_socket) = unix_socket.as_mut() {
        unix_socket.add_middleware(swagger_middleware.clone());
        unix_socket.add_middleware(controllers.clone());
    }

    http_server.add_middleware(swagger_middleware);

    http_server.add_middleware(controllers);

    let static_files_middleware = if cfg!(debug_assertions) {
        Arc::new(StaticFilesMiddleware::new(
            Some(vec![my_http_server::FilesMapping::new(
                "/typescript",
                "./typescript",
            )]),
            None,
        ))
    } else {
        Arc::new(StaticFilesMiddleware::new(None, None))
    };

    if let Some(unix_socket) = unix_socket.as_mut() {
        unix_socket.add_middleware(static_files_middleware.clone());
    }

    http_server.add_middleware(static_files_middleware);

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());

    if let Some(unix_socket) = unix_socket.as_mut() {
        unix_socket.start(app.app_states.clone(), my_logger::LOGGER.clone());
    }
}
