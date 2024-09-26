use crate::product::ProductInfo;

pub fn analyse_generic(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Generic".to_string(),
        brand: Some("Generic Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Generic specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
