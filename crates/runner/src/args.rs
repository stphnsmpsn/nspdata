#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Action {
    Convert,
    Download,
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum)]
    pub action: Action,
    #[arg(short, long)]
    pub uid: Option<String>,
    #[arg(short, long)]
    pub input_filename: Option<String>,
    #[arg(short, long)]
    pub output_filename: Option<String>,
    #[arg(short, long)]
    pub start: Option<u64>,
    #[arg(short, long)]
    pub end: Option<u64>,
}
