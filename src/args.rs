use std::path::PathBuf;

use crate::date_utils;
use clap::Parser;
use time::Date;

/// A bulk downloader for NYT XWord puzzle PDFs
#[derive(Parser)]
pub struct Args {
    /// The date to start downloading from (default today UTC)
    ///
    /// note: must be >= 2011-04-01
    #[arg(value_parser = date_utils::parse_date)]
    pub start: Option<Date>,
    /// The date to download up to (default today UTC)
    ///
    /// note: must be >= `start`
    #[arg(value_parser = date_utils::parse_date)]
    pub end: Option<Date>,
    /// The NYT-S token for authorizing your subscription (see readme)
    ///
    /// if not provided will attempt to find it on disk
    #[arg(short, long)]
    pub token: Option<String>,
    /// The destination directory (default to PWD)
    #[arg(short, long)]
    pub dest: Option<PathBuf>,
    /// If the download should skip Sunday puzzles
    #[arg(short, long)]
    pub skip_sunday: bool,
}

impl Args {
    pub fn get_dest(dest: Option<PathBuf>) -> PathBuf {
        if let Some(dest) = dest {
            return dest.clone();
        }
        match std::env::current_dir() {
            Ok(dest) => dest,
            Err(e) => {
                eprintln!("no dest provided and failed to look up PWD: {e}");
                std::process::exit(1);
            }
        }
    }
}
