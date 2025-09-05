use crate::types::JimakuFile;
use reqwest::Client;

pub async fn fetch_jimaku_files(
    client: &Client,
    entry_id: u32,
    auth_token: &str,
) -> Result<Vec<JimakuFile>, Box<dyn std::error::Error>> {
    let url = format!("https://jimaku.cc/api/entries/{}/files", entry_id);

    let response = client
        .get(&url)
        .header("Authorization", auth_token)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("API request failed: {}", response.status()).into());
    }

    let files: Vec<JimakuFile> = response.json().await?;
    Ok(files)
}