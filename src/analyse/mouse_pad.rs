use crate::product::ProductInfo;

pub fn analyse_mouse_pad(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Mouse".to_string(),
        brand: Some("Mouse Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Mouse specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
