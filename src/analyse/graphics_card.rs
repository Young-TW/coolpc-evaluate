use crate::product::ProductInfo;

pub fn analyse_graphics_card(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Graphics Card".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Graphics Card specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
