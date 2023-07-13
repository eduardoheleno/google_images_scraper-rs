use std::process::exit;

pub struct Args {
    pub search_parameter: String,
    pub download_limit: Option<i16>
}

impl Args {
    pub fn new(vec_args: Vec<String>) -> Args {
        let search_parameter = if let Some(search_parameter) = vec_args.get(1) {
            search_parameter.to_string()
        } else {
            eprintln!("The scraper needs a search parameter.");
            exit(1);
        };

        let download_limit = if let Some(download_limit) = vec_args.get(2) {
            let download_limit_buffer = download_limit.parse::<i16>();
            match download_limit_buffer {
                Ok(parsed_download_limit) => Some(parsed_download_limit),
                Err(_) => {
                    eprintln!("The download limit must be a integer.");
                    exit(1);
                }
            }
        } else {
            None
        };

        Args { search_parameter, download_limit }
    }
}
