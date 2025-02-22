use std::fs;
use std::io;
use std::path::Path;
use serde::Serialize;

use crate::images::ImageInfo;

#[derive(Serialize)]
pub struct ProjectInfo {
    pub name: String,
    pub scan_location: String,
    pub description: String,
    pub created_at: Option<f64>,
    pub updated_at: Option<f64>,
}

pub fn get_projects_directory() -> std::path::PathBuf {
    // path::Path(dirs::home_dir());
    // fs::create_dir();
    let home_dir = dirs::home_dir().unwrap();
    // Prepare path to the projects directory.
    let projects_dir = home_dir.join("idaf-scan-app").join("projects");
    fs::create_dir_all(projects_dir.clone()).ok();

    return projects_dir;
}

pub fn list_directory_names<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
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

pub fn list_files<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
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
    file_names.sort();
    Ok(file_names)
}


/**
 * Get information about the project.
 *
 * @param project_name The name of the project.
 */
pub fn get_project_info(project_name: String) -> io::Result<json::JsonValue> {
    let projects_dir = get_projects_directory();
    let project_dir = projects_dir.join(project_name.to_lowercase());
    let project_info_file = project_dir.join("project.json");
    let project_info = fs::read_to_string(project_info_file)?;

    Ok(json::parse(&project_info).unwrap())
}

/**
 * Creates a new ProjectImages struct from a given result and images.
 * This will be used to return the images from the project.
 *
 */
#[derive(Serialize)]
pub struct ProjectImages {
    /**
     * The result of the operation.
     * This will be true if the operation was successful and false if it was not.
     *
     */
    result: bool,
    /**
     * The images of the project.
     */
    images: Vec<ImageInfo>
}

/**
 * Creates a new ProjectImages struct from a given result and images.
 *
 * @param result The result of the operation.
 * @param images The images to be included in the struct.
 * @return A new ProjectImages struct.
 *
 */
impl ProjectImages {
    pub fn new(result: bool, images: Vec<ImageInfo>) -> Self {
        ProjectImages { result, images }
    }
}