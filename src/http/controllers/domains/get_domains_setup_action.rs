use std::sync::Arc;

use my_http_server::{macros::http_route, HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

use super::contracts::*;
use crate::app_ctx::AppContext;

#[http_route(
    method: "GET",
    route: "/api/domain",
    description: "Get domains schema",
    summary: "Returns domains schema",
    controller: "Domains",

    result:[
        {status_code: 200, description: "Ok response", model: "DomainsSetupHttpOutput"},
    ]
)]
pub struct GetDomainsAction {
    app: Arc<AppContext>,
}

impl GetDomainsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetDomainsAction,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::operations::get_domains(&action.app).await;

    if result.domain_setup.is_none() {
        return HttpOutput::as_json(DomainsSetupHttpOutput::create_none()).into_ok_result(false);
    }

    let mut products = Vec::new();

    if let Some(product_sub_domains) = result.product_sub_domains {
        for itm in product_sub_domains {
            products.push(ProductDomainHttpModel {
                product: itm.get_sub_domain().to_string(),
                is_cloud_flare_proxy: itm.is_cloud_flare_proxy,
                internal_domain_name: itm.internal_domain_name,
            });
        }
    }

    let response = DomainsSetupHttpOutput {
        result: DomainsSetup {
            domain: result.domain_setup.unwrap().domain,
            products,
        }
        .into(),
    };

    HttpOutput::as_json(response).into_ok_result(false)
}
