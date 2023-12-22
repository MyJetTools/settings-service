use my_no_sql_sdk::{abstractions::MyNoSqlEntity, macros::*};
use serde::*;

#[enum_of_my_no_sql_entity(table_name: "settings-domains", generate_unwraps = true)]
pub enum DomainMyNoSqlEntity {
    DomainSetup(DomainSetup),
    ProductSubDomain(ProductSubDomain),
}

#[enum_model(partition_key = "domain", row_key = "setup")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DomainSetup {
    pub domain: String,
}

#[enum_model(partition_key = "sub-domain")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductSubDomain {
    pub is_cloud_flare_proxy: bool,
    pub internal_domain_name: String,
}

impl ProductSubDomain {
    pub fn get_sub_domain(&self) -> &str {
        self.get_row_key()
    }
}
