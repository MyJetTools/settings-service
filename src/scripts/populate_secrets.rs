use crate::app_ctx::AppContext;

use crate::caches::SecretsSnapshot;
use crate::models::*;

use rust_common::placeholders::*;

pub fn populate_secrets(
    app: &AppContext,
    product_id: ProductId<'_>,
    content: &Content,
    secrets_snapshot: &SecretsSnapshot,
    min_secret_level: u8,
) -> Content {
    let mut result = String::new();

    for token in PlaceholdersIterator::new(
        content.as_str(),
        crate::consts::PLACEHOLDER_OPEN,
        crate::consts::PLACEHOLDER_CLOSE,
    ) {
        match token {
            ContentToken::Text(text) => result.push_str(text),
            ContentToken::Placeholder(secret_id) => {
                if secret_id.starts_with('$') {
                    result.push_str("${");
                    result.push_str(&secret_id[1..]);
                    result.push('}');
                } else {
                    let secret = secrets_snapshot.consume_secret(product_id, secret_id);

                    if let Some(secret) = secret {
                        if secret.level >= min_secret_level {
                            if secret.content.has_secret_inside() {
                                let content = populate_secrets(
                                    app,
                                    product_id,
                                    &secret.content,
                                    &secrets_snapshot,
                                    secret.level,
                                );

                                result.push_str(content.as_str());
                            } else {
                                result.push_str(secret.content.as_str());
                            }
                        } else {
                            fill_token_has_wrong_level(&secret, secret.level, &mut result);
                        }
                    } else {
                        populate_secret_not_found(&mut result, secret_id);
                    }
                }
            }
        }
    }

    let mut result: Content = result.into();

    while result.has_secret_inside() {
        result = populate_secrets(app, product_id, content, secrets_snapshot, min_secret_level);
    }

    result
}

pub fn populate_secret_not_found(result: &mut String, secret_name: &str) {
    result.push_str("/*Secret ");
    result.push_str(secret_name);
    result.push_str(" is not found*/");
}

pub fn fill_token_has_wrong_level(secret: &SecretItem, secret_min_level: u8, result: &mut String) {
    result.push_str(&format!(
        "/*Secret {} has lower level {} than required {}*/",
        secret.id, secret.level, secret_min_level
    ));
}
