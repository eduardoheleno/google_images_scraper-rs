use std::{thread, time, fs};

use rand::Rng;
use thirtyfour::prelude::*;

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

    pub async fn run(&self) -> WebDriverResult<()> {
        fs::create_dir(&self.args.folder_name).unwrap();

        let mut counter = 0;
        let mut download_limit = DEFAULT_DOWNLOAD_LIMIT;

        if self.args.download_limit.is_some() {
            download_limit = self.args.download_limit.unwrap();
        }

        self.driver.goto(&self.url).await?;
        let clickable_elements = self.driver.find_all(By::ClassName(CLICKABLE_ELEMENT)).await?;

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
                downloader.download();
            });

            handle.join().unwrap();

            counter += 1;
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

        println!("{}", seconds);
        time::Duration::from_secs(seconds)
    }

    pub async fn finish_webdriver(self) {
        self.driver.quit().await.unwrap();
    }
}
