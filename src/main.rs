use clap::Parser;
use configuration::Config;
use rust_translate::translate;

pub mod configuration;

#[tokio::main]
async fn main() {
    let config: Config = Config::parse();
    println!("{:?}", config.chinese_chars.clone()
        .as_bytes().iter()
        .map(|c| format!("{:x}", c))
        .collect::<Vec<String>>()
    );

    println!("{:?}", translate(config.chinese_chars.as_str(), "zh", "en").await);
}
