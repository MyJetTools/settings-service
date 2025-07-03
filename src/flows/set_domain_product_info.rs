use crate::{app_ctx::AppContext, domains_grpc::NginxConfigGrpcModel, my_no_sql::*};

pub async fn set_domain_product_info(
    app: &AppContext,
    product_domain: String,
    cloud_flare_proxy_pass: bool,
    nginx_config: Option<NginxConfigGrpcModel>,
) {
    let entity = DomainMyNoSqlEntity::ProductSubDomain(ProductSubDomainMyNoSqlEntity {
        row_key: product_domain.to_lowercase(),
        time_stamp: Default::default(),
        is_cloud_flare_proxy: cloud_flare_proxy_pass,

        nginx: if let Some(nginx_config) = nginx_config {
            NginxSetupMyNoSqlEntity {
                protected_with_ca: nginx_config.protected_with_ca,
                use_template: nginx_config.template,
                rotes: nginx_config
                    .routes
                    .into_iter()
                    .map(|itm| NginxRouteMyNoSqlEntity {
                        path: itm.path,
                        proxy_to: itm.proxy_to,
                        use_template: itm.template,
                    })
                    .collect(),
            }
            .into()
        } else {
            None
        },
    });

    app.domains_setup
        .insert_or_replace_entity(&entity)
        .await
        .unwrap();
}
