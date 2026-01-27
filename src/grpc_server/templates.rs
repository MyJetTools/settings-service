use std::pin::Pin;

use futures_core::Stream;

use super::server::GrpcService;
use crate::templates_grpc::templates_server::Templates;
use crate::templates_grpc::*;

#[tonic::async_trait]
impl Templates for GrpcService {
    type GetAllStream = Pin<
        Box<
            dyn Stream<Item = Result<TemplateListItemGrpcModel, tonic::Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;

    async fn get_all(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetAllStream>, tonic::Status> {
        let result = crate::flows::get_all_templates(&self.app).await;

        my_grpc_extensions::grpc_server_streams::send_from_iterator(result.into_iter()).await
    }

    async fn get_server_info(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<ServerInfoResponse>, tonic::Status> {
        let result = ServerInfoResponse {
            env_name: self.app.settings.env.to_string(),
        };

        Ok(tonic::Response::new(result))
    }

    async fn get_template_content(
        &self,
        request: tonic::Request<GetTemplateContentGrpcRequest>,
    ) -> Result<tonic::Response<GetTemplateContentGrpcResponse>, tonic::Status> {
        let request = request.into_inner();

        let content = self
            .app
            .templates
            .get_by_id(&request.product_id, &request.template_id, |itm| {
                itm.content.to_string()
            })
            .await
            .unwrap_or_default();

        let response = GetTemplateContentGrpcResponse { content };

        Ok(response.into())
    }

    async fn save(
        &self,
        request: tonic::Request<SaveTemplateGrpcRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        crate::flows::save_template(
            &self.app,
            &request.product_id,
            request.template_id,
            request.yaml,
        )
        .await;

        Ok(tonic::Response::new(()))
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteTemplateGrpcRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        crate::flows::delete_template(&self.app, &request.product_id, &request.template_id).await;

        Ok(tonic::Response::new(()))
    }

    async fn compile_yaml(
        &self,
        request: tonic::Request<CompileYamlRequest>,
    ) -> Result<tonic::Response<CompileYamlResponse>, tonic::Status> {
        let request = request.into_inner();

        let yaml = crate::flows::compile_yaml(&self.app, &request.env, &request.name)
            .await
            .unwrap_or_default();

        let result = CompileYamlResponse { yaml };

        Ok(result.into())
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
