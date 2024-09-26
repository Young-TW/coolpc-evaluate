use crate::product::ProductInfo;

use regex::Regex;

pub fn analyse_cpu(item_name: &str) -> ProductInfo {
    // 使用正則表達式來解析型號中的 CPU 規格
    let brand_regex = Regex::new(r"(Intel|AMD)").unwrap();
    let cores_threads_regex = Regex::new(r"(\d+)核/(\d+)緒").unwrap();
    let frequency_regex = Regex::new(r"(\d+(\.\d+)?G(?:Hz)?)").unwrap();
    let tdp_regex = Regex::new(r"(\d+)W").unwrap();
    let igpu_regex = Regex::new(r"(內顯|無內顯)").unwrap();

    let model_regex =
        Regex::new(r"(i\d+[-\s]\d+K?|R\d+[-\s]\d+X?|Ryzen \d+[-\s]\d+X?|Xeon \w+)").unwrap();

    // 提取 CPU 的品牌（Intel 或 AMD）
    let brand = brand_regex.find(item_name).map(|m| m.as_str().to_string());

    // 提取 CPU 的核心數和執行緒數
    let cores_threads = cores_threads_regex
        .captures(item_name)
        .map(|caps| format!("{}核/{}緒", &caps[1], &caps[2]));

    // 提取 CPU 的頻率並保證使用 GHz 格式
    let frequency = frequency_regex.captures(item_name).map(|caps| {
        let freq = &caps[1];
        if freq.ends_with("G") && !freq.ends_with("GHz") {
            format!("{}GHz", freq.trim_end_matches('G'))
        } else {
            freq.to_string()
        }
    });

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
        "{}{}{}{}",
        cores_threads
            .as_ref()
            .map_or("未知核心/緒數".to_string(), |v| v.to_string()),
        frequency
            .as_ref()
            .map_or(" 未知頻率".to_string(), |v| format!(" {}", v)),
        tdp.as_ref()
            .map_or(" 未知 TDP".to_string(), |v| format!(" {}", v)),
        igpu.as_ref()
            .map_or(" 未知內顯狀態".to_string(), |v| format!(" {}", v))
    )
    .trim()
    .to_string();

    ProductInfo {
        category: "CPU".to_string(),
        brand,
        model: Some(item_name.to_string()),
        specs: Some(specs),
        price: None,
        specific_model, // 新增具體的型號欄位
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyse_cpu() {
        let item_name = "AMD R9 9900X代理盒裝【12核/24緒】4.4G(↑5.6G)120W/具RDNA內顯, $15750 ◆ ★";
        let expected = ProductInfo {
            category: "CPU".to_string(),
            brand: Some("AMD".to_string()),
            model: Some(
                "AMD R9 9900X代理盒裝【12核/24緒】4.4G(↑5.6G)120W/具RDNA內顯, $15750 ◆ ★"
                    .to_string(),
            ),
            specs: Some("12核/24緒 4.4GHz 120W 內顯".to_string()), // 修正為 GHz 格式
            price: None,
            specific_model: Some("R9 9900X".to_string()),
        };

        let result = analyse_cpu(item_name);

        assert_eq!(result.category, expected.category);
        assert_eq!(result.brand, expected.brand);
        assert_eq!(result.model, expected.model);
        assert_eq!(result.specs, expected.specs);
        assert_eq!(result.specific_model, expected.specific_model);
    }

    #[test]
    fn test_analyse_cpu_intel() {
        let item_name = "Intel i9 11900K代理盒裝【8核/16緒】3.5G(↑5.3G)125W/無內顯, $15990";
        let expected = ProductInfo {
            category: "CPU".to_string(),
            brand: Some("Intel".to_string()),
            model: Some(
                "Intel i9 11900K代理盒裝【8核/16緒】3.5G(↑5.3G)125W/無內顯, $15990".to_string(),
            ),
            specs: Some("8核/16緒 3.5GHz 125W 無內顯".to_string()), // 修正為 GHz 格式
            price: None,
            specific_model: Some("i9 11900K".to_string()),
        };

        let result = analyse_cpu(item_name);

        assert_eq!(result.category, expected.category);
        assert_eq!(result.brand, expected.brand);
        assert_eq!(result.model, expected.model);
        assert_eq!(result.specs, expected.specs);
        assert_eq!(result.specific_model, expected.specific_model);
    }

    #[test]
    fn test_analyse_cpu_missing_info() {
        let item_name = "AMD R7 3700X代理盒裝";
        let expected = ProductInfo {
            category: "CPU".to_string(),
            brand: Some("AMD".to_string()),
            model: Some("AMD R7 3700X代理盒裝".to_string()),
            specs: Some("未知核心/緒數 未知頻率 未知 TDP 未知內顯狀態".to_string()),
            price: None,
            specific_model: Some("R7 3700X".to_string()), // 確保提取具體型號
        };

        let result = analyse_cpu(item_name);

        assert_eq!(result.category, expected.category);
        assert_eq!(result.brand, expected.brand);
        assert_eq!(result.model, expected.model);
        assert_eq!(result.specs, expected.specs);
        assert_eq!(result.specific_model, expected.specific_model);
    }
}
