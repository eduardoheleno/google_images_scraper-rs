use std::process::Child;
use std::{thread, time, fs};
use std::path::Path;

use rand::Rng;
use thirtyfour::prelude::*;

use crate::errors::dir_errors::DirError;

use super::Args;
use super::src_parser::SrcParser;

const CLICKABLE_ELEMENT: &str = "islib";
const IMG_ELEMENT_URL: &str = "iPVvYb";
const IMG_ELEMENT_BASE64: &str = "pT0Scc";

const DEFAULT_DOWNLOAD_LIMIT: i16 = 10;

pub struct ScraperEngine {
    driver: WebDriver,
    url: String,
    args: Args
}

impl ScraperEngine {
    pub fn new(driver: WebDriver, url: String, args: Args) -> ScraperEngine {
        ScraperEngine { driver, url, args }
    }

    pub async fn run(&self, mut webdriver_process: &mut Child) -> WebDriverResult<()> {
        match self.create_images_folder() {
            Ok(_) => {},
            Err(e) => {
                DirError::default_error_handler(e, self.driver.clone(), &mut webdriver_process).await;
            }
        }

        let mut counter = 0;
        let mut download_limit = DEFAULT_DOWNLOAD_LIMIT;

        if self.args.download_limit.is_some() {
            download_limit = self.args.download_limit.unwrap();
        }

        self.driver.goto(&self.url).await?;
        let clickable_elements = self.driver.find_all(By::ClassName(CLICKABLE_ELEMENT)).await?;

        let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

        for element in clickable_elements {
            if counter == download_limit {
                break;
            }

            element.click().await?;
            thread::sleep(ScraperEngine::generate_duration(None));

            let img_element = if let Some(img_element) = self.get_img_element().await {
                img_element
            } else {
                continue;
            };

            let alt = img_element.attr("alt").await?.unwrap();
            let src = img_element.attr("src").await?.unwrap();

            let downloader = SrcParser::parse(src, self.args.folder_name.clone(), alt);

            let handle = thread::spawn(move || {
                match downloader.download() {
                    Ok(_) => {},
                    Err(e) => DirError::print_error_handler(e)
                }
            });

            thread_vec.push(handle);

            counter += 1;
        }

        for thread in thread_vec.into_iter() {
            thread.join().unwrap();
        }

        Ok(())
    }

    fn create_images_folder(&self) -> Result<(), DirError> {
        let download_path = dirs::download_dir().unwrap();
        let images_folder_path = format!("{}/{}", download_path.to_str().unwrap(), &self.args.folder_name);
        let is_folder_already_created = Path::new(&images_folder_path).is_dir();

        if is_folder_already_created == true {
            return Err(DirError::FolderAlreadyExists);
        }

        match fs::create_dir(images_folder_path) {
            Ok(_) => {},
            Err(e) => {
                return Err(DirError::CouldntCreateFolder(e.to_string()));
            }
        }

        Ok(())
    }

    async fn get_img_element(&self) -> Option<WebElement> {
        let img_element = if let Ok(img_element) = self.driver.find(By::ClassName(IMG_ELEMENT_URL)).await {
            Some(img_element)
        } else {
            let base64_img = if let Ok(base64_img) = self.driver.find(By::ClassName(IMG_ELEMENT_BASE64)).await {
                Some(base64_img)
            } else {
                None
            };

            base64_img
        };

        img_element
    }

    pub fn generate_duration(seconds: Option<u64>) -> time::Duration {
        let seconds = match seconds {
            Some(duration) => duration,
            None => rand::thread_rng().gen_range(3..10)
        };

        println!("Waiting {} seconds...", seconds);
        time::Duration::from_secs(seconds)
    }

    pub async fn finish_webdriver(self) {
        self.driver.quit().await.unwrap();
    }
}
