use crate::product::ProductInfo;

pub fn analyse_item_by_category(item: &ProductInfo) -> ProductInfo {
    let default_str = "".to_string();
    let model_str = item.model.as_ref().unwrap_or(&default_str);

    let mut product_info = match item.category.as_str() {
        "4" => analyse_cpu(&item.model.clone().unwrap_or_default()), // 處理器 CPU
        "5" => analyse_motherboard(&item.model.clone().unwrap_or_default()), // 主機板 MB
        "6" => analyse_ram(&item.model.clone().unwrap_or_default()), // 記憶體 RAM
        "7" => analyse_ssd(&item.model.clone().unwrap_or_default()), // 固態硬碟 M.2 | SSD
        "8" => analyse_hdd(&item.model.clone().unwrap_or_default()), // 傳統內接硬碟 HDD
        "9" => analyse_external_storage(&item.model.clone().unwrap_or_default()), // 外接硬碟 | 隨身碟 | 記憶卡
        "10" => analyse_cooling(&item.model.clone().unwrap_or_default()),         // 散熱器
        "11" => analyse_liquid_cooling(&item.model.clone().unwrap_or_default()), // 封閉式 | 開放式水冷
        "12" => analyse_graphics_card(&item.model.clone().unwrap_or_default()),  // 顯示卡 VGA
        "13" => analyse_monitor(&item.model.clone().unwrap_or_default()), // 螢幕 | 投影機 | 壁掛
        "14" => analyse_case(&item.model.clone().unwrap_or_default()),    // CASE 機殼
        "15" => analyse_power_supply(&item.model.clone().unwrap_or_default()), // 電源供應器
        "16" => analyse_fan(&item.model.clone().unwrap_or_default()),     // 機殼風扇
        "17" => analyse_peripherals(&item.model.clone().unwrap_or_default()), // 鍵盤 + 鼠 | 搖桿 | 桌 + 椅
        "18" => analyse_mouse_pad(&item.model.clone().unwrap_or_default()), // 滑鼠 | 鼠墊 | 數位板
        "19" => analyse_network_equipment(&item.model.clone().unwrap_or_default()), // IP 分享器 | 網卡 | 網通設備
        "20" => analyse_nas_ipcam(&item.model.clone().unwrap_or_default()), // 網路 NAS | 網路 IPCAM
        "21" => analyse_sound_card(&item.model.clone().unwrap_or_default()), // 音效卡 | 電視卡
        "22" => analyse_speakers(&item.model.clone().unwrap_or_default()),  // 喇叭 | 耳機 | 麥克風
        "23" => analyse_optical_drive(&item.model.clone().unwrap_or_default()), // 燒錄器 CD/DVD/BD
        "24" => analyse_usb_storage(&item.model.clone().unwrap_or_default()), // USB 週邊 | 硬碟座 | 讀卡機
        "25" => analyse_dashcam(&item.model.clone().unwrap_or_default()), // 行車紀錄器 | USB 視訊鏡頭
        "26" => analyse_ups_printer(&item.model.clone().unwrap_or_default()), // UPS 不斷電 | 印表機 | 掃描
        "27" => analyse_raid_card(&item.model.clone().unwrap_or_default()), // 介面擴充卡 | 專業 Raid 卡
        "28" => analyse_adapters(&item.model.clone().unwrap_or_default()), // 轉接卡 | 傳輸線 | 轉換 | KVM
        "29" => analyse_os_software(&item.model.clone().unwrap_or_default()), // OS + 應用軟體 | 禮物卡
        _ => analyse_generic(&item.model.clone().unwrap_or_default()), // 其他類型使用通用分析器
    };

    // 使用可變引用來避免所有權轉移
    set_price(&mut product_info, model_str);

    product_info
}

use regex::Regex;

fn set_price(product_info: &mut ProductInfo, item_name: &str) {
    let price = get_price(item_name);
    product_info.price = price;
}

fn get_price(item_name: &str) -> Option<String> {
    // 使用正則表達式來解析價格
    let price_regex = Regex::new(r"\$([0-9,]+)").unwrap();
    price_regex
        .captures(item_name)
        .map(|caps| caps[1].to_string())
}

fn analyse_cpu(item_name: &str) -> ProductInfo {
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

fn analyse_motherboard(item_name: &str) -> ProductInfo {
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

fn analyse_ram(item_name: &str) -> ProductInfo {
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

fn analyse_ssd(item_name: &str) -> ProductInfo {
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

fn analyse_hdd(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "HDD".to_string(),
        brand: Some("HDD Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("HDD specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_external_storage(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "External Storage".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Storage specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_cooling(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Cooling".to_string(),
        brand: Some("Cooling Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Cooling specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_liquid_cooling(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Liquid Cooling".to_string(),
        brand: Some("Cooling Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Liquid Cooling specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_graphics_card(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Graphics Card".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Graphics Card specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_monitor(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Monitor".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Monitor specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_case(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Case".to_string(),
        brand: Some("Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Case specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_power_supply(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Power Supply".to_string(),
        brand: Some("Power Supply Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Power Supply specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_fan(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Fan".to_string(),
        brand: Some("Fan Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Fan specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_peripherals(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Keyboard & Mouse".to_string(),
        brand: Some("Keyboard & Mouse Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Keyboard & Mouse specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_mouse_pad(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Mouse".to_string(),
        brand: Some("Mouse Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Mouse specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_network_equipment(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Network Equipment".to_string(),
        brand: Some("Network Equipment Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Network Equipment specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_nas_ipcam(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "NAS / IPCAM".to_string(),
        brand: Some("NAS / IPCAM Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("NAS / IPCAM specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_sound_card(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Sound Card / TV Card".to_string(),
        brand: Some("Sound Card Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Sound Card specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_speakers(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Speakers / Headphones / Microphones".to_string(),
        brand: Some("Speakers Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Speakers specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_optical_drive(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Optical Drive".to_string(),
        brand: Some("Optical Drive Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Optical Drive specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_usb_storage(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "USB Storage / Docking".to_string(),
        brand: Some("USB Storage Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("USB Storage specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_dashcam(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Dashcam / Webcam".to_string(),
        brand: Some("Dashcam Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Dashcam specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_ups_printer(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "UPS / Printer / Scanner".to_string(),
        brand: Some("UPS / Printer Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("UPS / Printer specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_raid_card(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "RAID Card / Expansion".to_string(),
        brand: Some("RAID Card Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("RAID Card specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_adapters(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Adapters / Cables / KVM".to_string(),
        brand: Some("Adapter Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Adapter specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_os_software(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "OS / Software / Gift Card".to_string(),
        brand: Some("OS / Software Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("OS / Software specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}

fn analyse_generic(item_name: &str) -> ProductInfo {
    ProductInfo {
        category: "Generic".to_string(),
        brand: Some("Generic Brand".to_string()), // 可以根據需求進一步解析
        model: Some(item_name.to_string()),
        specs: Some("Generic specs".to_string()), // 可以根據需求進一步解析
        price: None,
        specific_model: None,
    }
}
