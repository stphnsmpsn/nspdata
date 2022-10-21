use crate::args::{Action, Args};
use clap::Parser;

pub mod args;
pub mod convert;
pub mod download;
pub mod error;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let args: Args = Args::parse();

    match args.action {
        Action::Convert => convert::convert(&args)?,
        Action::Download => download::download(&args).await?,
        Action::RunServer => server::run().await?,
    }

    Ok(())
}
