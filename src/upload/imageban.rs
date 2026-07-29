use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;

pub const API_URL: &str = "https://api.imageban.ru/v1";

#[derive(Deserialize)]
struct Response {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    link: String,
}

/// Upload image bytes to imageban.ru.
///
/// Requires API token.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, token: &str) -> Result<String> {
    let form = Form::new().part("image", Part::bytes(data).file_name("image"));

    let resp = client
        .post(url)
        .header("Authorization", format!("TOKEN {token}"))
        .multipart(form)
        .send()
        .await
        .context("failed to send request to imageban")?;

    let resp: Response = parse_json(resp, "imageban").await?;
    Ok(resp.data.link)
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
            .and(header("Authorization", "TOKEN test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {"link": "https://i5.imageban.ru/out/2023/06/21/image.png"}
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri(), "test_token")
            .await
            .unwrap();
        assert_eq!(url, "https://i5.imageban.ru/out/2023/06/21/image.png");
    }
}
