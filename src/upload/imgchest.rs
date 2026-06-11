use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://api.imgchest.com/v1/post";

#[derive(Deserialize)]
struct Response {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    images: Vec<Image>,
}

#[derive(Deserialize)]
struct Image {
    link: String,
}

/// Upload image bytes to imgchest.com.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let ext = detect_format(&data)?;
    let ext_str = ext.extensions_str()[0];
    let filename = format!("img.{ext_str}");

    let form = Form::new().part("images[]", Part::bytes(data).file_name(filename));

    let resp = client
        .post(url)
        .header("Authorization", format!("Bearer {key}"))
        .multipart(form)
        .send()
        .await
        .context("failed to send request to imgchest")?;

    let resp: Response = parse_json(resp, "imgchest").await?;

    let image = resp
        .data
        .images
        .into_iter()
        .next()
        .context("imgchest returned no images")?;
    Ok(image.link)
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
            .and(header("Authorization", "Bearer test_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "images": [{"link": "https://cdn.imgchest.com/files/3yrgcr3jpp4.png"}]
                }
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://cdn.imgchest.com/files/3yrgcr3jpp4.png");
    }
}
