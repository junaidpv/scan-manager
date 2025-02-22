use std::fs;
use std::path;
use json;
use regex::Regex;
use std::io;
use std::path::Path;
use serde::Serialize;

use crate::images::ImageInfo;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

fn get_projects_directory() -> std::path::PathBuf {
    // path::Path(dirs::home_dir());
    // fs::create_dir();
    let home_dir = dirs::home_dir().unwrap();
    // Prepare path to the projects directory.
    let projects_dir = home_dir.join("idaf-scan-app").join("projects");
    fs::create_dir_all(projects_dir.clone()).ok();

    return projects_dir;
}


#[tauri::command(rename_all = "snake_case")]
pub fn create_project(name: String, scan_location: String, description: String, created_at: f64, updated_at: f64) -> String {
    let mut result = false;
    let message;
    let projects_dir: path::PathBuf = get_projects_directory();
    let project_dir = projects_dir.join(name.to_lowercase());
    if project_dir.exists() {
        message = "Project already exists";
    }
    else {
        fs::create_dir_all(project_dir.clone()).ok();
        let project_info = json::object!{
            name: name,
            scan_location: scan_location,
            description: description,
            created_at: created_at,
            updated_at: updated_at,
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

// Get list of directory names in the projects directory.
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

fn list_files<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let mut file_names = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                file_names.push(file_name.to_string());
            }
        }
    }

    Ok(file_names)
}

fn get_project_info(project_name: String) -> io::Result<json::JsonValue> {
    let projects_dir = get_projects_directory();
    let project_dir = projects_dir.join(project_name.to_lowercase());
    let project_info_file = project_dir.join("project.json");
    let project_info = fs::read_to_string(project_info_file)?;

    Ok(json::parse(&project_info).unwrap())
}

#[tauri::command]
pub fn get_projects() -> String {
    let projects_dir = get_projects_directory();
    let mut result = true;
    let mut project_infos: Vec<json::JsonValue> = Vec::new();
    let result_call = list_directory_names(projects_dir);
    if result_call.is_ok() {
        let dir_names  = result_call.unwrap();
        for dir_name in dir_names.iter() {
            let project_info = get_project_info(dir_name.clone());
            if project_info.is_ok() {
                project_infos.push(project_info.unwrap());
            }
        }
    }
    else {
        result = false;
    }

    return json::stringify(json::object!{
        result: result,
        projects: project_infos
    });
}

#[derive(Serialize)]
struct ProjectImages {
    result: bool,
    images: Vec<ImageInfo>
}

impl ProjectImages {
    fn new(result: bool, images: Vec<ImageInfo>) -> Self {
        ProjectImages { result, images }
    }
}

#[tauri::command]
pub fn get_project_images(project_name: String) -> String {
    let project_info = get_project_info(project_name.clone());
    let scan_location = path::PathBuf::from( project_info.unwrap()["scan_location"].to_string());

    let mut result = true;
    let mut image_infos: Vec<ImageInfo> = Vec::new();
    let result_call = list_files(scan_location.clone());
    let re = Regex::new(r"^(?i)(.*\.(jpg|jpeg|png|gif|tif))$").unwrap();
    if result_call.is_ok() {
        let file_names  = result_call.unwrap();
        for file_name in file_names.iter() {
            if re.is_match(&file_name) {
                let image_full_path = scan_location.join(file_name);
                // image_infos.push(image_full_path.into_os_string().into_string().unwrap());
                let image_info = ImageInfo::new(image_full_path.into_os_string().into_string().unwrap());
                image_infos.push(image_info);
            }
        }
    }
    else {
        result = false;
    }

    let project_images = ProjectImages::new(result, image_infos);

    return serde_json::to_string(&project_images).expect("{result: false, images: []}");
}
