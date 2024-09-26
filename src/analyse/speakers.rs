use crate::product::ProductInfo;

pub fn analyse_speakers(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Speakers / Headphones / Microphones".to_string(),
        brand: Some("Speakers Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Speakers specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
