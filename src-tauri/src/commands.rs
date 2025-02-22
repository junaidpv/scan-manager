use std::fs;
use std::path;
use json;
use regex::Regex;
use std::io;

use crate::images::ImageInfo;
use crate::projects::get_projects_directory;
use crate::projects::list_directory_names;
use crate::projects::list_files;
use crate::projects::ProjectImages;



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


/**
 * Get information about the project.
 * 
 * @param project_name The name of the project.
 */
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

/**
 * Get the images of a project.
 * This will return a list of images that are in the project.
 */
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
