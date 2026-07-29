use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://redacted.sh/ajax.php?action=upload_image";

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "response")]
    data: Inner,
}

#[derive(Deserialize)]
struct Inner {
    url: String,
}

/// Upload image bytes to redacted.sh.
///
/// Requires API key, sent as the `Authorization` header.
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
        .header("Authorization", key)
        .multipart(form)
        .send()
        .await?;

    let resp: Response = parse_json(resp, "redacted").await?;
    Ok(resp.data.url)
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
            .and(header("Authorization", "test_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "success",
                "response": {
                    "url": "https://redacted.sh/i/uKYzElBaXhc.png",
                    "thumbUrl": "https://redacted.sh/t/uKYzElBaXhc.png"
                }
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://redacted.sh/i/uKYzElBaXhc.png");
    }

    #[tokio::test]
    async fn test_upload_failure() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "status": "failure",
                "error": "bad credentials"
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let err = upload(&client, png, &mock_server.uri(), "bad_key")
            .await
            .unwrap_err();
        assert!(err.to_string().contains("bad credentials"));
    }
}
