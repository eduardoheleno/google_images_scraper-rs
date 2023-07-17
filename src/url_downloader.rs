use super::DownloaderTrait;

pub struct UrlDownloader {
    pub src: String
}

impl DownloaderTrait for UrlDownloader {
    fn download(&self) {
        println!("{}", self.src);
        println!("Url download");
    }
}
