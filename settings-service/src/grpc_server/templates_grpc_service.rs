use std::sync::Arc;

use my_grpc_extensions::server::*;
use my_grpc_extensions::StreamedResponseWriter;

use crate::app_ctx::AppContext;
use crate::templates_grpc::*;

generate_server!(
    proto_file: "./proto/TemplatesService.proto",
    crate_ns: "crate::templates_grpc",
);

async fn get_server_info(app: &Arc<AppContext>, _request: ()) -> ServerInfoResponse {
    ServerInfoResponse {
        env_name: app.settings.env.to_string(),
    }
}

async fn get_template_content(
    app: &Arc<AppContext>,
    request: GetTemplateContentGrpcRequest,
) -> GetTemplateContentGrpcResponse {
    let content = app
        .templates
        .get_by_id(&request.product_id, &request.template_id, |itm| {
            itm.content.to_string()
        })
        .await
        .unwrap_or_default();

    GetTemplateContentGrpcResponse { content }
}

async fn get_all(
    app: &Arc<AppContext>,
    _request: (),
) -> StreamedResponseWriter<TemplateListItemGrpcModel> {
    let writer = StreamedResponseWriter::new(1024);
    let producer = writer.get_stream_producer();

    let app = app.clone();
    tokio::spawn(async move {
        let items = crate::flows::get_all_templates(&app).await;
        for item in items {
            producer.send(item).await.unwrap();
        }
    });

    writer
}

async fn save(app: &Arc<AppContext>, request: SaveTemplateGrpcRequest) {
    crate::flows::save_template(
        app,
        &request.product_id,
        request.template_id,
        request.yaml,
    )
    .await;
}

async fn compile_yaml(
    app: &Arc<AppContext>,
    request: CompileYamlGrpcRequest,
) -> CompileYamlGrpcResponse {
    let yaml = crate::flows::compile_yaml(app, &request.product_id, &request.template_id)
        .await
        .unwrap_or_default();

    CompileYamlGrpcResponse { yaml }
}

async fn delete(app: &Arc<AppContext>, request: DeleteTemplateGrpcRequest) {
    crate::flows::delete_template(app, &request.product_id, &request.template_id).await;
}

async fn get_products(app: &Arc<AppContext>, _request: ()) -> GetProductsGrpcResponse {
    let products = app.templates.get_products().await;
    GetProductsGrpcResponse { products }
}
