use crate::product::ProductInfo;

use regex::Regex;

pub fn analyse_cpu(item_name: &str) -> ProductInfo {
    // 使用正則表達式來解析型號中的 CPU 規格
    let brand_regex = Regex::new(r"(Intel|AMD)").unwrap();
    let cores_threads_regex = Regex::new(r"(\d+)核/(\d+)緒").unwrap();
    let frequency_regex = Regex::new(r"(\d+(\.\d+)?)GHz").unwrap();
    let tdp_regex = Regex::new(r"(\d+)W").unwrap();
    let igpu_regex = Regex::new(r"(內顯|無內顯)").unwrap();
    let model_regex = Regex::new(r"(i\d|R\d|Ryzen|Xeon)[\w\-]+").unwrap();

    // 提取 CPU 的品牌（Intel 或 AMD）
    let brand = brand_regex.find(item_name).map(|m| m.as_str().to_string());

    // 提取 CPU 的核心數和執行緒數
    let cores_threads = cores_threads_regex
        .captures(item_name)
        .map(|caps| format!("{}核/{}緒", &caps[1], &caps[2]));

    // 提取 CPU 的頻率
    let frequency = frequency_regex
        .captures(item_name)
        .map(|caps| format!("{}GHz", &caps[1]));

    // 提取 CPU 的 TDP (瓦特數)
    let tdp = tdp_regex
        .captures(item_name)
        .map(|caps| format!("{}W", &caps[1]));

    // 判斷是否包含內顯
    let igpu = igpu_regex.find(item_name).map(|m| m.as_str().to_string());

    // 提取具體型號
    let specific_model = model_regex.find(item_name).map(|m| m.as_str().to_string());

    // 將解析的規格組合為字串
    let specs = format!(
        "{} {} {} {}",
        cores_threads.as_ref().map_or("未知核心/緒數", |v| v),
        frequency.as_ref().map_or("未知頻率", |v| v),
        tdp.as_ref().map_or("未知 TDP", |v| v),
        igpu.as_ref().map_or("未知內顯狀態", |v| v)
    )
    .trim()
    .to_string();

    // 返回解析結果，不處理價格
    ProductInfo {
        category: "CPU".to_string(),
        brand,
        model: Some(item_name.to_string()),
        specs: Some(specs),
        price: None,
        specific_model, // 新增具體的型號欄位
    }
}
