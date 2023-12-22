use crate::{
    app_ctx::AppContext,
    my_no_sql::{DomainSetup, ProductSubDomain},
};

pub struct DomainsResult {
    pub domain_setup: Option<DomainSetup>,
    pub product_sub_domains: Option<Vec<ProductSubDomain>>,
}

pub async fn get_domains(app: &AppContext) -> DomainsResult {
    let result = app
        .domains_setup
        .get_entity(
            DomainSetup::PARTITION_KEY,
            DomainSetup::ROW_KEY.unwrap(),
            None,
        )
        .await
        .unwrap();

    if result.is_none() {
        return DomainsResult {
            domain_setup: None,
            product_sub_domains: None,
        };
    }

    let product_domains = app
        .domains_setup
        .get_by_partition_key(ProductSubDomain::PARTITION_KEY, None)
        .await
        .unwrap();

    DomainsResult {
        domain_setup: Some(result.unwrap().unwrap_domain_setup().clone()),
        product_sub_domains: if let Some(product_domains) = product_domains {
            let mut result = Vec::new();

            for itm in product_domains {
                result.push(itm.unwrap_product_sub_domain().clone());
            }

            Some(result)
        } else {
            None
        },
    }
}
