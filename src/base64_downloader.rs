use super::DownloaderTrait;

pub struct Base64Downloader {
    pub src: String
}

impl DownloaderTrait for Base64Downloader {
    fn download(&self) {
        println!("{}", self.src);
        println!("Base64 download");
    }
}
