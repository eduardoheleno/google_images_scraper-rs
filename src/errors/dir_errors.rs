use std::fmt;
use std::process::exit;

use thirtyfour::WebDriver;

pub enum DirError {
    FolderAlreadyExists,
    CouldntCreateFolder(String),
    CouldntCreateFile(String),
    CouldntWriteFile(String)
}

impl fmt::Display for DirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirError::FolderAlreadyExists => write!(f, "Folder already exists, try insert another name."),
            DirError::CouldntCreateFolder(message) => write!(f, "Fail at create folder. Detailed error: {}", message),
            DirError::CouldntCreateFile(message) => write!(f, "Fail at create file. Detailed error: {}", message),
            DirError::CouldntWriteFile(message) => write!(f, "Fail at write bytes to the file. Detailed error: {}", message)
        }
    }
}

impl DirError {
    pub async fn default_error_handler(error: DirError, driver: WebDriver, webdriver_process: &mut std::process::Child) {
        driver.quit().await.unwrap();
        webdriver_process.kill().unwrap();

        eprintln!("{}", error);
        exit(1);
    }
}
