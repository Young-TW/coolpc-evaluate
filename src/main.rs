use clap::{Arg, Command};
use coolpc_evaluate::analyse::analyse_item_by_category;
use coolpc_evaluate::cache;
use coolpc_evaluate::product::ProductInfo;
use coolpc_evaluate::request;

use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, FileChooserDialog, Label, ListBox, ListBoxRow,
    ResponseType, ScrolledWindow,
};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 使用 clap 定義 CLI 參數
    let matches = Command::new("CoolPC Evaluate")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Fetches product info from CoolPC and displays it in GUI or CLI")
        .arg(
            Arg::new("cli")
                .long("cli")
                .help("Run in CLI mode, print all ProductInfo")
                .action(clap::ArgAction::SetTrue), // 將 --cli 設為布爾標志
        )
        .get_matches();

    let url = "https://coolpc.com.tw/evaluate.php";
    let cache_file = "cache.html";

    let body = match cache::get_cache(cache_file) {
        Some(cached_body) => cached_body,
        None => {
            let fetched_body = request::fetch_page(url)?;
            cache::write_cache(cache_file, &fetched_body)?;
            fetched_body
        }
    };

    let items = request::parse_items_by_onchange(&body)?;

    // 檢查是否使用 --cli flag
    if matches.get_flag("cli") {
        // CLI 模式，列印所有 ProductInfo
        for item in &items {
            analyse_item_by_category(item);
            println!(
                "Model: {}\nPrice: {}\n",
                item.model.as_ref().unwrap_or(&"N/A".to_string()),
                item.price.as_ref().unwrap_or(&"N/A".to_string())
            );
        }
    } else {
        // GUI 模式
        let app = Application::builder()
            .application_id("com.example.coolpc")
            .build();

        app.connect_activate(move |app| {
            build_ui(app, &items); // 將 items 傳入 GUI
        });

        app.run();
    }

    Ok(())
}

fn build_ui(app: &Application, items: &[ProductInfo]) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("CoolPC Evaluate GUI")
        .default_width(600)
        .default_height(400)
        .build();

    // 創建滾動窗口以顯示 items
    let scrolled_window = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();

    // 創建 ListBox 來顯示每個 item
    let list_box = ListBox::new();
    for item in items {
        let row = ListBoxRow::new();
        let label = Label::new(Some(&format!(
            "{} - {}",
            item.model.as_ref().unwrap_or(&"".to_string()),
            item.price.as_ref().unwrap_or(&"".to_string())
        )));
        row.set_child(Some(&label));
        list_box.set_child(Some(&row)); // 在 ListBox 中添加 row
    }

    scrolled_window.set_child(Some(&list_box)); // 將 list_box 設為滾動窗口的子元素

    let button = Button::with_label("選擇檔案");
    button.connect_clicked(move |_| {
        let dialog = FileChooserDialog::builder()
            .title("選擇檔案")
            .action(gtk::FileChooserAction::Open)
            .build();

        dialog.add_button("取消", ResponseType::Cancel);
        dialog.add_button("開啟", ResponseType::Accept);

        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file_path) = dialog.file() {
                    let path = file_path.path().unwrap();
                    println!("已選擇檔案: {:?}", path);
                    // 在這裡你可以處理檔案並更新 items
                }
            }
            dialog.close();
        });

        dialog.show();
    });

    // 垂直佈局包含按鈕和滾動窗口
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.set_child(Some(&button)); // 將按鈕添加到 vbox 中
    vbox.set_child(Some(&scrolled_window)); // 將滾動窗口添加到 vbox 中

    window.set_child(Some(&vbox)); // 將 vbox 設為 window 的子元素
    window.show();
}
