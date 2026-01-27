use super::server::GrpcService;
use crate::domains_grpc::domains_server::Domains;
use crate::domains_grpc::*;

#[tonic::async_trait]
impl Domains for GrpcService {
    async fn get_domains_info(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<DomainsInfoGrpcResponse>, tonic::Status> {
        let _result = crate::flows::get_domains(&self.app).await;

        let response = DomainsInfoGrpcResponse::default();
        Ok(tonic::Response::new(response))
    }

    async fn set_domain_mask(
        &self,
        request: tonic::Request<SetDomainMaskRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        crate::flows::set_domain_mask(&self.app, &request.domain_mask).await;

        Ok(tonic::Response::new(()))
    }

    async fn set_product_info(
        &self,
        request: tonic::Request<DomainProductGrpcInfo>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        panic!("Not Implemented");

        Ok(tonic::Response::new(()))
    }

    async fn delete_product_info(
        &self,
        request: tonic::Request<DeleteDomainProductInfoRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        panic!("Not Implemented");
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
