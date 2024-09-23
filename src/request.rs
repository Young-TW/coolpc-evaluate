use regex::Regex;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

pub fn fetch_page(url: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;

    if response.status().is_success() {
        let body = response.text()?;
        Ok(body)
    } else {
        println!("無法取得網頁內容");
        Err("無法取得網頁內容".into())
    }
}

pub fn parse_items_by_onchange(
    body: &str,
) -> Result<Vec<(String, String, String)>, Box<dyn Error>> {
    let document = Html::parse_document(body);

    // 定義用來抓取 select 和 option 的選擇器
    let select_selector = Selector::parse("select").unwrap();
    let option_selector = Selector::parse("option").unwrap();

    let price_regex = Regex::new(r"\$([0-9,]+)").unwrap();
    let onchange_regex = Regex::new(r"cnt\((\d+)\)").unwrap(); // 用來抓取 onchange 中的數字

    let mut items: Vec<(String, String, String)> = Vec::new();

    for select in document.select(&select_selector) {
        // 取得 onchange 的數字，作為分類依據
        if let Some(onchange_attr) = select.value().attr("onchange") {
            if let Some(captures) = onchange_regex.captures(onchange_attr) {
                let category_id = captures.get(1).unwrap().as_str().to_string();

                for option in select.select(&option_selector) {
                    let option_text = option.text().collect::<Vec<_>>().join("");

                    if let Some(captures) = price_regex.captures(&option_text) {
                        let price = captures.get(1).unwrap().as_str().to_string();
                        let item_name = option_text
                            .split(',')
                            .next()
                            .unwrap_or("")
                            .trim()
                            .to_string();
                        items.push((category_id.clone(), item_name, price));
                    }
                }
            }
        }
    }

    Ok(items)
}
