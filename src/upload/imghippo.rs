use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://api.imghippo.com/v1/upload";

#[derive(Deserialize)]
struct Response {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    url: String,
}

/// Upload image bytes to imghippo.com.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let ext = detect_format(&data)?;
    let ext_str = ext.extensions_str()[0];

    let form = Form::new().text("api_key", key.to_owned()).part(
        "file",
        Part::bytes(data).file_name(format!("img.{ext_str}")),
    );

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to imghippo")?;

    let resp: Response = parse_json(resp, "imghippo").await?;
    Ok(resp.data.url)
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
                "success": true,
                "status": 200,
                "data": {"url": "https://i.imghippo.com/files/abc123.png"}
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://i.imghippo.com/files/abc123.png");
    }
}
