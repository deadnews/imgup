use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;

pub const API_URL: &str = "https://upload.gyazo.com/api/upload";

#[derive(Deserialize)]
struct Response {
    url: String,
}

/// Upload image bytes to gyazo.com.
///
/// Requires access token.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, token: &str) -> Result<String> {
    let form = Form::new().part("imagedata", Part::bytes(data).file_name("image"));

    let resp = client
        .post(url)
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to gyazo")?;

    let resp: Response = parse_json(resp, "gyazo").await?;
    Ok(resp.url)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn test_upload_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "url": "https://i.gyazo.com/45927ce6c3a6ba2e48a260328dc57d3d.png"
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri(), "test_token")
            .await
            .unwrap();
        assert_eq!(
            url,
            "https://i.gyazo.com/45927ce6c3a6ba2e48a260328dc57d3d.png"
        );
    }
}
