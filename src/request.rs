use regex::Regex;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

use crate::product::ProductInfo;

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

pub fn parse_items_by_onchange(body: &str) -> Result<Vec<ProductInfo>, Box<dyn Error>> {
    let document = Html::parse_document(body);

    // 定義用來抓取 select 和 option 的選擇器
    let select_selector = Selector::parse("select").unwrap();
    let option_selector = Selector::parse("option").unwrap();

    let onchange_regex = Regex::new(r"cnt\((\d+)\)").unwrap(); // 用來抓取 onchange 中的數字

    let mut items: Vec<ProductInfo> = Vec::new();

    for select in document.select(&select_selector) {
        // 取得 onchange 的數字，作為分類依據
        if let Some(onchange_attr) = select.value().attr("onchange") {
            if let Some(captures) = onchange_regex.captures(onchange_attr) {
                let category_id = captures.get(1).unwrap().as_str().to_string();

                for option in select.select(&option_selector) {
                    let option_text = option.text().collect::<Vec<_>>().join("");

                    // 將資料儲存到 ProductInfo 結構中
                    let product_info = ProductInfo {
                        category: category_id.clone(),    // 使用分類 ID 來儲存類別
                        brand: None,                      // 可以在其他地方解析品牌
                        model: Some(option_text.clone()), // 原始的 option 文本作為 model
                        specs: None,                      // 在其他地方解析規格
                        price: None,                      // 不處理價格，保留 None
                        specific_model: None,             // 在其他地方解析具體型號
                    };

                    // 將每個商品加入 items 列表
                    items.push(product_info);
                }
            }
        }
    }

    Ok(items)
}
