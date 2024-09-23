use std::fs::{self, File};
use std::io::{self, Write};
use std::time::Duration;

const CACHE_EXPIRATION: Duration = Duration::from_secs(60 * 60); // 1 小時有效期限

// 檢查快取是否存在且有效
pub fn get_cache(cache_file: &str) -> Option<String> {
    if let Ok(metadata) = fs::metadata(cache_file) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = modified.elapsed() {
                // 如果快取未過期，讀取檔案內容
                if elapsed < CACHE_EXPIRATION {
                    if let Ok(contents) = fs::read_to_string(cache_file) {
                        return Some(contents);
                    }
                }
            }
        }
    }
    None
}

// 寫入快取
pub fn write_cache(cache_file: &str, data: &str) -> io::Result<()> {
    let mut file = File::create(cache_file)?;
    file.write_all(data.as_bytes())?;
    file.flush()?; // 確保數據已寫入硬碟
    Ok(())
}
