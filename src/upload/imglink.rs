use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://imglink.cc/api/v1/upload";

#[derive(Deserialize)]
struct Response {
    url: String,
}

/// Upload image bytes to imglink.cc.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let ext = detect_format(&data)?;
    let name = format!("img.{}", ext.extensions_str()[0]);
    let content_type = ext.to_mime_type();

    let form = Form::new().part(
        "file",
        Part::bytes(data)
            .file_name(name)
            .mime_str(content_type)
            .context("invalid mime type")?,
    );

    let resp = client
        .post(url)
        .header("X-API-Key", key)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to imglink")?;

    let resp: Response = parse_json(resp, "imglink").await?;
    Ok(resp.url)
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
                "url": "https://imglink.cc/cdn/aBcDeFgHiJ.png"
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://imglink.cc/cdn/aBcDeFgHiJ.png");
    }
}
