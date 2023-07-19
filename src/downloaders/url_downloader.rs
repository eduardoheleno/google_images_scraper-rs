use std::fs::File;
use std::io::prelude::*;

use crate::DownloaderTrait;

pub struct UrlDownloader {
    pub src: String,
    pub folder_name: String,
    pub file_name: String
}

impl DownloaderTrait for UrlDownloader {
    fn download(&self) {
        println!("Downloading from: {}", self.src);

        let resp = reqwest::blocking::get(&self.src).unwrap();

        let file_extension = self.get_file_extension();

        let file_name_with_extension = format!("{}{}", self.file_name, file_extension);
        let file_path = format!("./{}/{}", self.folder_name, file_name_with_extension);
        let mut file = File::create(file_path).unwrap();

        file.write_all(&resp.bytes().unwrap()).unwrap();
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
}
