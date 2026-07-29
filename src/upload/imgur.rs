use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;

pub const API_URL: &str = "https://api.imgur.com/3/image";
pub const DEFAULT_CLIENT_ID: &str = "dd32dd3c6aaa9a0";

#[derive(Deserialize)]
struct Response {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    link: String,
}

/// Upload image bytes to imgur.com.
///
/// Uses provided client ID for authentication.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, client_id: &str) -> Result<String> {
    let form = Form::new().part("image", Part::bytes(data).file_name("image"));

    let resp = client
        .post(url)
        .header("Authorization", format!("Client-ID {client_id}"))
        .multipart(form)
        .send()
        .await
        .context("failed to send request to imgur")?;

    let resp: Response = parse_json(resp, "imgur").await?;
    Ok(resp.data.link)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn test_upload_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(header("Authorization", "Client-ID test_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {"link": "https://i.imgur.com/abc123.png"},
                "success": true
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri(), "test_id")
            .await
            .unwrap();
        assert_eq!(url, "https://i.imgur.com/abc123.png");
    }
}
