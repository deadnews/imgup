use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://kappa.lol/api/upload";

#[derive(Deserialize)]
struct Response {
    link: String,
    ext: String,
}

/// Upload image bytes to kappa.lol.
///
/// No authentication required.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str) -> Result<String> {
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
        .multipart(form)
        .send()
        .await
        .context("failed to send request to kappa")?;

    let resp: Response = parse_json(resp, "kappa").await?;
    Ok(format!("{}{}", resp.link, resp.ext))
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
                "link": "https://kappa.lol/dBZyFV",
                "ext": ".png"
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri()).await.unwrap();
        assert_eq!(url, "https://kappa.lol/dBZyFV.png");
    }
}
