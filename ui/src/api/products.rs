use crate::models::*;

use super::base_url;

pub async fn load_all_products(_env_id: String) -> Result<Vec<ProductHttpModel>, String> {
    let url = format!("{}/api/v1/products", base_url());
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn load_product(
    _env_id: String,
    product_id: String,
) -> Result<ProductHttpModel, String> {
    let url = format!(
        "{}/api/v1/products/get?product_id={}",
        base_url(),
        urlencode(&product_id),
    );
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn save_product(
    _env_id: String,
    value: UpdateProductHttpModel,
) -> Result<(), String> {
    let url = format!("{}/api/v1/products", base_url());
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
struct DeleteProductPayload<'a> {
    product_id: &'a str,
}

pub async fn delete_product(_env_id: String, product_id: String) -> Result<(), String> {
    let url = format!("{}/api/v1/products/delete", base_url());
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&DeleteProductPayload {
            product_id: &product_id,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("POST {url} → {}", resp.status()));
    }
    Ok(())
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
