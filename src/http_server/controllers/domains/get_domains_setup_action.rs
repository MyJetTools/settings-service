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
    let result = crate::flows::get_domains(&action.app).await;

    if result.domain_setup.is_none() {
        return HttpOutput::as_json(DomainsSetupHttpOutput::create_none()).into_ok_result(false);
    }

    let mut products = Vec::new();

    if let Some(product_sub_domains) = result.product_sub_domains {
        for itm in product_sub_domains {
            products.push(ProductDomainHttpModel {
                is_cloud_flare_proxy: itm.is_cloud_flare_proxy,
                product: itm.row_key,

                nginx: if let Some(nginx) = itm.nginx {
                    Some(NginxConfigHttpModel {
                        ca: nginx.protected_with_ca,
                        template: nginx.use_template,
                        routes: nginx
                            .rotes
                            .into_iter()
                            .map(|itm| NginxRouteHttpModel {
                                path: itm.path,
                                proxy_to: itm.proxy_to,
                                template: itm.use_template,
                            })
                            .collect(),
                    })
                } else {
                    None
                },
            });
        }
    }

    let response = DomainsSetupHttpOutput {
        result: DomainsSetup {
            domain: result.domain_setup.unwrap().domain_mask,
            products,
        }
        .into(),
    };

    HttpOutput::as_json(response).into_ok_result(false)
}
