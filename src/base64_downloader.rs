use super::DownloaderTrait;

pub struct Base64Downloader {
    pub src: String,
    pub folder_name: String,
    pub file_name: String
}

impl DownloaderTrait for Base64Downloader {
    fn download(&self) {
        println!("{}", self.src);
        println!("Base64 download");
    }

    fn get_file_extension(&self) -> Option<&'static str> {
        todo!();
    }
}
