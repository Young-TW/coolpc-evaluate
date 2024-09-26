use crate::product::ProductInfo;

use regex::Regex;

pub fn analyse_ssd(item_name: &str) -> ProductInfo {
    // 正則表達式來提取關鍵資訊
    let brand_regex = Regex::new(r"(三星|威剛|金士頓|西部數據|海康威視|Crucial)").unwrap();
    let capacity_regex = Regex::new(r"(\d+GB|\d+TB)").unwrap();
    let interface_regex = Regex::new(r"(NVMe|SATA)").unwrap();
    let form_factor_regex = Regex::new(r"(M\.2|2\.5\|PCIe)").unwrap();
    let read_speed_regex = Regex::new(r"讀取速度:? ?(\d+MB/s)").unwrap();
    let write_speed_regex = Regex::new(r"寫入速度:? ?(\d+MB/s)").unwrap();

    // 提取品牌
    let brand = brand_regex.find(item_name).map(|m| m.as_str().to_string());

    // 提取容量
    let capacity = capacity_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 提取介面(NVMe、SATA)
    let interface = interface_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 提取規格(M.2、2.5"等)
    let form_factor = form_factor_regex
        .find(item_name)
        .map(|m| m.as_str().to_string());

    // 提取讀取速度
    let read_speed = read_speed_regex
        .captures(item_name)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

    // 提取寫入速度
    let write_speed = write_speed_regex
        .captures(item_name)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()));

    // 組合提取的規格信息
    let specs = format!(
        "{} {} {} 讀取速度: {}MB/s 寫入速度: {}MB/s",
        capacity.as_deref().unwrap_or("未知容量"),
        interface.as_deref().unwrap_or("未知介面"),
        form_factor.as_deref().unwrap_or("未知規格"),
        read_speed.unwrap_or_else(|| "未知".to_string()),
        write_speed.unwrap_or_else(|| "未知".to_string())
    )
    .trim()
    .to_string();

    // 返回解析結果
    ProductInfo {
        category: "SSD".to_string(),
        brand,
        model: Some(item_name.to_string()),
        specs: Some(specs), // 組合好的規格
        price: None,
        specific_model: None,
    }
}
