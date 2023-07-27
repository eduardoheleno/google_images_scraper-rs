use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use crate::{DownloaderTrait, errors::dir_errors::DirError};
use base64::{Engine as _, engine::general_purpose};

pub struct Base64Downloader {
    pub src: String,
    pub folder_name: String,
    pub file_name: String
}

impl DownloaderTrait for Base64Downloader {
    fn download(&self) -> Result<(), DirError> {
        println!("Downloading from: {}", self.src);

        let image_uri: Vec<&str> = self.src.split(',').collect();
        let base64_image = if let Some(base64_image) = image_uri.get(1) {
            base64_image
        } else {
            println!("Couldn't find the base64 image.");
            exit(1);
        };

        let bytes = general_purpose::STANDARD
            .decode(base64_image).unwrap();

        let file_extension = self.get_file_extension();

        let file_name_with_extension = format!("{}{}", self.file_name, file_extension);
        let download_path = dirs::download_dir().unwrap();
        let file_path = format!("{}/{}/{}", download_path.to_str().unwrap(), self.folder_name, file_name_with_extension);

        let mut file = match self.create_file(file_path) {
            Ok(file) => file,
            Err(e) => {
                return Err(e);
            }
        };

        match file.write_all(&bytes) {
            Ok(_) => {},
            Err(e) => {
                return Err(DirError::CouldntWriteFile(e.to_string()));
            }
        };

        Ok(())
    }

    fn get_file_extension(&self) -> &'static str {
        if self.src.contains("data:image/jpeg") || self.src.contains("data:image/jpg") {
            return ".jpg";
        } else if self.src.contains("data:image/png") {
            return ".png";
        } else {
            return "";
        }
    }

    fn create_file(&self, file_path: String) -> Result<File, DirError> {
        let file = match File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                return Err(DirError::CouldntCreateFile(e.to_string()));
            }
        };

        Ok(file)
    }
}
