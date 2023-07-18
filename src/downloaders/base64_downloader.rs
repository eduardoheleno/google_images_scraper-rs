use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use crate::DownloaderTrait;
use base64::{Engine as _, engine::general_purpose};

pub struct Base64Downloader {
    pub src: String,
    pub folder_name: String,
    pub file_name: String
}

impl DownloaderTrait for Base64Downloader {
    fn download(&self) {
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
        if file_extension.is_none() {
            eprintln!("File doesn't have an extension.");
            exit(1);
        }

        let file_name_with_extension = format!("{}{}", self.file_name, file_extension.unwrap());
        let file_path = format!("./{}/{}", self.folder_name, file_name_with_extension);
        let mut file = File::create(file_path).unwrap();

        file.write_all(&bytes).unwrap();
    }

    fn get_file_extension(&self) -> Option<&'static str> {
        if self.src.contains("data:image/jpeg") || self.src.contains("data:image/jpg") {
            return Some(".jpg");
        } else if self.src.contains("data:image/png") {
            return Some(".png");
        } else {
            return None;
        }
    }
}
