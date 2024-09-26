use crate::product::ProductInfo;

pub fn analyse_fan(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Fan".to_string(),
        brand: Some("Fan Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Fan specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
