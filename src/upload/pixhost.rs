use anyhow::{Context, Result};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;
use tracing::debug;

use super::parse_json;

pub const API_URL: &str = "https://api.pixhost.to/images";

#[derive(Deserialize)]
struct Response {
    show_url: String,
}

/// Upload image bytes to pixhost.to.
///
/// No authentication required. Fetches the direct image link from the show page.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str) -> Result<String> {
    let form = Form::new()
        .text("content_type", "0")
        .part("img", Part::bytes(data).file_name("image"));

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to pixhost")?;

    let resp: Response = parse_json(resp, "pixhost").await?;

    Ok(extract_direct_link(client, &resp.show_url)
        .await
        .inspect_err(|e| debug!("pixhost direct link extraction failed: {e:#}"))
        .unwrap_or(resp.show_url))
}

/// Fetch the show page and extract the direct image URL.
async fn extract_direct_link(client: &Client, show_url: &str) -> Result<String> {
    let page = client
        .get(show_url)
        .send()
        .await?
        .text()
        .await
        .context("failed to read pixhost show page")?;

    // Extract path after /show/ to match against direct image URLs
    let show_path = show_url
        .split_once("/show/")
        .map(|(_, path)| path)
        .context("unexpected show_url format")?;

    // Look for the direct image URL in the page
    let pattern = format!("/images/{show_path}");
    for segment in page.split('"') {
        if segment.contains(&pattern) && segment.starts_with("https://") {
            return Ok(segment.to_owned());
        }
    }

    debug!("Response text:\n{page}");
    anyhow::bail!("direct link not found in pixhost show page")
}

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn test_upload_extracts_direct_link() {
        let mock_server = MockServer::start().await;

        let show_url = format!("{}/show/865/361824612_upload.png", mock_server.uri());
        let direct_url = "https://img865.pixhost.to/images/865/361824612_upload.png";

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "upload.png",
                "show_url": show_url,
                "th_url": "https://t865.pixhost.to/thumbs/865/361824612_upload.png"
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(format!(r#"<img src="{direct_url}">"#)),
            )
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri())
            .await
            .unwrap();
        assert_eq!(url, direct_url);
    }

    #[tokio::test]
    async fn test_upload_fallback_to_show_url() {
        let mock_server = MockServer::start().await;

        let show_url = format!("{}/show/865/361824612_upload.png", mock_server.uri());
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "upload.png",
                "show_url": show_url,
                "th_url": "https://t865.pixhost.to/thumbs/865/361824612_upload.png"
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        // Mock doesn't serve the show page, so falls back to show_url
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri())
            .await
            .unwrap();
        assert_eq!(url, show_url);
    }
}
