mod cache;
mod request;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://coolpc.com.tw/evaluate.php";
    let cache_file = "cache.html";

    // 嘗試從 cache.rs 取得快取的內容
    let body = match cache::get_cache(cache_file) {
        Some(cached_body) => cached_body,
        None => {
            // 如果快取不存在或無效，則發送 HTTP 請求
            let fetched_body = request::fetch_page(url)?;
            // 將取得的內容快取下來
            cache::write_cache(cache_file, &fetched_body)?;
            fetched_body
        }
    };

    // 解析網頁並取得項目和價格
    let items = request::parse_items(&body)?;

    if items.is_empty() {
        println!("無法取得任何項目或價格");
    } else {
        let max_length = items.iter().map(|(name, _)| name.len()).max().unwrap_or(0);

        for (item_name, price) in &items {
            let padding = max_length.saturating_sub(item_name.len());
            println!("{}{}價格: {}", item_name, " ".repeat(padding + 2), price);
        }
    }

    Ok(())
}
