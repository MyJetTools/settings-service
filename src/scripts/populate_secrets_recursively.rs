use crate::app_ctx::AppContext;

use crate::models::*;

use rust_common::placeholders::*;

pub async fn populate_secrets_recursively(
    app: &AppContext,
    src_secret_value: SecretValue,
) -> String {
    let mut result = String::new();

    for token in PlaceholdersIterator::new(
        &src_secret_value.content,
        super::PLACEHOLDER_OPEN,
        super::PLACEHOLDER_CLOSE,
    ) {
        match token {
            ContentToken::Text(text) => result.push_str(text),
            ContentToken::Placeholder(secret_name) => {
                if secret_name.starts_with('$') {
                    result.push_str("${");
                    result.push_str(&secret_name[1..]);
                    result.push('}');
                } else {
                    let secret_value = crate::scripts::secrets::get_value(app, secret_name).await;

                    if let Some(secret_value) = secret_value {
                        if secret_value.level > src_secret_value.level {
                            if super::has_secrets_to_populate(&secret_value.content) {
                                recompile_token(secret_value, &mut result);
                            } else {
                                result.push_str(&secret_value.content);
                            }
                        } else {
                            super::fill_token_has_wrong_level(
                                secret_name,
                                secret_value,
                                src_secret_value.level,
                                &mut result,
                            );
                        }
                    } else {
                        super::populate_secret_not_found(&mut result, secret_name);
                    }
                }
            }
        }
    }

    while super::has_secrets_to_populate(&result) {
        result = populate_with_secrets(app, &result).await;
    }

    result
}

async fn populate_with_secrets(app: &AppContext, content_to_populate: &str) -> String {
    let mut result = String::new();

    for template_token in PlaceholdersIterator::new(
        content_to_populate,
        super::PLACEHOLDER_OPEN,
        super::PLACEHOLDER_CLOSE,
    ) {
        match template_token {
            ContentToken::Text(text) => {
                result.push_str(text);
            }
            ContentToken::Placeholder(secret_name) => {
                if secret_name.starts_with('$') {
                    result.push_str("${");
                    result.push_str(&secret_name[1..]);
                    result.push('}');
                } else {
                    let (secret_name, secret_min_level) = super::parse_secret_line(secret_name);

                    match crate::scripts::secrets::get_value(app, secret_name).await {
                        Some(secret_value) => {
                            if let Some(secret_min_level) = secret_min_level {
                                if secret_value.level > secret_min_level {
                                    if super::has_secrets_to_populate(&secret_value.content) {
                                        recompile_token(secret_value, &mut result);
                                    } else {
                                        result.push_str(secret_value.content.as_str());
                                    }
                                } else {
                                    super::fill_token_has_wrong_level(
                                        secret_name,
                                        secret_value,
                                        secret_min_level,
                                        &mut result,
                                    );
                                }
                            } else {
                                if super::has_secrets_to_populate(&secret_value.content) {
                                    recompile_token(secret_value, &mut result);
                                } else {
                                    result.push_str(secret_value.content.as_str());
                                }
                            }
                        }
                        None => {
                            result.push_str(&format!("/*Secret {} not found*/", secret_name));
                        }
                    }
                }
            }
        }
    }

    result
}

fn recompile_token(secret_value: SecretValue, result: &mut String) {
    for secret_token in PlaceholdersIterator::new(
        secret_value.content.as_str(),
        super::PLACEHOLDER_OPEN,
        super::PLACEHOLDER_CLOSE,
    ) {
        match secret_token {
            ContentToken::Text(text) => {
                result.push_str(text);
            }
            ContentToken::Placeholder(secret_name) => {
                if secret_name.starts_with('$') {
                    result.push_str("${");
                    result.push_str(&secret_name[1..]);
                    result.push('}');
                } else {
                    result.push_str("${");
                    result.push_str(secret_name);
                    result.push_str(":");
                    result.push_str(secret_value.level.to_string().as_str());
                    result.push_str("}");
                }
            }
        }
    }
}
