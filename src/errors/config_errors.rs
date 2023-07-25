use std::fmt;
use std::process::exit;

pub enum ConfigError {
    NullConfigFolder,
    EmptyConfigFolder,
    AccessDeniedConfigFolder(String)
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NullConfigFolder => write!(f, ".google_images_scraper config folder doesn't exist on HOME."),
            ConfigError::EmptyConfigFolder => write!(f, ".google_images_scraper config folder doesn't have the selenium-driver file."),
            ConfigError::AccessDeniedConfigFolder(message) => write!(f, "Couldn't read the .google_images_scraper dir. Detailed error: {}", message)
        }
    }
}

impl ConfigError {
    pub fn default_error_handler(error: ConfigError) {
        eprintln!("{}", error);
        exit(1);
    }
}
