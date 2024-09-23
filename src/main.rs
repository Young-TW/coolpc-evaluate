use coolpc_evaluate::request;
use coolpc_evaluate::{analyse, cache};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://coolpc.com.tw/evaluate.php";
    let cache_file = "cache.html";

    let body = match cache::get_cache(cache_file) {
        Some(cached_body) => cached_body,
        None => {
            let fetched_body = request::fetch_page(url)?;
            cache::write_cache(cache_file, &fetched_body)?;
            fetched_body
        }
    };

    let items = request::parse_items_by_onchange(&body)?;

    if items.is_empty() {
        println!("無法取得任何項目或價格");
    } else {
        for item in &items {
            let analysed_item = analyse::analyse_item_by_category(item);
            println!("{:?}", analysed_item);
        }
    }

    Ok(())
}
