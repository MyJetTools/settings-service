use std::sync::Arc;

use my_grpc_extensions::server::*;
use my_grpc_extensions::StreamedResponseWriter;

use crate::app_ctx::AppContext;
use crate::secrets_grpc::*;

generate_server!(
    proto_file: "./proto/SecretsService.proto",
    crate_ns: "crate::secrets_grpc",
);

async fn get_all(
    app: &Arc<AppContext>,
    request: GetAllSecretsGrpcRequest,
) -> StreamedResponseWriter<SecretGrpcModel> {
    let writer = StreamedResponseWriter::new(1024);
    let producer = writer.get_stream_producer();

    let app = app.clone();
    tokio::spawn(async move {
        let items = crate::flows::get_all_secrets(
            &app,
            request.product_id.as_str(),
            request.include_shared,
        )
        .await;
        for item in items {
            producer.send(item).await.unwrap();
        }
    });

    writer
}

async fn get(app: &Arc<AppContext>, request: GetSecretGrpcRequest) -> SecretValueGrpcModel {
    crate::flows::get_secret(
        app,
        request.product_id.as_deref().into(),
        &request.secret_id,
    )
    .await
}

async fn save(app: &Arc<AppContext>, request: SaveSecretGrpcRequest) {
    crate::flows::save_secret(
        app,
        request.product_id.as_deref().into(),
        request.id,
        request.value,
        request.remote_value,
        request.level as u8,
    )
    .await;
}

async fn delete(app: &Arc<AppContext>, request: DeleteSecretGrpcRequest) {
    crate::flows::delete_secret(
        app,
        request.product_id.as_deref().into(),
        &request.secret_id,
    )
    .await;
}

async fn get_templates_usage(
    app: &Arc<AppContext>,
    request: GetTemplatesUsageGrpcRequest,
) -> GetTemplatesUsageGrpcResponse {
    let templates = crate::flows::get_templates_used_by_the_secret(
        app,
        request.product_id.as_deref().into(),
        &request.secret_id,
    )
    .await;
    GetTemplatesUsageGrpcResponse { templates }
}

async fn get_secrets_usage(
    app: &Arc<AppContext>,
    request: DeleteSecretGrpcRequest,
) -> GetSecretsUsageGrpcResponse {
    let secrets = crate::flows::get_secrets_used_by_the_secret(
        app,
        request.product_id.as_deref().into(),
        &request.secret_id,
    )
    .await;
    GetSecretsUsageGrpcResponse { secrets }
}
