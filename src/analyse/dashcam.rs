use crate::product::ProductInfo;

pub fn analyse_dashcam(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Dashcam / Webcam".to_string(),
        brand: Some("Dashcam Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Dashcam specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
