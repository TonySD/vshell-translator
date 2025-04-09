use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[clap(short, long)]
    pub verbose: bool,

    #[clap(short, long, required=true)]
    pub chinese_chars: String,

    #[clap(short, long, required=true)]
    pub binary_file: String,

    #[clap(short, long)]
    pub patch_with: Option<String>,

    #[clap(short, long, conflicts_with="patch_with")]
    pub random_canary: bool,

    #[clap(short, long, default_value="patched_binary.bin")]
    pub output_file: String,

    #[clap(short, long)]
    pub iterate_every_occurence: bool,

    #[clap(short, long, default_value="3")]
    pub number_of_rows: usize
}