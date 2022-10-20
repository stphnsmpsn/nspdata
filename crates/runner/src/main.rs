use crate::args::{Action, Args};
use clap::Parser;

pub mod args;
pub mod convert;
pub mod download;
pub mod error;

const BIDGELY_BASE_URL: &'static str = "https://caapi.bidgely.com/v2.0";

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let args: Args = Args::parse();

    match args.action {
        Action::Convert => convert::convert(&args)?,
        Action::Download => download::download(BIDGELY_BASE_URL, &args).await?,
    }

    Ok(())
}
