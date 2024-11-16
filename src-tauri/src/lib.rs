extern crate dirs;
use std::fs;
use std::path;
use json;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!(
        "Hello, {}! You've been greeted from Rust! modified by Junaid again.",
        name
    )
}

fn get_project_home_directory() -> std::path::PathBuf {
    // path::Path(dirs::home_dir());
    // fs::create_dir();
    let home_dir = dirs::home_dir().unwrap();
    let app_home = home_dir.join("idaf-scan-app");
    fs::create_dir_all(app_home.clone()).ok();

    return app_home;
}


#[tauri::command]
fn create_project(project_name: String, scan_location: String, description: String) -> bool {
    let app_home: path::PathBuf = get_project_home_directory();
    let project_dir = app_home.join(project_name.to_lowercase());
    if !project_dir.exists() {
        fs::create_dir_all(project_dir.clone()).ok();
    }

    let project_info = json::object!{
        name: project_name,
        scan_location: scan_location,
        description: description
    };

    let project_info_file = project_dir.join("project.json");

    fs::write(project_info_file.into_os_string().into_string().unwrap(), json::stringify(project_info)).expect("Unable to write file");

    return true;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![create_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
