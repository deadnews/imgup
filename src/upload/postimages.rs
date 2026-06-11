use anyhow::{Context, Result, anyhow};
use base64::Engine as _;
use reqwest::Client;
use tracing::debug;

use super::response_text;

pub const API_URL: &str = "https://api.postimage.org/1/upload";

/// Upload image bytes to postimages.org.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let image_b64 = base64::engine::general_purpose::STANDARD.encode(&data);
    // o/m are tokens required by the API.
    let resp = client
        .post(url)
        .form(&[
            ("key", key),
            ("image", image_b64.as_str()),
            ("name", "img"),
            ("type", "jpg"),
            ("version", "1.0.1"),
            ("o", "2b819584285c102318568238c7d4a4c7"),
            ("m", "fb733cccce28e7db3ff9f17d7ccff3d1"),
        ])
        .send()
        .await
        .context("failed to send request to postimages")?;

    let body = response_text(resp, "postimages").await?;
    extract_hotlink(&body)
}

/// Extract the direct image URL from the XML response.
fn extract_hotlink(xml: &str) -> Result<String> {
    xml.split_once("<hotlink>")
        .and_then(|(_, rest)| rest.split_once("</hotlink>"))
        .map(|(url, _)| url.to_owned())
        .inspect(|url| debug!("postimages hotlink: {url}"))
        .ok_or_else(|| {
            debug!("Response text:\n{xml}");
            anyhow!("hotlink not found in postimages response")
        })
}

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const XML_RESPONSE: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<data success="1" status="200">
  <links>
    <hotlink>https://i.postimg.cc/abc123/image.jpg</hotlink>
  </links>
</data>"#;

    #[tokio::test]
    async fn test_upload_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_string(XML_RESPONSE))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let url = upload(&client, vec![1, 2, 3], &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://i.postimg.cc/abc123/image.jpg");
    }

    #[test]
    fn test_extract_hotlink() {
        let url = extract_hotlink(XML_RESPONSE).unwrap();
        assert_eq!(url, "https://i.postimg.cc/abc123/image.jpg");
    }
}
