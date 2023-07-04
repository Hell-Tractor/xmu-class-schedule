// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schedule_analyzer;
mod commands;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // schedule_analyzer::parse("../temp/我的课表app.html", "../temp/我的课表app.json").unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::schedule::get_schedules, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
