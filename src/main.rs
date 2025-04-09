use clap::Parser;
use configuration::Config;
use file::{generate_canary, patch_all_findings, FoundChineseBytes};
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

    if let Some(payload) = &config.patch_with {
        if payload.len() > chinese_bytes.len() {
            error!("Your payload is bigger ({} bytes) than available ({} bytes)", payload.len(), chinese_bytes.len());
            return ExitCode::FAILURE;
        }
    }

    match translate(config.chinese_chars.as_str(), "zh", "en").await {
        Ok(translate) => info!("English translate: {:?}", translate),
        Err(e) => warn!("Error getting english translate: {:?}", e)
    }
    match translate(config.chinese_chars.as_str(), "zh", "ru").await {
        Ok(translate) => info!("Russian translate: {:?}", translate),
        Err(e) => warn!("Error getting russian translate: {:?}", e)
    }

    let mut found_occurences = match file::find_all_occurences(&config.binary_file, &chinese_bytes) {
        Ok(found) => {
            info!("Found bytes in file:");
            for (i, found_occurence) in found.iter().enumerate() {
                println!("{} occurence: ", i + 1);
                println!("   - start:   0x{:x}", found_occurence.start);
                println!("   - end:     0x{:x}\n", found_occurence.end);
            }
            found
        },
        Err(e) => {
            error!("Got error while processing file: {:?}", e);
            return ExitCode::FAILURE;
        }
    };

    if config.iterate_every_occurence {
        found_occurences = match file::iterate_every_occurence(&config.binary_file, found_occurences, config.number_of_rows) {
            Ok(result) => result,
            Err(e) => {
                error!("Error while filtering found chars: {:?}", e);
                return ExitCode::FAILURE
            }
        };
    }

    match patch_all_findings(found_occurences, &config.binary_file, &config.output_file, config.patch_with) {
        Ok(result) => {
            info!("Patched file written to {}, written canaries: ", &config.output_file);
            for (i, canary) in result.iter().enumerate() {
                println!("{} canary: ", i + 1);
                println!("   - content: {}", canary.content);
                println!("   - start:   0x{:x}", canary.start);
                println!("   - end:     0x{:x}\n\n", canary.end);
            }
        },
        Err(e) => {
            error!("Error while writing file: {:?}", e);
            return ExitCode::FAILURE
        }
    };
    
    ExitCode::SUCCESS
}
