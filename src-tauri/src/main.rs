// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use headless_chrome::Browser;
use headless_chrome::types::PrintToPdfOptions;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
struct PdfOptions {
    landscape: Option<bool>,
    display_header_footer: Option<bool>,
    print_background: Option<bool>,
    scale: Option<f64>,
    paper_width: Option<f64>,
    paper_height: Option<f64>,
    margin_top: Option<f64>,
    margin_bottom: Option<f64>,
    margin_left: Option<f64>,
    margin_right: Option<f64>,
    page_ranges: Option<String>,
    ignore_invalid_page_ranges: Option<bool>,
    header_template: Option<String>,
    footer_template: Option<String>,
    prefer_css_page_size: Option<bool>,
    transfer_mode: Option<String>,
}

#[tauri::command]
async fn convert(output_dir: &str, files: Vec<&str>, options: Option<PdfOptions>) -> Result<String, String> {
    let browser = Browser::default().unwrap();

    let tab = browser.new_tab().unwrap();

    files.iter().for_each(|path| {
        tab.navigate_to(&format!("file://{}", path)).unwrap();
        tab.wait_until_navigated().unwrap();

        let default = PdfOptions {
            landscape: Some(true),
            display_header_footer: Some(true),
            print_background: Some(true),
            scale: Some(1.),
            paper_width: Some(8.5),
            paper_height: Some(11.),
            margin_top: Some(0.4),
            margin_bottom: Some(0.4),
            margin_left: Some(0.4),
            margin_right: Some(0.4),
            page_ranges: None,
            ignore_invalid_page_ranges: None,
            header_template: Some("<div></div>".to_string()),
            footer_template: Some("<div></div>".to_string()),
            prefer_css_page_size: None,
            transfer_mode: None,
        };

        let options = options.as_ref().unwrap_or(&default);

        let pdf = tab.print_to_pdf(Some(PrintToPdfOptions {
            landscape: options.landscape,
            display_header_footer: options.display_header_footer,
            print_background: options.print_background,
            scale: options.scale,
            paper_width: options.paper_width,
            paper_height: options.paper_height,
            margin_top: options.margin_top,
            margin_bottom: options.margin_bottom,
            margin_left: options.margin_left,
            margin_right: options.margin_right,
            page_ranges: options.page_ranges.clone(),
            ignore_invalid_page_ranges: options.ignore_invalid_page_ranges,
            header_template: options.header_template.clone(),
            footer_template: options.footer_template.clone(),
            prefer_css_page_size: options.prefer_css_page_size,
            transfer_mode: None,
        })).unwrap();
            
        let path = Path::new(path);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_name = format!("{output_dir}/{file_name}.pdf");
    
        std::fs::write(file_name, pdf).unwrap();
    });

    Ok(String::new())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
