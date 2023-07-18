use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use super::DownloaderTrait;

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
        if file_extension.is_none() {
            eprint!("File doesn't have an extension.");
            exit(1);
        }

        let file_name_with_extension = format!("{}{}", self.file_name, file_extension.unwrap());
        let file_path = format!("./{}/{}", self.folder_name, file_name_with_extension);
        let mut file = File::create(file_path).unwrap();

        file.write_all(&resp.bytes().unwrap()).unwrap();
    }

    fn get_file_extension(&self) -> Option<&'static str> {
        if self.src.contains(".jpg") || self.src.contains(".jpeg") {
            return Some(".jpg");
        } else if self.src.contains(".png") {
            return Some(".png");
        } else {
            return None;
        }
    }
}
