use super::server::GrpcService;
use crate::secrets_grpc::secrets_server::Secrets;
use crate::secrets_grpc::*;

use my_grpc_extensions::server::*;

#[tonic::async_trait]
impl Secrets for GrpcService {
    generate_server_stream!(stream_name:"GetAllStream", item_name:"SecretGrpcModel");

    async fn get_all(
        &self,
        request: tonic::Request<GetAllSecretsGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetAllStream>, tonic::Status> {
        let request = request.into_inner();

        let result = crate::flows::get_all_secrets(
            &self.app,
            request.product_id.as_str(),
            request.include_shared,
        )
        .await;

        let result =
            my_grpc_extensions::grpc_server_streams::send_from_iterator(result.into_iter()).await;

        result
    }

    async fn get(
        &self,
        request: tonic::Request<GetSecretGrpcRequest>,
    ) -> Result<tonic::Response<SecretValueGrpcModel>, tonic::Status> {
        let request = request.into_inner();

        let response = crate::flows::get_secret(
            &self.app,
            request.product_id.as_deref().into(),
            &request.secret_id,
        )
        .await;

        Ok(response.into())
    }

    async fn save(
        &self,
        request: tonic::Request<SaveSecretGrpcRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        crate::flows::save_secret(
            &self.app,
            request.product_id.as_deref().into(),
            request.id,
            request.value,
            request.level as u8,
        )
        .await;

        Ok(().into())
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteSecretGrpcRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        crate::flows::delete_secret(
            &self.app,
            request.product_id.as_deref().into(),
            &request.secret_id,
        )
        .await;
        //crate::scripts::secrets::delete(&self.app, request.env.as_deref(), &request.name).await;
        Ok(tonic::Response::new(()))
    }

    async fn get_templates_usage(
        &self,
        request: tonic::Request<GetTemplatesUsageGrpcRequest>,
    ) -> Result<tonic::Response<GetTemplatesUsageGrpcResponse>, tonic::Status> {
        let request = request.into_inner();

        let templates = crate::flows::get_templates_used_by_the_secret(
            &self.app,
            request.product_id.as_deref().into(),
            &request.secret_id,
        )
        .await;

        Ok(tonic::Response::new(GetTemplatesUsageGrpcResponse {
            templates,
        }))
    }

    async fn get_secrets_usage(
        &self,
        request: tonic::Request<DeleteSecretGrpcRequest>,
    ) -> Result<tonic::Response<GetSecretsUsageGrpcResponse>, tonic::Status> {
        let request = request.into_inner();

        let secrets = crate::flows::get_secrets_used_by_the_secret(
            &self.app,
            request.product_id.as_deref().into(),
            &request.secret_id,
        )
        .await;

        Ok(tonic::Response::new(GetSecretsUsageGrpcResponse {
            secrets,
        }))
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
