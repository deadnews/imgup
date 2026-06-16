use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://api.tixte.com/v1/upload";

#[derive(Deserialize)]
struct Response {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    direct_url: String,
}

/// Upload image bytes to tixte.com.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let ext = detect_format(&data)?;
    let ext_str = ext.extensions_str()[0];
    let filename = format!("img.{ext_str}");

    let form = Form::new()
        .text("payload_json", r#"{"random":true}"#)
        .part("file", Part::bytes(data).file_name(filename));

    let resp = client
        .post(url)
        .header("Authorization", key)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to tixte")?;

    let resp: Response = parse_json(resp, "tixte").await?;
    Ok(resp.data.direct_url)
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
            .and(header("Authorization", "test_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {"direct_url": "https://cdx.tixte.co/r/lj5zsvnkwa0.png"}
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://cdx.tixte.co/r/lj5zsvnkwa0.png");
    }
}
