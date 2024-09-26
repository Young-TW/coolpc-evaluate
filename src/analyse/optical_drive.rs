use crate::product::ProductInfo;

pub fn analyse_optical_drive(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Optical Drive".to_string(),
        brand: Some("Optical Drive Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Optical Drive specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
