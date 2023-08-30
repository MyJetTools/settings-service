use std::pin::Pin;

use futures_core::Stream;

use super::server::GrpcService;
use crate::templates_grpc::templates_server::Templates;
use crate::templates_grpc::*;

#[tonic::async_trait]
impl Templates for GrpcService {
    type GetAllStream = Pin<
        Box<dyn Stream<Item = Result<TemplateListItem, tonic::Status>> + Send + Sync + 'static>,
    >;

    async fn get_all(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::GetAllStream>, tonic::Status> {
        let result = crate::operations::get_all_templates(&self.app).await;
        let time_snapshot = self.app.last_request.get_snapshot().await;

        my_grpc_extensions::grpc_server::send_vec_to_stream(result, move |item| {
            let last_time = if let Some(sub_items) = time_snapshot.get(&item.partition_key) {
                if let Some(value) = sub_items.get(&item.row_key) {
                    value.unix_microseconds / 1000
                } else {
                    0
                }
            } else {
                0
            };

            TemplateListItem {
                env: item.partition_key.clone(),
                name: item.row_key.clone(),
                created: item.create_date.clone(),
                updated: item.last_update_date.clone(),
                last_requests: last_time,
            }
        })
        .await
    }

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

    async fn delete(
        &self,
        request: tonic::Request<DeleteTemplateRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        crate::operations::templates::delete(&self.app, request.env, request.name).await;

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
