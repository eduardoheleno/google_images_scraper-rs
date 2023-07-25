mod args;
mod scraper_engine;
mod src_parser;
mod downloaders;
mod errors;

use std::thread;
use std::fs;
use std::path::Path;
use std::process::{Command, Child};

use thirtyfour::prelude::*;

use args::Args;
use scraper_engine::ScraperEngine;
use errors::webdriver_errors::WebdriverError;
use errors::config_errors::ConfigError;

const WEBDRIVER_HOST: &str = "http://localhost:4444";
const WEBDRIVER_FOLDER: &str = "/.google_images_scraper/";

pub trait DownloaderTrait {
    fn download(&self);
    fn get_file_extension(&self) -> &'static str;
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let args = Args::parse();

    let config_folder_path = get_config_folder_path().map_err(|err| ConfigError::default_error_handler(err)).unwrap();
    let selenium_file_name = get_selenium_file_name(&config_folder_path).map_err(|err| ConfigError::default_error_handler(err)).unwrap();

    let mut webdriver_process = start_webdriver_process(config_folder_path, selenium_file_name).map_err(|err| WebdriverError::default_error_handler(err)).unwrap();

    let google_images_url = format!("https://www.google.com/search?q={}&tbm=isch", args.search_parameter);

    let driver = start_webdriver().await.map_err(|err| WebdriverError::default_error_handler(err)).unwrap();
    let scraper_engine = ScraperEngine::new(driver, google_images_url, args);

    match scraper_engine.run().await {
        Ok(_) => {
            scraper_engine.finish_webdriver().await;
            webdriver_process.kill().unwrap();
        },
        Err(_) => {
            scraper_engine.finish_webdriver().await;
            webdriver_process.kill().unwrap();
        }
    }

    Ok(())
}

fn start_webdriver_process(config_folder_path: String, selenium_file_name: String) -> Result<Child, WebdriverError> {
    let webdriver_execution = Command::new("java")
        .arg("-jar")
        .current_dir(config_folder_path)
        .arg(selenium_file_name)
        .arg("standalone")
        .spawn();

    let webdriver_process = match webdriver_execution {
        Ok(webdriver_process) => webdriver_process,
        Err(e) => {
            return Err(WebdriverError::InitProcessError(e.to_string()));
        }
    };

    println!("Starting webdriver process...");
    thread::sleep(ScraperEngine::generate_duration(Some(5)));
    println!("Webdriver process has been started successfully!");

    Ok(webdriver_process)
}

async fn start_webdriver() -> Result<WebDriver, WebdriverError> {
    let caps = DesiredCapabilities::chrome();

    match WebDriver::new(WEBDRIVER_HOST, caps).await {
        Ok(webdriver) => Ok(webdriver),
        Err(e) => Err(WebdriverError::InitDriverError(e.to_string()))
    }
}

fn get_config_folder_path() -> Result<String, ConfigError> {
    let home_path = dirs::home_dir().unwrap();
    let selenium_path = format!("{}{}", home_path.to_str().unwrap(), WEBDRIVER_FOLDER);

    let is_path_a_folder = Path::new(&selenium_path).is_dir();
    if is_path_a_folder == false {
        return Err(ConfigError::NullConfigFolder);
    }

    Ok(selenium_path)
}

fn get_selenium_file_name(config_folder_path: &String) -> Result<String, ConfigError> {
    let mut path_buffer = match fs::read_dir(config_folder_path) {
        Ok(paths) => paths,
        Err(e) => {
            return Err(ConfigError::AccessDeniedConfigFolder(e.to_string()));
        }
    };

    let file_name = if let Some(file) = path_buffer.nth(0) {
        file.unwrap().path().file_name().unwrap().to_string_lossy().to_string()
    } else {
        return Err(ConfigError::EmptyConfigFolder);
    };

    Ok(file_name)
}
