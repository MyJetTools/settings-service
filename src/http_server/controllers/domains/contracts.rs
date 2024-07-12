use my_http_server::macros::{MyHttpInput, MyHttpObjectStructure};
use serde_derive::*;

#[derive(MyHttpObjectStructure, Serialize)]
pub struct DomainsSetupHttpOutput {
    pub result: Option<DomainsSetup>,
}

impl DomainsSetupHttpOutput {
    pub fn create_none() -> Self {
        Self { result: None }
    }
}
#[derive(MyHttpObjectStructure, Serialize)]
pub struct ProductDomainHttpModel {
    pub product: String,
    pub is_cloud_flare_proxy: bool,
    pub nginx: Option<NginxConfigHttpModel>,
}

#[derive(MyHttpObjectStructure, Serialize)]
pub struct NginxConfigHttpModel {
    pub ca: Option<String>,
    pub template: Option<String>,
    pub routes: Vec<NginxRouteHttpModel>,
}

#[derive(MyHttpObjectStructure, Serialize)]
pub struct NginxRouteHttpModel {
    pub path: String,
    pub proxy_to: String,
    pub template: Option<String>,
}

#[derive(MyHttpObjectStructure, Serialize)]
pub struct DomainsSetup {
    pub domain: String,
    pub products: Vec<ProductDomainHttpModel>,
}

#[derive(MyHttpInput)]
pub struct SetupDomainHttpRequest {
    #[http_body(
        name = "domainMask",
        description = "Domain mask. Format is: *-env_name.domain.com"
    )]
    pub domain_mask: String,
}
