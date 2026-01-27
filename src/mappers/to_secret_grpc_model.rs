use crate::{
    app_ctx::AppContext,
    caches::SecretsSnapshot,
    models::{ProductId, SecretItem},
    secrets_grpc::SecretGrpcModel,
};

pub async fn to_secret_grpc_model(
    app: &AppContext,
    product_id: ProductId<'_>,
    secrets: &SecretsSnapshot,
    item: &SecretItem,
) -> SecretGrpcModel {
    let used_by_secrets = secrets.get_count(product_id, |itm| {
        itm.content.has_the_secret_inside(&item.id)
    });

    let used_by_templates = app
        .templates
        .get_count_from_all(|itm| itm.content.has_secret_inside())
        .await;

    SecretGrpcModel {
        name: item.id.to_string(),
        level: item.level as i32,
        created: item.created.to_rfc3339(),
        updated: item.updated.to_rfc3339(),
        used_by_secrets: used_by_secrets as i32,
        used_by_templates: used_by_templates as i32,
    }
}
