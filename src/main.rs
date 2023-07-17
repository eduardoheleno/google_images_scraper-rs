mod args;
mod scraper_engine;
mod src_parser;
mod url_downloader;
mod base64_downloader;

use thirtyfour::prelude::*;

use args::Args;
use scraper_engine::ScraperEngine;

const WEBDRIVER_HOST: &str = "http://localhost:4444";

pub trait DownloaderTrait {
    fn download(&self);
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let args = Args::parse();

    let google_images_url = format!("https://www.google.com/search?q={}&tbm=isch", args.search_parameter);

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(WEBDRIVER_HOST, caps).await?;

    let scraper_engine = ScraperEngine::new(driver, google_images_url, args);
    scraper_engine.run().await?;

    Ok(())
}
