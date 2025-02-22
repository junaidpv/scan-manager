
use serde::Serialize;
use image::io::Reader as ImageReader;
use std::fs;
use std::path::{self, Path};

/**
 * Struct to hold image information which will be serialized to JSON
 * and sent to the front-end side.
 */
#[derive(Serialize)]
pub struct ImageInfo {
    width: u32,
    height: u32,
    size: u64,
    format: String,
    path: String,
}

impl ImageInfo {

    /**
     * Creates a new ImageInfo struct from a given path.
     * @param path The path to the image.
     * @return A new ImageInfo struct.
     */
    pub fn new(path: String) -> Self {
        let (width, height) = get_image_size(&path).unwrap();
        let size = fs::metadata(&path).unwrap().len();
        let format = path::Path::new(&path).extension().unwrap().to_str().unwrap().to_string();
        ImageInfo { width, height, size, format, path }
    }
}

/**
 * Gets the size of an image from a given path.
 * @param path The path to the image.
 * @return A tuple containing the width and height of the image.
 */
pub fn get_image_size<P: AsRef<Path>>(path: P) -> Result<(u32, u32), image::ImageError> {
    // Open the image file
    let img = ImageReader::open(path)?;
    let dimensions = img.into_dimensions()?;

    // Get the dimensions (width, height) of the image
    Ok(dimensions)
}