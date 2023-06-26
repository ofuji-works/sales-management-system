#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;

use infrastructure::tauri::customer;
use tauri::Manager;

use crate::infrastructure::{database, tauri::product};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sqlite_pool = database::excute()?;

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(sqlite_pool)
        .invoke_handler(tauri::generate_handler![
            product::find_by_id_product,
            product::search_product,
            product::create_product,
            product::update_product,
            product::delete_product,
            customer::create_customer,
            customer::update_customer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
