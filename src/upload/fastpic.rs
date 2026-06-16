use anyhow::{Context, Result, bail};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use tracing::debug;

use super::response_text;

pub const API_URL: &str = "https://fastpic.org/upload?api=1";

/// Upload image bytes to fastpic.org.
///
/// No authentication required. Returns XML response, parsed for image URL.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str) -> Result<String> {
    let form = Form::new()
        .text("method", "file")
        .text("check_thumb", "no")
        .text("uploading", "1")
        .part("file1", Part::bytes(data).file_name("image"));

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to fastpic")?;

    let body = response_text(resp, "fastpic").await?;

    if let Some(link) = extract_tag(&body, "imagepath") {
        return Ok(link);
    }

    debug!("Response text:\n{body}");
    match extract_tag(&body, "error") {
        Some(e) => bail!("fastpic error: {e}"),
        None => bail!("image link not found in fastpic response"),
    }
}

/// Extract trimmed text content of an XML tag.
fn extract_tag(body: &str, tag: &str) -> Option<String> {
    let (_, rest) = body.split_once(&format!("<{tag}>"))?;
    let (inner, _) = rest.split_once(&format!("</{tag}>"))?;
    let trimmed = inner.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
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

        let xml = "<imagepath>https://i122.fastpic.org/big/2023/image.png</imagepath>";
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_string(xml))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri())
            .await
            .unwrap();
        assert_eq!(url, "https://i122.fastpic.org/big/2023/image.png");
    }

    #[tokio::test]
    async fn test_upload_error_response() {
        let mock_server = MockServer::start().await;

        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<UploadSettings>
<imagepath></imagepath>
<status>err</status>
<error>Sorry, uploading are not allowed.</error>
</UploadSettings>"#;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_string(xml))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let err = upload(&client, vec![1, 2, 3], &mock_server.uri())
            .await
            .unwrap_err();
        assert!(
            err.to_string()
                .contains("Sorry, uploading are not allowed."),
            "expected server error message, got: {err}"
        );
    }
}
