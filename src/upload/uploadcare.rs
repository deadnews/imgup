use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://upload.uploadcare.com/base/";

#[derive(Deserialize)]
struct Response {
    filename: String,
}

/// Upload image bytes to uploadcare.com.
///
/// Requires API key (public key).
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let ext = detect_format(&data)?;
    let ext_str = ext.extensions_str()[0];

    let form = Form::new()
        .text("UPLOADCARE_PUB_KEY", key.to_owned())
        .text("UPLOADCARE_STORE", "1")
        .part(
            "filename",
            Part::bytes(data).file_name(format!("img.{ext_str}")),
        );

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to uploadcare")?;

    let resp: Response = parse_json(resp, "uploadcare").await?;
    Ok(format!(
        "https://ucarecdn.com/{}/img.{ext_str}",
        resp.filename
    ))
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
                "filename": "7eb40c10-fc9b-42a2-a086-e9eef5bafb9c"
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(
            url,
            "https://ucarecdn.com/7eb40c10-fc9b-42a2-a086-e9eef5bafb9c/img.png"
        );
    }
}
