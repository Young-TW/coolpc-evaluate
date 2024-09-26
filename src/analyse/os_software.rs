use crate::product::ProductInfo;

pub fn analyse_os_software(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "OS / Software / Gift Card".to_string(),
        brand: Some("OS / Software Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("OS / Software specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
