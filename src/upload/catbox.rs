use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};

use super::response_text;

pub const API_URL: &str = "https://catbox.moe/user/api.php";

/// Upload image bytes to catbox.moe.
///
/// No authentication required. Returns the direct file URL.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str) -> Result<String> {
    let form = Form::new()
        .text("reqtype", "fileupload")
        .part("fileToUpload", Part::bytes(data).file_name("image"));

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to catbox")?;

    response_text(resp, "catbox").await
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
            .respond_with(
                ResponseTemplate::new(200).set_body_string("https://files.catbox.moe/abc123.png"),
            )
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri())
            .await
            .unwrap();
        assert_eq!(url, "https://files.catbox.moe/abc123.png");
    }
}
