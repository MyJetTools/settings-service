use crate::models::*;

use super::base_url;

pub async fn get_envs() -> Result<EnvsHttpResponse, String> {
    let url = format!("{}/api/v1/env", base_url());
    let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("GET {url} → {}", resp.status()));
    }
    let info: EnvInfoModel = resp.json().await.map_err(|e| e.to_string())?;
    Ok(EnvsHttpResponse {
        name: info.name.clone(),
        envs: vec![info.name],
        prompt_ssh_pass_key: false,
    })
}
