use log::{debug};
use rand::RngCore;
use memchr::memmem;

pub fn open_binary_file(filepath: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(filepath)
}

pub fn hexdump_content(content: &Vec<u8>, addr: usize, window_size: usize) {
    let aligned_addr = addr - (addr % 16);
    let start = aligned_addr - window_size;
    let end = aligned_addr + window_size;

    hexdump::hexdump(&content[start..end]);
}

#[derive(Debug, Clone)]
pub struct FoundChineseBytes {
    pub start: usize,
    pub end: usize
}

pub fn find_all_occurences(filepath: &str, chinese_bytes: &[u8]) -> Result<Vec<FoundChineseBytes>, std::io::Error> {
    let file_content = open_binary_file(filepath)?;
    let chinese_bytes_size = chinese_bytes.len();

    debug!("Size of file: {}", file_content.len());
    let it = memmem::find_iter(&file_content, chinese_bytes);
    
    Ok(
        it
            .map(|index| FoundChineseBytes { start: index, end: index + chinese_bytes_size })
            .collect()
    )
}

pub fn iterate_every_occurence(filepath: &str, found_occurences: Vec<FoundChineseBytes>, number_of_rows: usize) -> Result<Vec<FoundChineseBytes>, std::io::Error> {
    let mut filtered_occurences: Vec<FoundChineseBytes> = Vec::new();
    let file_content = open_binary_file(filepath)?;
    
    for found_occurence in found_occurences.into_iter() {
        let mut buffer = String::new();
        hexdump_content(&file_content, found_occurence.start, number_of_rows * 16);
        while buffer.to_lowercase().trim() != "y" && buffer.to_lowercase().trim() != "n" {
            
            println!("Patch it? [Y/n]");
            std::io::stdin().read_line(&mut buffer).expect("Something wrong with stdin");
            debug!("Got: {:?}", buffer.to_lowercase().trim().as_bytes());
        }
        match buffer.to_ascii_lowercase().trim() {
            "y" => filtered_occurences.push(found_occurence),
            "n" => {},
            _ => unimplemented!("Only y/n can be passed through stdin")
        }
    };

    Ok(filtered_occurences)
}

#[derive(Debug)]
pub struct Canary {
    pub start: usize,
    pub end: usize,
    pub content: String
}

pub fn generate_canary(size: usize) -> String {
    let mut result = vec![0 as u8; (size / 2) + 1];
    rand::rng().fill_bytes(&mut result);
    let mut result: String = result.iter().map(|byte| format!("{:02x}", byte)).collect();
    result.truncate(size);
    result
}

pub fn patch_all_findings(found_occurences: Vec<FoundChineseBytes>, filename: &str, output_filename: &str, patch_with: Option<String>) -> Result<Vec<Canary>, std::io::Error> {
    let mut size: usize;
    let mut current_canary: String;
    let mut canaries: Vec<Canary> = Vec::new();
    let mut file_content = open_binary_file(filename)?;

    for occurence in found_occurences.into_iter() {
        size = occurence.end - occurence.start;
        current_canary = match &patch_with {
            Some(canary) => canary.clone(),
            None => {
                let result_canary;
                loop {
                    let candidate = generate_canary(size);
                    if !canaries.iter().any(|c| c.content == candidate) {
                        result_canary = candidate;
                        break;
                    }
                };
                result_canary
            }
        };

        for (i, canary_char) in current_canary.as_bytes().iter().enumerate() {
            file_content[occurence.start + i] = *canary_char;
        }

        canaries.push(
            Canary {
                start: occurence.start,
                end: occurence.end,
                content: current_canary
            }
        );
    }

    std::fs::write(output_filename, file_content)?;

    Ok(canaries)
}