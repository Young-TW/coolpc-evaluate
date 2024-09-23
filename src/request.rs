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

pub fn parse_items(body: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let document = Html::parse_document(body);

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
                    let item_name = option_text
                        .split(',')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    items.push((item_name, price));
                }
            }
        }
    }

    Ok(items)
}
