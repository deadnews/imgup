use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://upload.imagekit.io/api/v1/files/upload";

#[derive(Deserialize)]
struct Response {
    url: String,
}

/// Upload image bytes to imagekit.io.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let ext = detect_format(&data)?;
    let ext_str = ext.extensions_str()[0];
    let filename = format!("img.{ext_str}");

    let form = Form::new()
        .text("fileName", filename.clone())
        .part("file", Part::bytes(data).file_name(filename));

    let resp = client
        .post(url)
        .basic_auth(key, Option::<&str>::None)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to imagekit")?;

    let resp: Response = parse_json(resp, "imagekit").await?;
    Ok(resp.url)
}

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn test_upload_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "fileId": "abc123",
                "name": "img.png",
                "url": "https://ik.imagekit.io/demo/img_demo.png"
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://ik.imagekit.io/demo/img_demo.png");
    }
}
