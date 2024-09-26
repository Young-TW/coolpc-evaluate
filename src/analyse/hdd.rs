use crate::product::ProductInfo;

use regex::Regex;

pub fn analyse_hdd(item_name: &str) -> ProductInfo {
    // 正則表達式來提取關鍵資訊
    let brand_regex = Regex::new(r"(西部數據|希捷|東芝|HGST)").unwrap();
    let capacity_regex = Regex::new(r"(\d+GB|\d+TB)").unwrap();
    let form_factor_regex = Regex::new(r"(3\.5|2\.5)吋").unwrap();
    let rpm_regex = Regex::new(r"(\d{4,5})RPM").unwrap(); // 提取轉速，如 5400RPM 或 7200RPM
    let cache_regex = Regex::new(r"快取(?:容量)?[:：]? ?(\d+MB)").unwrap();

    // 提取品牌
    let brand = brand_regex.find(item_name).map(|m| m.as_str().to_string());

    // 提取容量
    let capacity = capacity_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 提取硬碟規格（2.5 吋或 3.5 吋）
    let form_factor = form_factor_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 提取轉速（如 5400RPM 或 7200RPM）
    let rpm = rpm_regex.find(item_name).map(|m| m.as_str().to_string());

    // 提取快取容量
    let cache = cache_regex
        .captures(item_name)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

    // 組合提取的規格信息
    let specs = format!(
        "{} {} {} RPM 快取: {}MB",
        capacity.as_deref().unwrap_or("未知容量"),
        form_factor.as_deref().unwrap_or("未知規格"),
        rpm.unwrap_or_else(|| "未知轉速".to_string()),
        cache.unwrap_or_else(|| "未知快取".to_string()),
    )
    .trim()
    .to_string();

    // 返回解析結果
    ProductInfo {
        category: "HDD".to_string(),
        brand,
        model: Some(item_name.to_string()),
        specs: Some(specs), // 組合好的規格
        price: None,
        specific_model: None,
    }
}
