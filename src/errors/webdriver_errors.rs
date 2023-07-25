use std::fmt;
use std::process::exit;

pub enum WebdriverError {
    InitDriverError(String),
    InitProcessError(String)
}

impl fmt::Display for WebdriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebdriverError::InitDriverError(message) => write!(f, "Fail at init driver. Detailed error: {}", message),
            WebdriverError::InitProcessError(message) => write!(f, "Fail at init driver process. Detailed error: {}", message)
        }
    }
}

impl WebdriverError {
    pub fn default_error_handler(error: WebdriverError) {
        eprintln!("{}", error);
        exit(1);
    }
}
