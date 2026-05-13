use crate::models::*;

use super::base_url;

pub async fn get_templates(_env_id: String) -> Result<Vec<TemplateHttpModel>, String> {
    let url = format!("{}/api/v1/templates", base_url());
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn save_template(
    _env_id: String,
    data: UpdateTemplateHttpModel,
) -> Result<(), String> {
    let url = format!("{}/api/v1/templates", base_url());
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&data)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("POST {url} → {}", resp.status()));
    }
    Ok(())
}

#[derive(serde::Serialize)]
struct DeleteTemplatePayload<'a> {
    product_id: &'a str,
    template_id: &'a str,
}

pub async fn delete_template(
    _env_id: String,
    product_id: String,
    template_id: String,
) -> Result<(), String> {
    let url = format!("{}/api/v1/templates/delete", base_url());
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&DeleteTemplatePayload {
            product_id: &product_id,
            template_id: &template_id,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("POST {url} → {}", resp.status()));
    }
    Ok(())
}

#[derive(serde::Deserialize)]
struct TemplateContentResponse {
    content: String,
}

pub async fn get_template_content(
    _env_id: String,
    product_id: String,
    template_id: String,
) -> Result<String, String> {
    let url = format!(
        "{}/api/v1/templates/content?product_id={}&template_id={}",
        base_url(),
        urlencode(&product_id),
        urlencode(&template_id),
    );
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    let body: TemplateContentResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.content)
}

#[derive(serde::Serialize)]
struct CompileYamlPayload<'a> {
    product_id: &'a str,
    template_id: &'a str,
}

pub async fn load_yaml(
    _env_id: String,
    product_id: String,
    template_id: String,
) -> Result<PopulatedYamlModelApiModel, String> {
    let url = format!("{}/api/v1/templates/yaml", base_url());
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&CompileYamlPayload {
            product_id: &product_id,
            template_id: &template_id,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("POST {url} → {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
}

pub async fn download_snapshot(
    _env_id: String,
    request: Vec<DownloadFileRequestModel>,
) -> Result<String, String> {
    let product_id = request
        .first()
        .map(|r| r.product_id.clone())
        .unwrap_or_default();

    if product_id.is_empty() {
        return Err("No product selected for export".to_string());
    }

    let url = format!(
        "{}/api/v1/templates/snapshot/export?product_id={}",
        base_url(),
        urlencode(&product_id),
    );
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    resp.text().await.map_err(|e| e.to_string())
}

pub async fn upload_snapshot(_env_id: String, snapshot: String) -> Result<(), String> {
    // best-effort product extraction: leave to first template product or empty
    let product_id = serde_json::from_str::<serde_json::Value>(&snapshot)
        .ok()
        .and_then(|v| {
            v.get("templates")
                .and_then(|t| t.get(0))
                .and_then(|t0| t0.get("product_id"))
                .and_then(|p| p.as_str().map(|s| s.to_string()))
        })
        .unwrap_or_default();

    let form = reqwest::multipart::Form::new().part(
        "snapshot",
        reqwest::multipart::Part::bytes(snapshot.into_bytes())
            .file_name("snapshot.json")
            .mime_str("application/json")
            .map_err(|e| e.to_string())?,
    );

    let url = format!(
        "{}/api/v1/templates/snapshot/import?product_id={}",
        base_url(),
        urlencode(&product_id),
    );

    let resp = reqwest::Client::new()
        .post(&url)
        .multipart(form)
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
