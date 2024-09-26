use crate::product::ProductInfo;

pub fn analyse_sound_card(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Sound Card / TV Card".to_string(),
        brand: Some("Sound Card Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Sound Card specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
