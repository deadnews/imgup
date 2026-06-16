use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://api.gofile.io/servers";

#[derive(Deserialize)]
struct ServersResponse {
    data: ServersData,
}

#[derive(Deserialize)]
struct ServersData {
    servers: Vec<Server>,
}

#[derive(Deserialize)]
struct Server {
    name: String,
}

#[derive(Deserialize)]
struct UploadResponse {
    data: UploadData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadData {
    download_page: String,
}

/// Upload image bytes to gofile.io.
///
/// No API key required.
pub async fn upload(client: &Client, data: Vec<u8>, servers_url: &str) -> Result<String> {
    let upload_url = get_server(client, servers_url).await?;
    upload_to(client, data, &upload_url).await
}

async fn get_server(client: &Client, servers_url: &str) -> Result<String> {
    let resp = client
        .get(servers_url)
        .send()
        .await
        .context("failed to fetch gofile server list")?;

    let resp: ServersResponse = parse_json(resp, "gofile servers").await?;
    let server = resp
        .data
        .servers
        .into_iter()
        .next()
        .context("gofile returned no servers")?;

    Ok(format!(
        "https://{}.gofile.io/contents/uploadfile",
        server.name
    ))
}

async fn upload_to(client: &Client, data: Vec<u8>, upload_url: &str) -> Result<String> {
    let ext = detect_format(&data)?;
    let ext_str = ext.extensions_str()[0];

    let form = Form::new().part(
        "file",
        Part::bytes(data).file_name(format!("img.{ext_str}")),
    );

    let resp = client
        .post(upload_url)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to gofile")?;

    let resp: UploadResponse = parse_json(resp, "gofile").await?;
    Ok(resp.data.download_page)
}

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn test_get_server() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {"servers": [{"name": "store1", "zone": "eu"}]}
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = get_server(&client, &mock_server.uri()).await.unwrap();
        assert_eq!(url, "https://store1.gofile.io/contents/uploadfile");
    }

    #[tokio::test]
    async fn test_upload_to() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "data": {"downloadPage": "https://gofile.io/d/YxNORW"}
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload_to(&client, png, &mock_server.uri()).await.unwrap();
        assert_eq!(url, "https://gofile.io/d/YxNORW");
    }
}
