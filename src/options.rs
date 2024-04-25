use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short)]
    pub url: String,
    #[arg(short)]
    pub delay: u64
}
