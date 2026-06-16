use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://api.cloudinary.com/v1_1";

#[derive(Deserialize)]
struct Response {
    secure_url: String,
}

/// Upload image bytes to cloudinary.com.
///
/// Requires cloud name, API key, and API secret.
pub async fn upload(
    client: &Client,
    data: Vec<u8>,
    url: &str,
    api_key: &str,
    api_secret: &str,
) -> Result<String> {
    let ext = detect_format(&data)?;
    let ext_str = ext.extensions_str()[0];

    let form = Form::new().part(
        "file",
        Part::bytes(data).file_name(format!("img.{ext_str}")),
    );

    let resp = client
        .post(url)
        .basic_auth(api_key, Some(api_secret))
        .multipart(form)
        .send()
        .await
        .context("failed to send request to cloudinary")?;

    let resp: Response = parse_json(resp, "cloudinary").await?;
    Ok(resp.secure_url)
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
                "secure_url": "https://res.cloudinary.com/demo/image/upload/v1/img.png",
                "public_id": "img"
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri(), "test_key", "test_secret")
            .await
            .unwrap();
        assert_eq!(
            url,
            "https://res.cloudinary.com/demo/image/upload/v1/img.png"
        );
    }
}
