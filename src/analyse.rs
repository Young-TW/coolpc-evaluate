pub mod adapters;
pub mod case;
pub mod cooling;
pub mod cpu;
pub mod dashcam;
pub mod external_storage;
pub mod fan;
pub mod generic;
pub mod graphics_card;
pub mod hdd;
pub mod liquid_cooling;
pub mod monitor;
pub mod motherboard;
pub mod mouse_pad;
pub mod nas_ipcam;
pub mod network_equipment;
pub mod optical_drive;
pub mod os_software;
pub mod peripherals;
pub mod power_supply;
pub mod raid_card;
pub mod ram;
pub mod sound_card;
pub mod speakers;
pub mod ssd;
pub mod ups_printer;
pub mod usb_storage;

use crate::product::ProductInfo;

use crate::analyse::adapters::analyse_adapters;
use crate::analyse::case::analyse_case;
use crate::analyse::cooling::analyse_cooling;
use crate::analyse::cpu::analyse_cpu;
use crate::analyse::dashcam::analyse_dashcam;
use crate::analyse::external_storage::analyse_external_storage;
use crate::analyse::fan::analyse_fan;
use crate::analyse::generic::analyse_generic;
use crate::analyse::graphics_card::analyse_graphics_card;
use crate::analyse::hdd::analyse_hdd;
use crate::analyse::liquid_cooling::analyse_liquid_cooling;
use crate::analyse::monitor::analyse_monitor;
use crate::analyse::motherboard::analyse_motherboard;
use crate::analyse::mouse_pad::analyse_mouse_pad;
use crate::analyse::nas_ipcam::analyse_nas_ipcam;
use crate::analyse::network_equipment::analyse_network_equipment;
use crate::analyse::optical_drive::analyse_optical_drive;
use crate::analyse::os_software::analyse_os_software;
use crate::analyse::peripherals::analyse_peripherals;
use crate::analyse::power_supply::analyse_power_supply;
use crate::analyse::raid_card::analyse_raid_card;
use crate::analyse::ram::analyse_ram;
use crate::analyse::sound_card::analyse_sound_card;
use crate::analyse::speakers::analyse_speakers;
use crate::analyse::ssd::analyse_ssd;
use crate::analyse::ups_printer::analyse_ups_printer;
use crate::analyse::usb_storage::analyse_usb_storage;

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
