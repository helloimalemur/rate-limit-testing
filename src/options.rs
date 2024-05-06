use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short)]
    pub url: String,
    #[arg(short)]
    pub delay: u64,
    #[arg(short)]
    pub post_data: Option<String>,
    #[arg(short)]
    pub array_headers: Option<String>,
}
