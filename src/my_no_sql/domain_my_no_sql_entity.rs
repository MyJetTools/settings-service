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
    pub nginx: Option<NginxSetupMyNoSqlEntity>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NginxSetupMyNoSqlEntity {
    pub protected_with_ca: Option<String>,
    pub use_template: Option<String>,
    pub rotes: Vec<NginxRouteMyNoSqlEntity>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NginxRouteMyNoSqlEntity {
    pub path: String,
    pub proxy_to: String,
    pub use_template: Option<String>,
}
