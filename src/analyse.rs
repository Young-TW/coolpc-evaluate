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
        "4" => analyse_cpu(item_name, price),              // 處理器 CPU
        "5" => analyse_motherboard(item_name, price),      // 主機板 MB
        "6" => analyse_ram(item_name, price),              // 記憶體 RAM
        "7" => analyse_ssd(item_name, price),              // 固態硬碟 M.2 | SSD
        "8" => analyse_hdd(item_name, price),              // 傳統內接硬碟 HDD
        "9" => analyse_external_storage(item_name, price), // 外接硬碟 | 隨身碟 | 記憶卡
        "10" => analyse_cooling(item_name, price),         // 散熱器
        "11" => analyse_liquid_cooling(item_name, price),  // 封閉式 | 開放式水冷
        "12" => analyse_graphics_card(item_name, price),   // 顯示卡 VGA
        "13" => analyse_monitor(item_name, price),         // 螢幕 | 投影機 | 壁掛
        "14" => analyse_case(item_name, price),            // CASE 機殼
        "15" => analyse_power_supply(item_name, price),    // 電源供應器
        "16" => analyse_fan(item_name, price),             // 機殼風扇
        "17" => analyse_peripherals(item_name, price),     // 鍵盤 + 鼠 | 搖桿 | 桌 + 椅
        "18" => analyse_mouse_pad(item_name, price),       // 滑鼠 | 鼠墊 | 數位板
        "19" => analyse_network_equipment(item_name, price), // IP 分享器 | 網卡 | 網通設備
        "20" => analyse_nas_ipcam(item_name, price),       // 網路 NAS | 網路 IPCAM
        "21" => analyse_sound_card(item_name, price),      // 音效卡 | 電視卡
        "22" => analyse_speakers(item_name, price),        // 喇叭 | 耳機 | 麥克風
        "23" => analyse_optical_drive(item_name, price),   // 燒錄器 CD/DVD/BD
        "24" => analyse_usb_storage(item_name, price),     // USB 週邊 | 硬碟座 | 讀卡機
        "25" => analyse_dashcam(item_name, price),         // 行車紀錄器 | USB 視訊鏡頭
        "26" => analyse_ups_printer(item_name, price),     // UPS 不斷電 | 印表機 | 掃描
        "27" => analyse_raid_card(item_name, price),       // 介面擴充卡 | 專業 Raid 卡
        "28" => analyse_adapters(item_name, price),        // 轉接卡 | 傳輸線 | 轉換 | KVM
        "29" => analyse_os_software(item_name, price),     // OS + 應用軟體 | 禮物卡
        _ => analyse_generic(item_name, price),            // 其他類型使用通用分析器
    }
}

fn analyse_cpu(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "CPU".to_string(),
        brand: Some("Intel or AMD".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("CPU specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_motherboard(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Motherboard".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Motherboard specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_ram(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "RAM".to_string(),
        brand: Some("RAM Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("RAM specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_ssd(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "SSD".to_string(),
        brand: Some("SSD Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("SSD specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_hdd(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "HDD".to_string(),
        brand: Some("HDD Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("HDD specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_external_storage(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "External Storage".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Storage specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_cooling(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Cooling".to_string(),
        brand: Some("Cooling Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Cooling specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_liquid_cooling(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Liquid Cooling".to_string(),
        brand: Some("Cooling Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Liquid Cooling specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_graphics_card(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Graphics Card".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Graphics Card specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_monitor(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Monitor".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Monitor specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_case(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Case".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Case specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_power_supply(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Power Supply".to_string(),
        brand: Some("Power Supply Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Power Supply specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_fan(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Fan".to_string(),
        brand: Some("Fan Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Fan specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_keyboard(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Keyboard".to_string(),
        brand: Some("Keyboard Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Keyboard specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_peripherals(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Keyboard & Mouse".to_string(),
        brand: Some("Keyboard & Mouse Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Keyboard & Mouse specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_mouse_pad(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Mouse".to_string(),
        brand: Some("Mouse Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Mouse specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_network_equipment(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Network Equipment".to_string(),
        brand: Some("Network Equipment Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Network Equipment specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_nas_ipcam(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "NAS / IPCAM".to_string(),
        brand: Some("NAS / IPCAM Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("NAS / IPCAM specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_sound_card(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Sound Card / TV Card".to_string(),
        brand: Some("Sound Card Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Sound Card specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_speakers(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Speakers / Headphones / Microphones".to_string(),
        brand: Some("Speakers Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Speakers specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_optical_drive(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Optical Drive".to_string(),
        brand: Some("Optical Drive Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Optical Drive specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_usb_storage(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "USB Storage / Docking".to_string(),
        brand: Some("USB Storage Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("USB Storage specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_dashcam(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Dashcam / Webcam".to_string(),
        brand: Some("Dashcam Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Dashcam specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_ups_printer(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "UPS / Printer / Scanner".to_string(),
        brand: Some("UPS / Printer Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("UPS / Printer specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_raid_card(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "RAID Card / Expansion".to_string(),
        brand: Some("RAID Card Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("RAID Card specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_adapters(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Adapters / Cables / KVM".to_string(),
        brand: Some("Adapter Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Adapter specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_os_software(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "OS / Software / Gift Card".to_string(),
        brand: Some("OS / Software Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("OS / Software specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}

fn analyse_generic(item_name: &str, price: &str) -> ProductInfo {
    ProductInfo {
        category: "Generic".to_string(),
        brand: Some("Generic Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Generic specs".to_string()), // 可以根據需求進一步解析
        price: Some(price.to_string()),
    }
}
