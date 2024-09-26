use crate::product::ProductInfo;

pub fn analyse_liquid_cooling(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Liquid Cooling".to_string(),
        brand: Some("Cooling Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Liquid Cooling specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
