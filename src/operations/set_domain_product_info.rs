use my_no_sql_sdk::macros::time_stamp_init;

use crate::{app_ctx::AppContext, my_no_sql::*};

pub async fn set_domain_product_info(
    app: &AppContext,
    product_domain: String,
    cloud_flare_proxy_pass: bool,
    internal_domain: String,
) {
    let entity = DomainMyNoSqlEntity::ProductSubDomain(ProductSubDomainMyNoSqlEntity {
        row_key: product_domain.to_lowercase(),
        time_stamp: time_stamp_init!(),
        is_cloud_flare_proxy: cloud_flare_proxy_pass,
        internal_domain_name: internal_domain,
    });

    app.domains_setup
        .insert_or_replace_entity(&entity)
        .await
        .unwrap();
}
