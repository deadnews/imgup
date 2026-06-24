use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;

pub const API_URL: &str = "https://pixvid.org/api/1/upload";

#[derive(Deserialize)]
struct Response {
    image: Image,
}

#[derive(Deserialize)]
struct Image {
    url: String,
}

/// Upload image bytes to pixvid.org.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let form = Form::new()
        .text("format", "json")
        .part("source", Part::bytes(data).file_name("image"));

    let resp = client
        .post(url)
        .header("X-API-Key", key)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to pixvid")?;

    let resp: Response = parse_json(resp, "pixvid").await?;
    Ok(resp.image.url)
}

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use wiremock::matchers::{header, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn test_upload_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(header("X-API-Key", "test_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "image": {"url": "https://pixvid.org/images/2026/06/24/77z7e.png"}
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://pixvid.org/images/2026/06/24/77z7e.png");
    }
}
