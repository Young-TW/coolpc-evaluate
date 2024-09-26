use crate::product::ProductInfo;

pub fn analyse_ups_printer(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "UPS / Printer / Scanner".to_string(),
        brand: Some("UPS / Printer Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("UPS / Printer specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
