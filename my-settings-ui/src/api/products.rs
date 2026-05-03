use dioxus::prelude::*;

use crate::models::*;

#[get("/api/products/list?env_id")]
pub async fn get_list_of_products(env_id: String) -> Result<Vec<String>, ServerFnError> {
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let result = ctx.templates_grpc.get_products(()).await.unwrap();

    Ok(result.products)
}

#[get("/api/products/load_all?env_id")]
pub async fn load_all_products(env_id: String) -> Result<Vec<ProductHttpModel>, ServerFnError> {
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx.products_grpc.get_all(()).await.unwrap();

    let result = response
        .products
        .into_iter()
        .map(|item| ProductHttpModel {
            id: item.id,
            description: item.description,
            prompt: item.prompt,
            templates_amount: item.templates_amount,
            secrets_amount: item.secrets_amount,
            has_metadata: item.has_metadata,
        })
        .collect();

    Ok(result)
}

#[get("/api/products/load_one?env_id&product_id")]
pub async fn load_product(
    env_id: String,
    product_id: String,
) -> Result<ProductHttpModel, ServerFnError> {
    use crate::server::products_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .products_grpc
        .get(GetProductGrpcRequest {
            id: product_id.clone(),
        })
        .await
        .unwrap();

    Ok(ProductHttpModel {
        id: response.id,
        description: response.description,
        prompt: response.prompt,
        templates_amount: response.templates_amount,
        secrets_amount: response.secrets_amount,
        has_metadata: response.has_metadata,
    })
}

#[post("/api/products/save")]
pub async fn save_product(env_id: String, value: UpdateProductHttpModel) -> Result<(), ServerFnError> {
    use crate::server::products_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.products_grpc
        .save(SaveProductGrpcRequest {
            id: value.id,
            description: value.description,
            prompt: value.prompt,
        })
        .await
        .unwrap();

    Ok(())
}

#[post("/api/products/delete")]
pub async fn delete_product(env_id: String, product_id: String) -> Result<(), ServerFnError> {
    use crate::server::products_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.products_grpc
        .delete(DeleteProductGrpcRequest { id: product_id })
        .await
        .unwrap();

    Ok(())
}
