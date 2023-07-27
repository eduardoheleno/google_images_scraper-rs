use std::env;
use std::process::exit;

pub struct Args {
    pub search_parameter: String,
    pub folder_name: String,
    pub download_limit: Option<i16>
}

impl Args {
    pub fn parse() -> Args {
        let download_limit_arg: String = String::from("--download-limit");
        let folder_name_arg: String = String::from("--folder-name");

        let vec_args: Vec<String> = env::args().collect();

        let search_parameter = if let Some(search_parameter) = vec_args.get(1) {
            search_parameter.to_string()
        } else {
            eprintln!("The scraper needs a search parameter.");
            exit(1);
        };

        let download_limit = if vec_args.contains(&download_limit_arg) {
            let download_limit_arg_index = vec_args.iter().position(|arg| arg == &download_limit_arg).unwrap();
            let download_limit_value = vec_args.get(download_limit_arg_index + 1);

            if download_limit_value.is_none() {
                eprintln!("You must insert the download limit value.");
                exit(1);
            }

            let parse_action = download_limit_value.unwrap().parse::<i16>();
            match parse_action {
                Ok(parsed_download_limit) => Some(parsed_download_limit),
                Err(_) => {
                    eprintln!("The download limit must be an integer.");
                    exit(1);
                }
            }
        } else {
            None
        };

        let folder_name = if vec_args.contains(&folder_name_arg) {
            let folder_name_arg_index = vec_args.iter().position(|arg| arg == &folder_name_arg).unwrap();
            let folder_name_value = vec_args.get(folder_name_arg_index + 1);

            if folder_name_value.is_none() || folder_name_value.unwrap() == &download_limit_arg {
                eprintln!("You must insert the folder name value.");
                exit(1);
            }

            folder_name_value.unwrap().to_string()
        } else {
            search_parameter.clone()
        };

        Args { search_parameter, download_limit, folder_name }
    }
}
