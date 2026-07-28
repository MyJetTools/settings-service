use std::sync::Arc;

use my_grpc_extensions::server::*;

use crate::app_ctx::AppContext;
use crate::products_grpc::*;

generate_server!(
    proto_file: "./proto/ProductsService.proto",
    crate_ns: "crate::products_grpc",
);

async fn save(app: &Arc<AppContext>, request: SaveProductGrpcRequest) {
    crate::flows::save_product(app, request.id, request.description, request.prompt).await;
}

async fn get(app: &Arc<AppContext>, request: GetProductGrpcRequest) -> ProductGrpcModel {
    let snapshot = app.products.get_snapshot().await;
    if let Some(product) = snapshot.get(request.id.as_str()) {
        ProductGrpcModel {
            id: product.id.clone(),
            description: Some(product.description.clone()),
            prompt: Some(product.prompt.clone()),
            templates_amount: 0,
            secrets_amount: 0,
            has_metadata: true,
            created: product.created.unix_microseconds,
            updated: product.updated.unix_microseconds,
        }
    } else {
        ProductGrpcModel {
            id: request.id,
            description: None,
            prompt: None,
            templates_amount: 0,
            secrets_amount: 0,
            has_metadata: false,
            created: 0,
            updated: 0,
        }
    }
}

async fn get_all(app: &Arc<AppContext>, _request: ()) -> GetAllProductsGrpcResponse {
    let products = crate::flows::get_all_products(app).await;
    let snapshot = app.products.get_snapshot().await;

    let products = products
        .into_iter()
        .map(|item| {
            let timestamps = snapshot
                .get(item.id.as_str())
                .map(|p| (p.created.unix_microseconds, p.updated.unix_microseconds))
                .unwrap_or((0, 0));
            ProductGrpcModel {
                id: item.id,
                description: item.description,
                prompt: item.prompt,
                templates_amount: item.templates_count,
                secrets_amount: item.secrets_count,
                has_metadata: item.has_metadata,
                created: timestamps.0,
                updated: timestamps.1,
            }
        })
        .collect();

    GetAllProductsGrpcResponse { products }
}

async fn delete(app: &Arc<AppContext>, request: DeleteProductGrpcRequest) {
    crate::flows::delete_product(app, &request.id).await;
}
