use crate::product::ProductInfo;

pub fn analyse_peripherals(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Keyboard & Mouse".to_string(),
        brand: Some("Keyboard & Mouse Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Keyboard & Mouse specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
