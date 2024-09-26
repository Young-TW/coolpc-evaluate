use crate::product::ProductInfo;

pub fn analyse_monitor(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Monitor".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Monitor specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
