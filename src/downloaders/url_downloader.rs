use std::fs::File;
use std::io::prelude::*;

use crate::DownloaderTrait;
use crate::errors::dir_errors::DirError;

pub struct UrlDownloader {
    pub src: String,
    pub folder_name: String,
    pub file_name: String
}

impl DownloaderTrait for UrlDownloader {
    fn download(&self) -> Result<(), DirError> {
        println!("Downloading from: {}", self.src);

        let resp = reqwest::blocking::get(&self.src).unwrap();

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

        match file.write_all(&resp.bytes().unwrap()) {
            Ok(_) => {},
            Err(e) => {
                return Err(DirError::CouldntWriteFile(e.to_string()));
            }
        };

        Ok(())
    }

    fn get_file_extension(&self) -> &'static str {
        if self.src.contains(".jpg") || self.src.contains(".jpeg") {
            return ".jpg";
        } else if self.src.contains(".png") {
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
