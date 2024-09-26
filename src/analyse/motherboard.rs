use crate::product::ProductInfo;

use regex::Regex;

pub fn analyse_motherboard(item_name: &str) -> ProductInfo {
    // 正則表達式來提取關鍵資訊
    let brand_regex = Regex::new(r"(微星|華碩|技嘉|華擎)").unwrap();
    let specific_model_regex = Regex::new(r"((Z\d+|B\d+|X\d+|H\d+|A\d+)\w+(-[A-Z]+)?)").unwrap();
    let form_factor_regex = Regex::new(r"(ATX|M-ATX|E-ATX|Mini-ITX)").unwrap();
    let lan_regex = Regex::new(r"(Intel|Realtek|Marvell) \d+Gb").unwrap();
    let wifi_regex = Regex::new(r"(Wi-Fi \d+|無線)").unwrap();
    let power_phases_regex = Regex::new(r"(\d+\+\d+相電源)").unwrap();

    // 提取品牌
    let brand = brand_regex.find(item_name).map(|m| m.as_str().to_string());

    // 提取具體型號
    let specific_model = specific_model_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 提取主機板的尺寸（ATX、M-ATX等）
    let form_factor = form_factor_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 提取主機板的網卡信息（LAN）
    let lan = lan_regex.find(item_name).map(|m| m.as_str().to_string());

    // 提取是否有 Wi-Fi 支持
    let wifi = wifi_regex.find(item_name).map(|m| m.as_str().to_string());

    // 提取供電相數
    let power_phases = power_phases_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 組合提取的規格信息
    let specs = format!(
        "{} {} {} {} {}",
        form_factor.as_deref().unwrap_or("未知尺寸"),
        lan.as_deref().unwrap_or("未知網卡"),
        wifi.as_deref().unwrap_or("未知 Wi-Fi"),
        power_phases.as_deref().unwrap_or("未知供電相數"),
        specific_model.as_deref().unwrap_or("未知型號")
    )
    .trim()
    .to_string();

    // 返回解析結果
    ProductInfo {
        category: "Motherboard".to_string(),
        brand,
        model: Some(item_name.to_string()),
        specs: Some(specs), // 組合好的規格
        price: None,
        specific_model, // 具體型號
    }
}
