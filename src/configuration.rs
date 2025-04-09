use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[clap(short, long)]
    pub verbose: bool,

    #[clap(short, long)]
    pub chinese_chars: String,
}