use super::url_downloader::UrlDownloader;
use super::base64_downloader::Base64Downloader;
use super::DownloaderTrait;

pub struct SrcParser;

impl SrcParser {
    pub fn parse(src: String) -> Box<dyn DownloaderTrait> {
        if src.contains("data:image") {
            return Box::new(Base64Downloader{ src });
        }

        Box::new(UrlDownloader{ src })
    }
}
