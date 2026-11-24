use anyhow::Result;
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;

pub const API_URL: &str = "https://basepic.com/api/v1/images";

#[derive(Deserialize)]
struct Response {
    image_url: String,
}

/// Upload image bytes to basepic.com.
///
/// No authentication required. Returns the direct image URL.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str) -> Result<String> {
    let form = Form::new().part("image", Part::bytes(data).file_name("image"));

    let resp = client.post(url).multipart(form).send().await?;

    let resp: Response = parse_json(resp, "basepic").await?;
    Ok(resp.image_url)
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
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": "abc123",
                "page_url": "https://basepic.com/abc123",
                "image_url": "https://basepic.com/abc123/image.png"
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri())
            .await
            .unwrap();
        assert_eq!(url, "https://basepic.com/abc123/image.png");
    }
}
