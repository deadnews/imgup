use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use super::parse_json;
use crate::image::detect_format;

pub const API_URL: &str = "https://sxcu.net/api/files/create";

#[derive(Deserialize)]
struct Response {
    url: String,
}

/// Upload image bytes to sxcu.net.
///
/// No authentication required.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str) -> Result<String> {
    let ext = detect_format(&data)?;
    let ext_str = ext.extensions_str()[0];

    let form = Form::new().part("file", Part::bytes(data).file_name("image"));

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to sxcu")?;

    let resp: Response = parse_json(resp, "sxcu").await?;
    Ok(format!("{}.{ext_str}", resp.url))
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
                "id": "66iFnGoQ6",
                "url": "https://sxcu.net/66iFnGoQ6",
                "del_url": "https://sxcu.net/api/files/delete/66iFnGoQ6/token",
                "thumb": "https://sxcu.net/t/66iFnGoQ6.png"
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri()).await.unwrap();
        assert_eq!(url, "https://sxcu.net/66iFnGoQ6.png");
    }
}
