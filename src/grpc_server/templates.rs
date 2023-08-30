use super::server::GrpcService;
use crate::templates_grpc::templates_server::Templates;
use crate::templates_grpc::*;

#[tonic::async_trait]
impl Templates for GrpcService {
    async fn get(
        &self,
        request: tonic::Request<GetTemplateRequest>,
    ) -> Result<tonic::Response<GetTemplateResponse>, tonic::Status> {
        let request = request.into_inner();

        let template =
            crate::operations::templates::get(&self.app, &request.env, &request.name).await;

        let result = if let Some(template) = template {
            GetTemplateResponse {
                yaml: template.yaml_template.clone(),
            }
        } else {
            GetTemplateResponse {
                yaml: "".to_string(),
            }
        };

        Ok(tonic::Response::new(result))
    }

    async fn save(
        &self,
        request: tonic::Request<SaveTemplateRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        crate::operations::templates::post(&self.app, request.env, request.name, request.yaml)
            .await;

        Ok(tonic::Response::new(()))
    }

    async fn compile_yaml(
        &self,
        request: tonic::Request<CompileYamlRequest>,
    ) -> Result<tonic::Response<CompileYamlResponse>, tonic::Status> {
        let request = request.into_inner();

        let yaml = crate::operations::templates::get_populated_template(
            &self.app,
            &request.env,
            &request.name,
        )
        .await;

        let result = if let Some(yaml) = yaml {
            CompileYamlResponse { yaml }
        } else {
            CompileYamlResponse {
                yaml: "".to_string(),
            }
        };

        Ok(tonic::Response::new(result))
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
