mod cli;
mod format;
mod image;
mod upload;
mod util;

use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use futures::stream::{self, StreamExt};
use tracing::level_filters::LevelFilter;
use tracing::{error, info};
use tracing_subscriber::fmt;

use crate::cli::Args;
use crate::format::{Format, LinkPair, format_links};
use crate::upload::Hosting;

pub(crate) const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub(crate) const TIMEOUT: Duration = Duration::from_mins(2);

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    init_logging(args.verbose);

    let default_env_path = util::get_config_path();
    let env_path = args.env_file.as_ref().or(default_env_path.as_ref());

    if let Some(path) = env_path
        && path.exists()
        && let Err(e) = dotenvy::from_path(path)
    {
        error!("failed to load env file {}: {e}", path.display());
    }

    match run(&args).await {
        Ok(code) => code,
        Err(e) => {
            error!("{e:#}");
            ExitCode::FAILURE
        }
    }
}

/// Upload a single image (and its thumbnail if requested).
async fn upload_single(
    client: &reqwest::Client,
    hosting: Hosting,
    path: &Path,
    font: Option<&image::Font>,
) -> Option<LinkPair> {
    let data = tokio::fs::read(path)
        .await
        .inspect_err(|e| error!("failed to read {}: {e}", path.display()))
        .ok()?;

    let thumb_data = font
        .map(|f| image::make_thumbnail(&data, f))
        .transpose()
        .inspect_err(|e| error!("thumbnail failed for {}: {e:#}", path.display()))
        .ok()?;

    let img_url = upload::upload(client, hosting, data)
        .await
        .inspect_err(|e| error!("upload failed for {}: {e:#}", path.display()))
        .ok()?;
    info!("uploaded \"{}\" -> {img_url}", path.display());

    let thumb_url = if let Some(td) = thumb_data {
        Some(
            upload::upload(client, hosting, td)
                .await
                .inspect_err(|e| error!("thumbnail upload failed for {}: {e:#}", path.display()))
                .ok()?,
        )
    } else {
        None
    };

    Some((img_url, thumb_url))
}

async fn run(args: &Args) -> Result<ExitCode> {
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(TIMEOUT)
        .build()
        .context("failed to build HTTP client")?;

    let font = args.thumbnail.then(image::get_font);

    let results: Vec<_> = stream::iter(&args.images)
        .map(|path| upload_single(&client, args.hosting, path, font.as_ref()))
        .buffered(args.jobs.get())
        .collect()
        .await;

    let total = results.len();
    let links: Vec<LinkPair> = results.into_iter().flatten().collect();
    let has_errors = links.len() < total;

    if !links.is_empty() {
        // Thumbnail mode defaults to bbcode
        let fmt = if args.thumbnail && args.format == Format::Plain {
            Format::Bbcode
        } else {
            args.format
        };

        let output = format_links(&links, fmt);
        println!("{output}");

        if !args.no_clipboard {
            util::clipboard_copy(&output);
        }
        if args.notify {
            util::notify_send(&output);
        }
    }

    Ok(if has_errors {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

fn init_logging(verbose: u8) {
    let filter = match verbose {
        0 => LevelFilter::WARN,
        1 => LevelFilter::INFO,
        _ => LevelFilter::DEBUG,
    };

    fmt()
        .with_max_level(filter)
        .with_target(false)
        .with_writer(std::io::stderr)
        .init();
}
