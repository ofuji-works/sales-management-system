#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;

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
            product::create_product,
            product::search_product,
            product::update_product,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
