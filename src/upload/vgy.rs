use anyhow::Result;
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://vgy.me/upload";

#[derive(Deserialize)]
struct Response {
    image: String,
}

/// Upload image bytes to vgy.me.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let ext = detect_format(&data)?.extensions_str()[0];
    let filename = format!("img.{ext}");

    let form = Form::new()
        .text("userkey", key.to_owned())
        .part("file[]", Part::bytes(data).file_name(filename));

    let resp = client.post(url).multipart(form).send().await?;

    let resp: Response = parse_json(resp, "vgy").await?;
    Ok(resp.image)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn test_upload_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "error": false,
                "image": "https://i.vgy.me/3Kyfvf.png"
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://i.vgy.me/3Kyfvf.png");
    }
}
