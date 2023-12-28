use super::server::GrpcService;
use crate::domains_grpc::domains_server::Domains;
use crate::domains_grpc::*;

#[tonic::async_trait]
impl Domains for GrpcService {
    async fn get_domains_info(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<DomainsInfoGrpcResponse>, tonic::Status> {
        let result = crate::operations::get_domains(&self.app).await;

        let response = DomainsInfoGrpcResponse {
            domain_mask: if let Some(domain_setup) = result.domain_setup {
                Some(domain_setup.domain_mask)
            } else {
                None
            },
            products: if let Some(sub_domains) = result.product_sub_domains {
                sub_domains
                    .into_iter()
                    .map(|itm| DomainProductGrpcInfo {
                        product_name: itm.row_key,
                        is_cloud_flare_proxy: itm.is_cloud_flare_proxy,
                        nginx_config: if let Some(nginx) = itm.nginx {
                            Some(nginx.into())
                        } else {
                            None
                        },
                    })
                    .collect()
            } else {
                vec![]
            },
        };

        Ok(tonic::Response::new(response))
    }

    async fn set_domain_mask(
        &self,
        request: tonic::Request<SetDomainMaskRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        crate::operations::set_domain_mask(&self.app, &request.domain_mask).await;

        Ok(tonic::Response::new(()))
    }

    async fn set_product_info(
        &self,
        request: tonic::Request<DomainProductGrpcInfo>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();
        crate::operations::set_domain_product_info(
            &self.app,
            request.product_name,
            request.is_cloud_flare_proxy,
            request.nginx_config,
        )
        .await;

        Ok(tonic::Response::new(()))
    }

    async fn delete_product_info(
        &self,
        request: tonic::Request<DeleteDomainProductInfoRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let request = request.into_inner();

        crate::operations::delete_domain_product_info(&self.app, &request.product_name).await;

        Ok(tonic::Response::new(()))
    }

    async fn ping(&self, _: tonic::Request<()>) -> Result<tonic::Response<()>, tonic::Status> {
        Ok(tonic::Response::new(()))
    }
}
