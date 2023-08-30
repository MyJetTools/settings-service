use super::server::GrpcService;
use crate::app_ctx::SecretsValueReader;
use crate::secrets_grpc::secrets_server::Secrets;
use crate::secrets_grpc::*;

#[tonic::async_trait]
impl Secrets for GrpcService {
    async fn get(
        &self,
        request: tonic::Request<GetSecretRequest>,
    ) -> Result<tonic::Response<SecretModel>, tonic::Status> {
        let request = request.into_inner();

        let result = self.app.get_secret_value(&request.name).await;

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

    async fn get_templates_usage(
        &self,
        request: tonic::Request<GetTemplatesUsageRequest>,
    ) -> Result<tonic::Response<GetTemplatesUsageResponse>, tonic::Status> {
        let request = request.into_inner();

        let result =
            crate::operations::get_secret_usage_in_templates(&self.app, &request.name).await;

        let templates = result
            .into_iter()
            .map(|x| TemplateModel {
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

        let result = crate::operations::get_secret_usage_in_secrets(&self.app, &request.name).await;

        let secrets = result
            .into_iter()
            .map(|x| SecretModel {
                name: x.name,
                value: x.value,
                level: 0,
            })
            .collect();

        Ok(tonic::Response::new(GetSecretsUsageResponse { secrets }))
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
