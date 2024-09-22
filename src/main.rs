use reqwest::blocking::get;
use scraper::{Html, Selector};
use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://coolpc.com.tw/evaluate.php";
    let response = get(url)?;

    if response.status().is_success() {
        let body = response.text()?;
        let document = Html::parse_document(&body);

        let select_selector = Selector::parse("select").unwrap();
        let option_selector = Selector::parse("option").unwrap();

        let price_regex = Regex::new(r"\$([0-9,]+)").unwrap();

        let mut items: Vec<(String, String)> = Vec::new();

        for select in document.select(&select_selector) {
            if let Some(_name) = select.value().attr("name") {
                for option in select.select(&option_selector) {
                    let option_text = option.text().collect::<Vec<_>>().join("");

                    if let Some(captures) = price_regex.captures(&option_text) {
                        let price = captures.get(1).unwrap().as_str().to_string();
                        let item_name = option_text.split(',').next().unwrap_or("").trim().to_string();
                        items.push((item_name, price));
                    }
                }

                let max_length = items.iter().map(|(name, _)| name.len()).max().unwrap_or(0);

                for (item_name, price) in &items {
                    let padding = max_length.saturating_sub(item_name.len());
                    println!("{}{}價格: {}", item_name, " ".repeat(padding + 2), price);
                }

                items.clear();
            }
        }
    } else {
        println!("無法取得網頁內容");
    }

    Ok(())
}
