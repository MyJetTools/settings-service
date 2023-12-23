use my_no_sql_sdk::macros::*;
use serde::*;

#[enum_of_my_no_sql_entity(table_name: "settings-domains", generate_unwraps = true)]
pub enum DomainMyNoSqlEntity {
    DomainSetup(DomainSetupMyNoSqlEntity),
    ProductSubDomain(ProductSubDomainMyNoSqlEntity),
}

#[enum_model(partition_key = "domain", row_key = "setup")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DomainSetupMyNoSqlEntity {
    pub domain_mask: String,
}

#[enum_model(partition_key = "sub-domain")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductSubDomainMyNoSqlEntity {
    pub is_cloud_flare_proxy: bool,
    pub internal_domain_name: String,
}
