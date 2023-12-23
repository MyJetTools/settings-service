use crate::{
    app_ctx::AppContext,
    my_no_sql::{DomainSetupMyNoSqlEntity, ProductSubDomainMyNoSqlEntity},
};

pub struct DomainsResult {
    pub domain_setup: Option<DomainSetupMyNoSqlEntity>,
    pub product_sub_domains: Option<Vec<ProductSubDomainMyNoSqlEntity>>,
}

pub async fn get_domains(app: &AppContext) -> DomainsResult {
    let domain_setup: Option<DomainSetupMyNoSqlEntity> =
        app.domains_setup.get_enum_case(None).await.unwrap();

    if domain_setup.is_none() {
        return DomainsResult {
            domain_setup: None,
            product_sub_domains: None,
        };
    }

    let product_sub_domains: Option<Vec<ProductSubDomainMyNoSqlEntity>> = app
        .domains_setup
        .get_enum_cases_by_partition_key(None)
        .await
        .unwrap();

    DomainsResult {
        domain_setup,
        product_sub_domains,
    }
}
