use anyhow::Result;
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;

pub const API_URL: &str = "https://freeimage.host/api/1/upload";

#[derive(Deserialize)]
struct Response {
    image: Image,
}

#[derive(Deserialize)]
struct Image {
    url: String,
}

/// Upload image bytes to freeimage.host.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let form = Form::new()
        .text("key", key.to_owned())
        .part("source", Part::bytes(data).file_name("image"));

    let resp = client.post(url).multipart(form).send().await?;

    let resp: Response = parse_json(resp, "freeimage").await?;
    Ok(resp.image.url)
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
                "image": {"url": "https://iili.io/HPTxvWu.png"}
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://iili.io/HPTxvWu.png");
    }
}
