use anyhow::Result;
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;

pub const API_URL: &str = "https://api.pixelvault.dev/v1/images";

#[derive(Deserialize)]
struct Response {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    url: String,
}

/// Upload image bytes to pixelvault.dev.
///
/// Works without an API key (the link then expires after 30 days);
/// with a key the upload is permanent.
pub async fn upload(
    client: &Client,
    data: Vec<u8>,
    url: &str,
    key: Option<&str>,
) -> Result<String> {
    let form = Form::new().part("file", Part::bytes(data).file_name("image"));

    let mut req = client.post(url).multipart(form);
    if let Some(key) = key {
        req = req.bearer_auth(key);
    }

    let resp: Response = parse_json(req.send().await?, "pixelvault").await?;
    Ok(resp.data.url)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn created() -> ResponseTemplate {
        ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "data": {"id": "img_abc123", "url": "https://img.pixelvault.dev/proj_x/img_abc123.png"}
        }))
    }

    #[tokio::test]
    async fn test_upload_success_without_key() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(created())
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri(), None)
            .await
            .unwrap();
        assert_eq!(url, "https://img.pixelvault.dev/proj_x/img_abc123.png");
    }

    #[tokio::test]
    async fn test_upload_sends_bearer_key() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(header("Authorization", "Bearer pv_live_test"))
            .respond_with(created())
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(
            &client,
            vec![1, 2, 3],
            &mock_server.uri(),
            Some("pv_live_test"),
        )
        .await
        .unwrap();
        assert_eq!(url, "https://img.pixelvault.dev/proj_x/img_abc123.png");
    }
}
