use crate::product::ProductInfo;

pub fn analyse_power_supply(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Power Supply".to_string(),
        brand: Some("Power Supply Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Power Supply specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
