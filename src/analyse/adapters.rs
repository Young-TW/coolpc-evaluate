use crate::product::ProductInfo;

pub fn analyse_adapters(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Adapters / Cables / KVM".to_string(),
        brand: Some("Adapter Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Adapter specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
