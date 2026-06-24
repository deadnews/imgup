mod beeimg;
mod catbox;
mod cloudinary;
mod fastpic;
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
mod lensdump;
mod pixeldrain;
mod pixhost;
mod postimages;
mod ptpimg;
mod sxcu;
mod thumbsnap;
mod tixte;
mod uplio;
mod uploadcare;
mod vgy;
mod zpic;

use anyhow::{Context, Result, ensure};
use clap::ValueEnum;
use reqwest::Client;
use serde::de::DeserializeOwned;
use tracing::debug;

use crate::util::get_env;

/// Parse a JSON response, checking status and logging the body at debug level on failure.
pub(crate) async fn parse_json<T: DeserializeOwned>(
    resp: reqwest::Response,
    provider: &str,
) -> Result<T> {
    let body = response_text(resp, provider).await?;
    serde_json::from_str(&body)
        .inspect_err(|_| debug!("Response text:\n{body}"))
        .with_context(|| format!("failed to parse {provider} response"))
}

/// Read response text, checking status first.
pub(crate) async fn response_text(resp: reqwest::Response, provider: &str) -> Result<String> {
    let status = resp.status();
    let body = resp
        .text()
        .await
        .with_context(|| format!("failed to read {provider} response"))?;

    ensure!(status.is_success(), "{provider} returned {status}: {body}");
    Ok(body)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Hosting {
    Beeimg,
    Catbox,
    Cloudinary,
    Fastpic,
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
    Lensdump,
    Pixeldrain,
    Pixhost,
    Postimages,
    Ptpimg,
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
    let url = match hosting {
        Hosting::Beeimg => beeimg::upload(client, data, beeimg::API_URL).await,
        Hosting::Catbox => catbox::upload(client, data, catbox::API_URL).await,
        Hosting::Fastpic => fastpic::upload(client, data, fastpic::API_URL).await,
        Hosting::Pixhost => pixhost::upload(client, data, pixhost::API_URL).await,
        Hosting::Sxcu => sxcu::upload(client, data, sxcu::API_URL).await,
        Hosting::Cloudinary => {
            let cloud_name = get_env("CLOUDINARY_CLOUD_NAME")?;
            let api_key = get_env("CLOUDINARY_API_KEY")?;
            let api_secret = get_env("CLOUDINARY_API_SECRET")?;
            let url = format!("{}/{cloud_name}/image/upload", cloudinary::API_URL);
            cloudinary::upload(client, data, &url, &api_key, &api_secret).await
        }
        Hosting::Freeimage => {
            let key = get_env("FREEIMAGE_KEY")?;
            freeimage::upload(client, data, freeimage::API_URL, &key).await
        }
        Hosting::Gofile => gofile::upload(client, data, gofile::API_URL).await,
        Hosting::Gyazo => {
            let token = get_env("GYAZO_TOKEN")?;
            gyazo::upload(client, data, gyazo::API_URL, &token).await
        }
        Hosting::Imageban => {
            let token = get_env("IMAGEBAN_TOKEN")?;
            imageban::upload(client, data, imageban::API_URL, &token).await
        }
        Hosting::Imagekit => {
            let key = get_env("IMAGEKIT_PRIVATE_KEY")?;
            imagekit::upload(client, data, imagekit::API_URL, &key).await
        }
        Hosting::Imgbb => {
            let key = get_env("IMGBB_KEY")?;
            imgbb::upload(client, data, imgbb::API_URL, &key).await
        }
        Hosting::Imghippo => {
            let key = get_env("IMGHIPPO_KEY")?;
            imghippo::upload(client, data, imghippo::API_URL, &key).await
        }
        Hosting::Imglink => {
            let key = get_env("IMGLINK_KEY")?;
            imglink::upload(client, data, imglink::API_URL, &key).await
        }
        Hosting::Imgbox => imgbox::upload(client, data, imgbox::API_URL).await,
        Hosting::Imgchest => {
            let key = get_env("IMGCHEST_KEY")?;
            imgchest::upload(client, data, imgchest::API_URL, &key).await
        }
        Hosting::Imgur => {
            let client_id = std::env::var("IMGUR_CLIENT_ID")
                .unwrap_or_else(|_| imgur::DEFAULT_CLIENT_ID.to_owned());
            imgur::upload(client, data, imgur::API_URL, &client_id).await
        }
        Hosting::Lensdump => {
            let key = get_env("LENSDUMP_KEY")?;
            lensdump::upload(client, data, lensdump::API_URL, &key).await
        }
        Hosting::Pixeldrain => {
            let key = get_env("PIXELDRAIN_KEY")?;
            pixeldrain::upload(client, data, pixeldrain::API_URL, &key).await
        }
        Hosting::Postimages => {
            let key = get_env("POSTIMAGES_KEY")?;
            postimages::upload(client, data, postimages::API_URL, &key).await
        }
        Hosting::Ptpimg => {
            let key = get_env("PTPIMG_KEY")?;
            ptpimg::upload(client, data, ptpimg::API_URL, &key).await
        }
        Hosting::Thumbsnap => {
            let key = get_env("THUMBSNAP_KEY")?;
            thumbsnap::upload(client, data, thumbsnap::API_URL, &key).await
        }
        Hosting::Tixte => {
            let key = get_env("TIXTE_KEY")?;
            tixte::upload(client, data, tixte::API_URL, &key).await
        }
        Hosting::Uplio => {
            let key = get_env("UPLIO_KEY")?;
            uplio::upload(client, data, uplio::API_URL, &key).await
        }
        Hosting::Uploadcare => {
            let key = get_env("UPLOADCARE_KEY")?;
            uploadcare::upload(client, data, uploadcare::API_URL, &key).await
        }
        Hosting::Vgy => {
            let key = get_env("VGY_KEY")?;
            vgy::upload(client, data, vgy::API_URL, &key).await
        }
        Hosting::Zpic => {
            let key = get_env("ZPIC_KEY")?;
            zpic::upload(client, data, zpic::API_URL, &key).await
        }
    }?;

    ensure!(!url.is_empty(), "{hosting} returned empty URL");
    Ok(url)
}
