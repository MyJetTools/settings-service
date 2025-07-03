use crate::{app_ctx::AppContext, my_no_sql::*};

pub async fn set_domain_mask(app: &AppContext, domain_mask: &str) {
    let entity = DomainMyNoSqlEntity::DomainSetup(DomainSetupMyNoSqlEntity {
        time_stamp: Default::default(),
        domain_mask: domain_mask.to_lowercase(),
    });

    app.domains_setup
        .insert_or_replace_entity(&entity)
        .await
        .unwrap();
}
