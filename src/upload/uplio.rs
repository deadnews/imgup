use anyhow::{Context, Result, bail};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use tracing::debug;

use super::response_text;
use crate::image::detect_format;

pub const API_URL: &str = "https://upl.io";

/// Upload image bytes to upl.io.
///
/// Requires API key.
pub async fn upload(client: &Client, data: Vec<u8>, url: &str, key: &str) -> Result<String> {
    let ext = detect_format(&data)?;
    let ext_str = ext.extensions_str()[0];
    let filename = format!("img.{ext_str}");

    let form = Form::new()
        .text("key", key.to_owned())
        .part("file", Part::bytes(data).file_name(filename));

    let resp = client
        .post(url)
        .multipart(form)
        .send()
        .await
        .context("failed to send request to uplio")?;

    let text = response_text(resp, "uplio").await?;

    // Convert "https://upl.io/UID[.ext]" to "https://upl.io/i/UID.ext"
    let Some((host, uid)) = text.rsplit_once('/') else {
        debug!("Response text:\n{text}");
        bail!("unexpected uplio response format");
    };
    let uid_base = uid.rsplit_once('.').map_or(uid, |(base, _)| base);
    Ok(format!("{host}/i/{uid_base}.{ext_str}"))
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
            .respond_with(ResponseTemplate::new(200).set_body_string("https://upl.io/0w25y7"))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri(), "test_key")
            .await
            .unwrap();
        assert_eq!(url, "https://upl.io/i/0w25y7.png");
    }
}
