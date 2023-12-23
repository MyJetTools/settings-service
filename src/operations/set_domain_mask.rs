use my_no_sql_sdk::macros::*;

use crate::{app_ctx::AppContext, my_no_sql::*};

pub async fn set_domain_mask(app: &AppContext, domain_mask: &str) {
    let entity = DomainMyNoSqlEntity::DomainSetup(DomainSetupMyNoSqlEntity {
        time_stamp: time_stamp_init!(),
        domain_mask: domain_mask.to_lowercase(),
    });

    app.domains_setup
        .insert_or_replace_entity(&entity)
        .await
        .unwrap();
}
