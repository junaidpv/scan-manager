extern crate dirs;

pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![commands::greet, commands::create_project, commands::get_projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
