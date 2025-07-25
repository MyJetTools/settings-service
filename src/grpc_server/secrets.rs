use super::server::GrpcService;
use crate::models::*;
use crate::secrets_grpc::secrets_server::Secrets;
use crate::secrets_grpc::*;

use my_grpc_extensions::server::*;

#[tonic::async_trait]
impl Secrets for GrpcService {
    generate_server_stream!(stream_name:"GetAllStream", item_name:"SecretListItem");

    async fn get_all(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetAllStream>, tonic::Status> {
        let result = if let Some(items) = crate::scripts::secrets::get_all(&self.app).await {
            let mut result = Vec::with_capacity(items.len());
            for item in items {
                let templates_amount = crate::scripts::secrets::get_secret_usage_by_templates(
                    &self.app,
                    item.get_secret_name(),
                )
                .await;

                let secrets_amount = crate::scripts::secrets::get_secret_usage_by_secrets(
                    &self.app,
                    item.get_secret_name(),
                )
                .await;

                result.push(SecretListItem {
                    name: item.row_key.clone(),
                    level: item.level.unwrap_or(0) as i32,
                    created: item.create_date.clone(),
                    updated: item.last_update_date.clone(),
                    used_by_secrets: secrets_amount.len() as i32,
                    used_by_templates: templates_amount.len() as i32,
                });
            }

            result
        } else {
            vec![]
        };

        let result =
            my_grpc_extensions::grpc_server::send_vec_to_stream(result.into_iter(), |item| item)
                .await;

        result
    }

    async fn get(
        &self,
        request: tonic::Request<GetSecretRequest>,
    ) -> Result<tonic::Response<SecretModel>, tonic::Status> {
        let request = request.into_inner();

        let result = crate::scripts::secrets::get_value(&self.app, &request.name).await;

        let result = match result {
            Some(value) => SecretModel {
                name: request.name,
                value: value.content,
                level: value.level as i32,
            },
            None => SecretModel {
                name: "".to_string(),
                value: "".to_string(),
                level: 0,
            },
        };

        Ok(tonic::Response::new(result))
    }

    async fn save(
        &self,
        request: tonic::Request<SaveSecretRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        let model = request.model.unwrap();

        crate::scripts::secrets::update(
            &self.app,
            model.name,
            SecretValue {
                content: model.value,
                level: model.level as u8,
            },
        )
        .await;

        Ok(tonic::Response::new(()))
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteSecretRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        crate::scripts::secrets::delete(&self.app, &request.name).await;
        Ok(tonic::Response::new(()))
    }

    async fn get_templates_usage(
        &self,
        request: tonic::Request<GetTemplatesUsageRequest>,
    ) -> Result<tonic::Response<GetTemplatesUsageResponse>, tonic::Status> {
        let request = request.into_inner();

        let result =
            crate::scripts::secrets::get_secret_usage_by_templates(&self.app, &request.name).await;

        let templates = result
            .into_iter()
            .map(|x| TemplateUsageModel {
                env: x.env,
                name: x.name,
                yaml: x.yaml,
            })
            .collect();

        Ok(tonic::Response::new(GetTemplatesUsageResponse {
            templates,
        }))
    }

    async fn get_secrets_usage(
        &self,
        request: tonic::Request<GetSecretsUsageRequest>,
    ) -> Result<tonic::Response<GetSecretsUsageResponse>, tonic::Status> {
        let request = request.into_inner();

        let result =
            crate::scripts::secrets::get_secret_usage_by_secrets(&self.app, &request.name).await;

        let secrets = result
            .into_iter()
            .map(|x| SecretUsageModel {
                name: x.name,
                value: x.value,
            })
            .collect();

        Ok(tonic::Response::new(GetSecretsUsageResponse { secrets }))
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
