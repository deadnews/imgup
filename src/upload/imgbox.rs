use anyhow::{Context, Result, bail};
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::Deserialize;
use tracing::debug;

use super::{parse_json, response_text};
use crate::image::detect_format;
use crate::{TIMEOUT, USER_AGENT};

pub const API_URL: &str = "https://imgbox.com";

#[derive(Deserialize)]
struct TokenResponse {
    token_id: u64,
    token_secret: String,
    gallery_id: Option<String>,
    gallery_secret: Option<String>,
}

#[derive(Deserialize)]
struct UploadFile {
    original_url: String,
}

#[derive(Deserialize)]
struct UploadResponse {
    files: Vec<UploadFile>,
}

/// Upload image bytes to imgbox.com.
///
/// No API key required. Uses a CSRF token + session cookie flow.
/// A local cookie-aware client maintains the session across the three steps.
pub async fn upload(_client: &Client, data: Vec<u8>, url: &str) -> Result<String> {
    let ext = detect_format(&data)?;
    let filename = format!("image.{}", ext.extensions_str()[0]);

    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .cookie_store(true)
        .timeout(TIMEOUT)
        .build()
        .context("failed to build imgbox client")?;

    let csrf_token = fetch_csrf(&client, url).await?;
    let token = fetch_token(&client, url, &csrf_token).await?;

    let gallery_id = token.gallery_id.unwrap_or_else(|| "null".to_owned());
    let gallery_secret = token.gallery_secret.unwrap_or_else(|| "null".to_owned());

    let form = Form::new()
        .text("token_id", token.token_id.to_string())
        .text("token_secret", token.token_secret)
        .text("gallery_id", gallery_id)
        .text("gallery_secret", gallery_secret)
        .text("content_type", "1")
        .text("thumbnail_size", "100r")
        .text("comments_enabled", "0")
        .part("files[]", Part::bytes(data).file_name(filename));

    let resp = client
        .post(format!("{url}/upload/process"))
        .header("X-CSRF-Token", &csrf_token)
        .multipart(form)
        .send()
        .await
        .context("failed to send upload request to imgbox")?;

    let resp: UploadResponse = parse_json(resp, "imgbox").await?;

    let file = resp
        .files
        .into_iter()
        .next()
        .context("imgbox returned no files")?;
    Ok(file.original_url)
}

async fn fetch_csrf(client: &Client, base_url: &str) -> Result<String> {
    let resp = client
        .get(base_url)
        .send()
        .await
        .context("failed to fetch imgbox main page")?;
    let body = response_text(resp, "imgbox").await?;
    let csrf_token = extract_csrf_token(&body)?;
    debug!("csrf_token={csrf_token}");
    Ok(csrf_token)
}

fn extract_csrf_token(html: &str) -> Result<String> {
    // Find the line with "csrf-token" and extract content="..." (any attribute order)
    for line in html.lines() {
        if line.contains("csrf-token")
            && let Some(after) = line.split("content=\"").nth(1)
            && let Some(token) = after.split('"').next()
            && !token.is_empty()
        {
            return Ok(token.to_owned());
        }
    }
    debug!("HTML:\n{html}");
    bail!("csrf token not found in imgbox page")
}

async fn fetch_token(client: &Client, base_url: &str, csrf_token: &str) -> Result<TokenResponse> {
    let resp = client
        .post(format!("{base_url}/ajax/token/generate"))
        .header("X-CSRF-Token", csrf_token)
        .form(&[
            ("gallery", "true"),
            ("gallery_title", ""),
            ("comments_enabled", "0"),
        ])
        .send()
        .await
        .context("failed to fetch imgbox upload token")?;

    parse_json(resp, "imgbox token").await
}

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn test_upload_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200)
                    .append_header("set-cookie", "_imgbox_session=sess; Path=/; HttpOnly")
                    .set_body_string(r#"<meta content="test_csrf" name="csrf-token" />"#),
            )
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/ajax/token/generate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "ok": true,
                "token_id": 12345,
                "token_secret": "tsec",
                "gallery_id": "gid",
                "gallery_secret": "gsec"
            })))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/upload/process"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "ok": true,
                "files": [{"original_url": "https://images2.imgbox.com/test/img_o.png"}]
            })))
            .mount(&mock_server)
            .await;

        let png = crate::image::create_test_png();

        let client = Client::new();
        let url = upload(&client, png, &mock_server.uri()).await.unwrap();
        assert_eq!(url, "https://images2.imgbox.com/test/img_o.png");
    }

    #[test]
    fn test_extract_csrf_token() {
        let html = r#"<meta content="abc123" name="csrf-token" />"#;
        assert_eq!(extract_csrf_token(html).unwrap(), "abc123");
    }

    #[test]
    fn test_extract_csrf_token_missing() {
        assert!(extract_csrf_token("<html><head></head></html>").is_err());
    }
}
