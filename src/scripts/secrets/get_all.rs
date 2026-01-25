use std::{collections::BTreeMap, sync::Arc};

use crate::{app_ctx::AppContext, my_no_sql::SecretMyNoSqlEntity};

pub async fn get_all(app: &AppContext, env: Option<&str>) -> Option<Vec<Arc<SecretMyNoSqlEntity>>> {
    let partition_key = env.unwrap_or(SecretMyNoSqlEntity::DEFAULT_PARTITION_KEY);
    app.secrets_storage_reader
        .get_by_partition_key_as_vec(partition_key)
        .await
}

pub async fn get_all_as_hash_map(
    app: &AppContext,
    env: Option<&str>,
) -> Option<BTreeMap<String, Arc<SecretMyNoSqlEntity>>> {
    let partition_key = env.unwrap_or(SecretMyNoSqlEntity::DEFAULT_PARTITION_KEY);
    app.secrets_storage_reader
        .get_by_partition_key(partition_key)
        .await
}
