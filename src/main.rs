use clap::Parser;

mod args;
mod date_utils;
mod fetch;

use args::Args;

#[tokio::main]
async fn main() {
    let Args {
        start,
        end,
        token,
        dest,
        skip_sunday,
    } = Args::parse();
    fetch::execute(start, end, token, skip_sunday, Args::get_dest(dest)).await
}
