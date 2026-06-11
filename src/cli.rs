use std::num::NonZeroUsize;
use std::path::PathBuf;

use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{ArgAction, Parser};

use crate::format::Format;
use crate::upload::Hosting;

/// Upload images via APIs.
#[derive(Parser, Debug)]
#[command(version, about, max_term_width = 120, styles = STYLES)]
pub struct Args {
    /// Image files to upload.
    #[arg(required = true)]
    pub images: Vec<PathBuf>,

    /// Hosting service to use.
    #[arg(short = 'H', long, default_value_t = Hosting::Imgur)]
    pub hosting: Hosting,

    /// Output format for the links.
    #[arg(short, long, default_value_t = Format::Plain)]
    pub format: Format,

    /// Create captioned thumbnails.
    #[arg(short, long)]
    pub thumbnail: bool,

    /// Send desktop notification on completion.
    #[arg(short, long)]
    pub notify: bool,

    /// Disable copying the result to the clipboard.
    #[arg(long)]
    pub no_clipboard: bool,

    /// Path to .env file. Overrides default config path.
    #[arg(long)]
    pub env_file: Option<PathBuf>,

    /// Max concurrent uploads.
    #[arg(short, long, default_value_t = const { NonZeroUsize::new(4).expect("nonzero") })]
    pub jobs: NonZeroUsize,

    /// Increase verbosity (-v for info, -vv for debug).
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());
