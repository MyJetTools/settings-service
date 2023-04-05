use crate::app_ctx::AppContext;

pub async fn populate_with_secrets(
    app: &AppContext,
    content_to_populate: &str,
    min_level: u8,
) -> String {
    let template = content_to_populate.as_bytes();

    let mut result = Vec::new();

    let mut first = false;
    let mut second = false;
    let mut start = 0;

    for i in 0..template.len() {
        let b = template[i];

        if b == b'$' {
            first = true;
            continue;
        }

        if first && b == b'{' {
            second = true;
            start = i + 1;
            continue;
        }

        if first && second {
            if b == b'}' {
                let key = std::str::from_utf8(&template[start..i]).unwrap();

                if let Some(value) = app.key_value_repository.get_secret(key).await {
                    if value.level >= min_level {
                        result.extend_from_slice(value.value.as_bytes());
                    } else {
                        result.extend_from_slice(
                            format!(
                                "Secret has {} level. But required level is {}",
                                value.level, min_level
                            )
                            .as_bytes(),
                        );
                    }
                }

                first = false;
                second = false;
            }
            continue;
        }

        result.push(b);
        first = false;
        second = false;
    }
    String::from_utf8(result).unwrap()
}
