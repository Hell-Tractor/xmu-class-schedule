// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schedule_analyzer;
mod commands;

fn main() {
    // schedule_analyzer::parse("../temp/我的课表app.html", "../temp/我的课表app.json").unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::schedule::get_schedules,
            commands::schedule::get_schedule_list,
            commands::schedule::parse_schedule,
            commands::settings::get_or_default_settings,
            commands::settings::set_settings
        ]).run(tauri::generate_context!())
        .expect("error while running tauri application");
}
