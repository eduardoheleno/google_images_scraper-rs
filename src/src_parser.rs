use super::downloaders::url_downloader::UrlDownloader;
use super::downloaders::base64_downloader::Base64Downloader;
use super::DownloaderTrait;

pub struct SrcParser;

impl SrcParser {
    pub fn parse(src: String, folder_name: String, raw_file_name: String) -> Box<dyn DownloaderTrait + Sync + Send> {
        let file_name = SrcParser::format_file_name(raw_file_name);

        if src.contains("data:image") {
            return Box::new(Base64Downloader{ src, folder_name, file_name });
        }

        Box::new(UrlDownloader{ src, folder_name, file_name })
    }

    fn format_file_name(file_name: String) -> String {
        file_name.replace(' ', "-")
    }
}
