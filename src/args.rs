use std::env;
use std::process::exit;

pub struct Args {
    pub search_parameter: String,
    pub folder_name: String,
    pub download_limit: Option<i16>
}

impl Args {
    pub fn parse() -> Args {
        let vec_args: Vec<String> = env::args().collect();

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

        let folder_name = if let Some(folder_name) = vec_args.get(3) {
            folder_name.to_string()
        } else {
            eprintln!("You must insert a folder name.");
            exit(1);
        };

        Args { search_parameter, download_limit, folder_name }
    }
}
