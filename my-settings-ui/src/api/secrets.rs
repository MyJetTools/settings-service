use crate::models::*;

use super::base_url;

pub async fn load_secrets(
    _env_id: String,
    product_id: String,
) -> Result<Vec<SecretHttpModel>, String> {
    let url = format!(
        "{}/api/v1/secrets?product_id={}&include_shared=true",
        base_url(),
        urlencode(&product_id),
    );
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn save_secret(
    _env_id: String,
    value: UpdateSecretValueHttpModel,
) -> Result<(), String> {
    let url = format!("{}/api/v1/secrets", base_url());
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&value)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("POST {url} → {}", resp.status()));
    }
    Ok(())
}

#[derive(serde::Serialize)]
struct DeleteSecretPayload<'a> {
    product_id: Option<&'a str>,
    secret_id: &'a str,
}

pub async fn delete_secret(
    _env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<(), String> {
    let url = format!("{}/api/v1/secrets/delete", base_url());
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&DeleteSecretPayload {
            product_id: product_id.as_deref(),
            secret_id: &secret_id,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("POST {url} → {}", resp.status()));
    }
    Ok(())
}

pub async fn load_secret(
    _env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<SecretApiModel, String> {
    let url = format!(
        "{}/api/v1/secrets/get?product_id={}&secret_id={}",
        base_url(),
        urlencode(product_id.as_deref().unwrap_or("")),
        urlencode(&secret_id),
    );
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    let value: SecretValueApiModel = resp.json().await.map_err(|e| e.to_string())?;
    Ok(SecretApiModel {
        secret_id,
        value: value.value,
        level: value.level,
        remote_value: value.remote_value,
        description: value.description,
        visible_for_mcp: value.visible_for_mcp,
    })
}

pub async fn load_secret_value(
    _env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<SecretValueApiModel, String> {
    let url = format!(
        "{}/api/v1/secrets/get?product_id={}&secret_id={}",
        base_url(),
        urlencode(product_id.as_deref().unwrap_or("")),
        urlencode(&secret_id),
    );
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn load_secret_usage_by_secret(
    _env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<Vec<SecretUsageBySecretApiModel>, String> {
    let url = format!(
        "{}/api/v1/secrets/usage/by-secrets?product_id={}&secret_id={}",
        base_url(),
        urlencode(product_id.as_deref().unwrap_or("")),
        urlencode(&secret_id),
    );
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn load_secret_usage_by_templates(
    _env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<Vec<TemplateUsageApiModel>, String> {
    let url = format!(
        "{}/api/v1/secrets/usage/by-templates?product_id={}&secret_id={}",
        base_url(),
        urlencode(product_id.as_deref().unwrap_or("")),
        urlencode(&secret_id),
    );
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
}

fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char)
            }
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}
