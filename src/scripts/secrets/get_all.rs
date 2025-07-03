use std::{collections::BTreeMap, sync::Arc};

use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

pub async fn get_all(app: &AppContext) -> Option<Vec<Arc<SecretMyNoSqlEntity>>> {
    app.secrets_storage_reader
        .get_by_partition_key_as_vec(SecretMyNoSqlEntity::generate_partition_key())
        .await
}

pub async fn get_all_as_hash_map(
    app: &AppContext,
) -> Option<BTreeMap<String, Arc<SecretMyNoSqlEntity>>> {
    app.secrets_storage_reader
        .get_by_partition_key(SecretMyNoSqlEntity::generate_partition_key())
        .await
}
