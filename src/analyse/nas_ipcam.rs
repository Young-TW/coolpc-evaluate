use crate::product::ProductInfo;

pub fn analyse_nas_ipcam(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "NAS / IPCAM".to_string(),
        brand: Some("NAS / IPCAM Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("NAS / IPCAM specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
