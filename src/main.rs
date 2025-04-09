use clap::Parser;
use configuration::Config;
use std::process::ExitCode;
use rust_translate::translate;
use log::{info, warn, error};

pub mod configuration;
pub mod file;

#[tokio::main]
async fn main() -> ExitCode {
    colog::init();
    let config: Config = Config::parse();

    let chinese_bytes = config.chinese_chars.as_bytes();
    info!("Bytes of got chars: {:?}", 
        &chinese_bytes.iter()
            .map(|c| format!("{:x}", c))
            .collect::<Vec<String>>()
    );
    match translate(config.chinese_chars.as_str(), "zh", "en").await {
        Ok(translate) => info!("English translate: {:?}", translate),
        Err(e) => warn!("Error getting english translate: {:?}", e)
    }
    match translate(config.chinese_chars.as_str(), "zh", "ru").await {
        Ok(translate) => info!("Russian translate: {:?}", translate),
        Err(e) => warn!("Error getting russian translate: {:?}", e)
    }

    let found_occurences = match file::find_all_occurences(&config.binary_file, &chinese_bytes) {
        Ok(found) => {
            info!("Found bytes in file: {:?}", found);
            found
        },
        Err(e) => {
            error!("Got error while processing file: {:?}", e);
            return ExitCode::FAILURE;
        }
    };
    
    ExitCode::SUCCESS
}
