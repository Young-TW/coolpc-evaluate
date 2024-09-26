use crate::product::ProductInfo;

use regex::Regex;

pub fn analyse_ram(item_name: &str) -> ProductInfo {
    // 正則表達式來提取關鍵資訊
    let brand_regex = Regex::new(r"(金士頓|威剛|芝奇|海盜船|宇瞻)").unwrap();
    let capacity_regex = Regex::new(r"(\d+GB|\d+TB)").unwrap();
    let frequency_regex = Regex::new(r"(\d+MHz|\d+MT/s)").unwrap();
    let channel_regex = Regex::new(r"(單通道|雙通道|四通道)").unwrap();
    let ddr_regex = Regex::new(r"(DDR[1-5])").unwrap();

    // 提取品牌
    let brand = brand_regex.find(item_name).map(|m| m.as_str().to_string());

    // 提取容量
    let capacity = capacity_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 提取頻率
    let frequency = frequency_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 提取通道數（單通道、雙通道等）
    let channel = channel_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 提取 DDR 版本
    let ddr_version = ddr_regex.find(item_name).map(|m| m.as_str().to_string());

    // 組合提取的規格信息
    let specs = format!(
        "{} {} {} {}",
        capacity.as_deref().unwrap_or("未知容量"),
        frequency.as_deref().unwrap_or("未知頻率"),
        ddr_version.as_deref().unwrap_or("未知DDR版本"),
        channel.as_deref().unwrap_or("未知通道數")
    )
    .trim()
    .to_string();

    // 返回解析結果
    ProductInfo {
        category: "RAM".to_string(),
        brand,
        model: Some(item_name.to_string()),
        specs: Some(specs), // 組合好的規格
        price: None,
        specific_model: None,
    }
}
