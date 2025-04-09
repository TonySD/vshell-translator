use log::{debug};
use memchr::memmem;

fn open_binary_file(filepath: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(filepath)
}

#[derive(Debug)]
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