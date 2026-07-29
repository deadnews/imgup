mod beeimg;
mod catbox;
mod cloudinary;
mod fastpic;
mod filepost;
mod freeimage;
mod gofile;
mod gyazo;
mod imageban;
mod imagekit;
mod imgbb;
mod imgbox;
mod imgchest;
mod imghippo;
mod imglink;
mod imgur;
mod kappa;
mod lensdump;
mod pixeldrain;
mod pixhost;
mod pixvid;
mod postimages;
mod redacted;
mod sxcu;
mod thumbsnap;
mod tixte;
mod uplio;
mod uploadcare;
mod vgy;
mod zpic;

use anyhow::{Context, Result, bail, ensure};
use clap::ValueEnum;
use reqwest::Client;
use serde::de::DeserializeOwned;
use tracing::debug;

use crate::util::get_env;

/// Cap `body` for an error message, logging the full body at debug when truncated.
fn error_body(body: &str) -> String {
    const MAX_BYTES: usize = 128;
    let end = body.floor_char_boundary(MAX_BYTES);
    if end == body.len() {
        body.to_owned()
    } else {
        debug!("Response text:\n{body}");
        format!("{}…", &body[..end])
    }
}

/// Parse a JSON response, checking status first.
pub(crate) async fn parse_json<T: DeserializeOwned>(
    resp: reqwest::Response,
    provider: &str,
) -> Result<T> {
    let body = response_text(resp, provider).await?;
    serde_json::from_str(&body)
        .with_context(|| format!("failed to parse {provider} response: {}", error_body(&body)))
}

/// Read response text, checking status first.
pub(crate) async fn response_text(resp: reqwest::Response, provider: &str) -> Result<String> {
    let status = resp.status();
    let body = resp
        .text()
        .await
        .with_context(|| format!("failed to read {provider} response"))?;

    if !status.is_success() {
        bail!("{provider} returned {status}: {}", error_body(&body));
    }
    Ok(body)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Hosting {
    Beeimg,
    Catbox,
    Cloudinary,
    Fastpic,
    Filepost,
    Freeimage,
    Gofile,
    Gyazo,
    Imageban,
    Imagekit,
    Imgbb,
    Imgbox,
    Imgchest,
    Imghippo,
    Imglink,
    Imgur,
    Kappa,
    Lensdump,
    Pixeldrain,
    Pixhost,
    Pixvid,
    Postimages,
    Redacted,
    Sxcu,
    Thumbsnap,
    Tixte,
    Uplio,
    Uploadcare,
    Vgy,
    Zpic,
}

impl std::fmt::Display for Hosting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.to_possible_value()
                .expect("all variants have values")
                .get_name(),
        )
    }
}

/// Dispatch upload to the appropriate provider.
pub async fn upload(client: &Client, hosting: Hosting, data: Vec<u8>) -> Result<String> {
    macro_rules! anon {
        ($m:ident) => {
            $m::upload(client, data, $m::API_URL).await
        };
    }
    macro_rules! keyed {
        ($m:ident, $env:literal) => {
            $m::upload(client, data, $m::API_URL, &get_env($env)?).await
        };
    }

    let url = match hosting {
        Hosting::Beeimg => anon!(beeimg),
        Hosting::Catbox => anon!(catbox),
        Hosting::Fastpic => anon!(fastpic),
        Hosting::Gofile => anon!(gofile),
        Hosting::Kappa => anon!(kappa),
        Hosting::Pixhost => anon!(pixhost),
        Hosting::Sxcu => anon!(sxcu),
        Hosting::Filepost => keyed!(filepost, "FILEPOST_KEY"),
        Hosting::Freeimage => keyed!(freeimage, "FREEIMAGE_KEY"),
        Hosting::Gyazo => keyed!(gyazo, "GYAZO_TOKEN"),
        Hosting::Imageban => keyed!(imageban, "IMAGEBAN_TOKEN"),
        Hosting::Imagekit => keyed!(imagekit, "IMAGEKIT_PRIVATE_KEY"),
        Hosting::Imgbb => keyed!(imgbb, "IMGBB_KEY"),
        Hosting::Imgchest => keyed!(imgchest, "IMGCHEST_KEY"),
        Hosting::Imghippo => keyed!(imghippo, "IMGHIPPO_KEY"),
        Hosting::Imglink => keyed!(imglink, "IMGLINK_KEY"),
        Hosting::Lensdump => keyed!(lensdump, "LENSDUMP_KEY"),
        Hosting::Pixeldrain => keyed!(pixeldrain, "PIXELDRAIN_KEY"),
        Hosting::Pixvid => keyed!(pixvid, "PIXVID_KEY"),
        Hosting::Postimages => keyed!(postimages, "POSTIMAGES_KEY"),
        Hosting::Redacted => keyed!(redacted, "REDACTED_API_KEY"),
        Hosting::Thumbsnap => keyed!(thumbsnap, "THUMBSNAP_KEY"),
        Hosting::Tixte => keyed!(tixte, "TIXTE_KEY"),
        Hosting::Uplio => keyed!(uplio, "UPLIO_KEY"),
        Hosting::Uploadcare => keyed!(uploadcare, "UPLOADCARE_KEY"),
        Hosting::Vgy => keyed!(vgy, "VGY_KEY"),
        Hosting::Zpic => keyed!(zpic, "ZPIC_KEY"),
        Hosting::Imgbox => imgbox::upload(data, imgbox::API_URL).await,
        Hosting::Cloudinary => {
            let cloud_name = get_env("CLOUDINARY_CLOUD_NAME")?;
            let api_key = get_env("CLOUDINARY_API_KEY")?;
            let api_secret = get_env("CLOUDINARY_API_SECRET")?;
            let url = format!("{}/{cloud_name}/image/upload", cloudinary::API_URL);
            cloudinary::upload(client, data, &url, &api_key, &api_secret).await
        }
        Hosting::Imgur => {
            let client_id = std::env::var("IMGUR_CLIENT_ID")
                .unwrap_or_else(|_| imgur::DEFAULT_CLIENT_ID.to_owned());
            imgur::upload(client, data, imgur::API_URL, &client_id).await
        }
    }?;

    ensure!(!url.is_empty(), "{hosting} returned empty URL");
    Ok(url)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{Hosting, ValueEnum, error_body};

    #[test]
    fn test_harness_lists_every_hosting() {
        let mk = include_str!("../../Makefile.test.mk");
        let listed: BTreeSet<&str> = mk
            .lines()
            .find_map(|l| l.strip_prefix("HOSTINGS :="))
            .expect("HOSTINGS line")
            .split_whitespace()
            .collect();

        let all: Vec<String> = Hosting::value_variants()
            .iter()
            .map(ToString::to_string)
            .collect();

        assert_eq!(listed, all.iter().map(String::as_str).collect());
    }

    #[test]
    fn test_error_body_short_unchanged() {
        assert_eq!(error_body("boom"), "boom");
    }

    #[test]
    fn test_error_body_long_truncated() {
        let s = error_body(&"a".repeat(300));
        assert_eq!(s.chars().count(), 129);
        assert!(s.ends_with('…'));
    }

    #[test]
    fn test_error_body_multibyte_char_boundary() {
        let s = error_body(&"あ".repeat(100));
        assert_eq!(s.chars().count(), 43);
        assert!(s.ends_with('…'));
    }
}
