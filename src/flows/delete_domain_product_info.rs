use crate::{app_ctx::AppContext, my_no_sql::ProductSubDomainMyNoSqlEntity};

pub async fn delete_domain_product_info(app: &AppContext, domain_product: &str) {
    let domain_product = domain_product.to_lowercase();
    app.domains_setup
        .delete_enum_case_with_row_key::<ProductSubDomainMyNoSqlEntity>(domain_product.as_str())
        .await
        .unwrap();
}
