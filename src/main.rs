mod args;
mod scraper_engine;
mod src_parser;
mod downloaders;

use std::process::{Command, Child, exit};

use thirtyfour::prelude::*;

use args::Args;
use scraper_engine::ScraperEngine;

const WEBDRIVER_HOST: &str = "http://localhost:4444";
const WEBDRIVER_FOLDER: &str = "/.google_images_scraper/";

pub trait DownloaderTrait {
    fn download(&self);
    fn get_file_extension(&self) -> &'static str;
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let args = Args::parse();

    let mut webdriver_process = start_webdriver_process();

    let google_images_url = format!("https://www.google.com/search?q={}&tbm=isch", args.search_parameter);

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(WEBDRIVER_HOST, caps).await?;

    let scraper_engine = ScraperEngine::new(driver, google_images_url, args);
    scraper_engine.run().await?;

    webdriver_process.kill().unwrap();

    Ok(())
}

fn start_webdriver_process() -> Child {
    let home_path = dirs::home_dir().unwrap();
    let selenium_path = format!("{}{}", home_path.to_str().unwrap(), WEBDRIVER_FOLDER);

    let webdriver_execution = Command::new("java")
        .arg("-jar")
        .current_dir(selenium_path)
        .arg("selenium-server-4.10.0.jar")
        .arg("standalone")
        .spawn();

    let webdriver_process = match webdriver_execution {
        Ok(webdriver_process) => webdriver_process,
        Err(_) => {
            eprintln!("Fail at start webdriver_process.");
            exit(1);
        }
    };

    webdriver_process
}
