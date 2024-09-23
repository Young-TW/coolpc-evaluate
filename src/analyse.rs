#[derive(Debug)]
pub struct ProductInfo {
    pub category: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub specs: Option<String>,
    pub price: Option<String>,
}

// 根據不同的 category_id 來分類解析
pub fn analyse_item_by_category(category_id: &str, item_name: &str, price: &str) -> ProductInfo {
    match category_id {
        "4" => analyse_cpu(item_name, price),         // 假設 4 對應 CPU
        "5" => analyse_motherboard(item_name, price), // 假設 5 對應主機板
        "6" => analyse_ram(item_name, price),         // 假設 6 對應記憶體
        _ => analyse_generic(item_name, price),       // 其他類型使用通用分析器
    }
}

// CPU 分析器
fn analyse_cpu(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "CPU".to_string(),
        brand: Some("Intel or AMD".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("CPU specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

// 主機板分析器
fn analyse_motherboard(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Motherboard".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Motherboard specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

// 記憶體分析器
fn analyse_ram(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "RAM".to_string(),
        brand: Some("RAM Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("RAM specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

// 通用分析器 (其他類型的零件)
fn analyse_generic(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Other".to_string(),
        brand: None,
        model: Some(item_name.to_string()),
        specs: None,
        price: Some(price.to_string()),
    }
}
