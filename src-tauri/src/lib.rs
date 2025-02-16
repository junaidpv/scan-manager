extern crate dirs;
use std::fs;
use std::path;
use json;
use std::io;
use std::path::Path;
use serde_json::Value;
use serde_json::json as serde_json;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!(
        "Hello, {}! You've been greeted from Rust! modified by Junaid again.",
        name
    )
}

fn get_projects_directory() -> std::path::PathBuf {
    // path::Path(dirs::home_dir());
    // fs::create_dir();
    let home_dir = dirs::home_dir().unwrap();
    let projects_dir = home_dir.join("idaf-scan-app").join("projects");
    fs::create_dir_all(projects_dir.clone()).ok();

    return projects_dir;
}


#[tauri::command]
fn create_project(project_name: String, scan_location: String, description: String) -> String {
    let mut result = false;
    let message;
    let projects_dir: path::PathBuf = get_projects_directory();
    let project_dir = projects_dir.join(project_name.to_lowercase());
    if project_dir.exists() {
        message = "Project already exists";
    }
    else {
        fs::create_dir_all(project_dir.clone()).ok();
        let project_info = json::object!{
            name: project_name,
            scan_location: scan_location,
            description: description
        };
    
        let project_info_file = project_dir.join("project.json");
    
        fs::write(project_info_file.into_os_string().into_string().unwrap(), json::stringify(project_info)).expect("Unable to write file");
        result = true;
        message = "Project created";
    }


    return json::stringify(json::object!{
        result: result,
        message: message
    });
}

fn list_directory_names<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let mut dir_names = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|name| name.to_str()) {
                dir_names.push(dir_name.to_string());
            }
        }
    }

    Ok(dir_names)
}

#[tauri::command]
fn get_projects() -> String {
    let projects_dir = get_projects_directory();
    // let mut vec = vec![];
    // let mut dir_names = Vec::new();
    // let mut dir_reader = fs::read_dir(projects_dir)?;
    // for entry in dir_reader {
    //     let entry = entry?;
    //     let path = entry.path();
    //     if path.is_dir() {
    //         vec.push(entry.file_name().into_string().unwrap());
    //         // visit_dirs(&path, cb)?;
    //     }
    // }
    // return json::stringify(json::object!{
    //     projects: vec
    // });
    let result = true;
    // let mut dir_names = Vec::new();
    let dir_names = match list_directory_names(projects_dir) {
        Ok(projects_dir) => projects_dir,
        Err(_e) => Vec::new()
    };
    // let json_value: Value = serde_json!(dir_names); // Convert Vec<String> to JSON
    // Ok(json_value.to_string()); // Serialize JSON to a string

    return json::stringify(json::object!{
        result: result,
        names: dir_names
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![create_project])
        .invoke_handler(tauri::generate_handler![get_projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
