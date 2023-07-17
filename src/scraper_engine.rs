use std::{thread, time};

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

    pub async fn run(self) -> WebDriverResult<()> {
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
            thread::sleep(ScraperEngine::get_random_duration());

            let img_element = if let Some(img_element) = self.get_img_element().await {
                img_element
            } else {
                continue;
            };

            let src = img_element.attr("src").await?.unwrap();
            let downloader = SrcParser::parse(src);
            downloader.download();

            counter += 1;
        }

        self.driver.quit().await?;

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

    fn get_random_duration() -> time::Duration {
        let seconds: u64 = rand::thread_rng().gen_range(3..10);
        println!("{}", seconds);
        time::Duration::from_secs(seconds)
    }
}
