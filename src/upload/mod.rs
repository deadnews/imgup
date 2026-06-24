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
        Hosting::Imgbox => anon!(imgbox),
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
        Hosting::Ptpimg => keyed!(ptpimg, "PTPIMG_KEY"),
        Hosting::Thumbsnap => keyed!(thumbsnap, "THUMBSNAP_KEY"),
        Hosting::Tixte => keyed!(tixte, "TIXTE_KEY"),
        Hosting::Uplio => keyed!(uplio, "UPLIO_KEY"),
        Hosting::Uploadcare => keyed!(uploadcare, "UPLOADCARE_KEY"),
        Hosting::Vgy => keyed!(vgy, "VGY_KEY"),
        Hosting::Zpic => keyed!(zpic, "ZPIC_KEY"),
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
