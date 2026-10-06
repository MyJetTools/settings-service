use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::{
    app_ctx::AppContext,
    flows::{MoveSecretError, MoveSecretImpact},
    models::ProductId,
};

#[http_route(
    method: "POST",
    route: "/api/v1/secrets/move",
    description: "Move a secret between a product scope and the shared scope. Unless forced, a move that would break references is not performed - the response lists them with moved=false",
    summary: "Move secret",
    controller: "V1::Secrets",
    input_data: MoveSecretInput,
    result: [
        {status_code: 200, description: "Ok response", model: "MoveSecretHttpModel"},
        {status_code: 400, description: "Source and target scope are the same"},
        {status_code: 404, description: "Secret not found in the source scope"},
        {status_code: 409, description: "Target scope already has a secret with this id"},
    ]
)]
pub struct MoveSecretAction {
    app: Arc<AppContext>,
}

impl MoveSecretAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &MoveSecretAction,
    input_data: MoveSecretInput,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let from: ProductId = input_data.from_product_id.as_deref().into();
    let to: ProductId = input_data.to_product_id.as_deref().into();

    let result = crate::flows::try_move_secret(
        &action.app,
        &input_data.secret_id,
        from,
        to,
        input_data.force,
    )
    .await;

    let model = match result {
        Ok(impact) => to_http_model(true, impact),
        Err(MoveSecretError::BreaksReferences(impact)) => to_http_model(false, impact),
        Err(MoveSecretError::SameScope) => {
            return Err(HttpFailResult::as_validation_error(
                "Source and target scope are the same",
            ));
        }
        Err(MoveSecretError::NotFound) => {
            return Err(HttpFailResult::as_not_found(
                "Secret not found".to_string(),
                false,
            ));
        }
        Err(MoveSecretError::AlreadyExists) => {
            return Err(HttpFailResult::from((
                409,
                "Target scope already has a secret with this id",
            )));
        }
    };

    HttpOutput::as_json(model).into_ok_result(false)
}

fn to_http_model(moved: bool, impact: MoveSecretImpact) -> MoveSecretHttpModel {
    MoveSecretHttpModel {
        moved,
        broken_dependencies: impact.broken_dependencies,
        broken_consumers: impact
            .broken_consumers
            .into_iter()
            .map(|consumer| MoveSecretBrokenConsumerHttpModel {
                product_id: consumer.product_id,
                kind: consumer.kind.as_str().to_string(),
                id: consumer.id,
            })
            .collect(),
    }
}
