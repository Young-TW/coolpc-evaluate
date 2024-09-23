mod request;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 呼叫從 request.rs 引入的 fetch_items 函數
    let items = request::fetch_items("https://coolpc.com.tw/evaluate.php")?;

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
