use crate::product::ProductInfo;

pub fn analyse_external_storage(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "External Storage".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Storage specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
