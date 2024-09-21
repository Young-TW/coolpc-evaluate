use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://coolpc.com.tw/evaluate.php";
    let response = get(url)?;

    if response.status().is_success() {
        let body = response.text()?;
        let document = Html::parse_document(&body);

        let select_selector = Selector::parse("select").unwrap();
        let option_selector = Selector::parse("option").unwrap();

        for select in document.select(&select_selector) {
            if let Some(name) = select.value().attr("name") {
                println!("選單名稱: {}", name);

                for option in select.select(&option_selector) {
                    let option_text = option.text().collect::<Vec<_>>().join("");
                    println!("選項: {}", option_text);
                }
            }
        }
    } else {
        println!("無法取得網頁內容");
    }

    Ok(())
}
