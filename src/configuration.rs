use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[clap(short, long)]
    pub verbose: bool,

    #[clap(short, long, required=true)]
    pub chinese_chars: String,

    #[clap(short, long, required=true)]
    pub binary_file: String,
}