extern crate dirs;

pub mod commands;
pub mod images;
pub mod projects;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::create_project,
            commands::get_projects,
            commands::get_project_images
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
