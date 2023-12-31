# google_images_scraper-rs

This is a command-line app that scrapes the results of a Google Images search.<br>
The app uses a Selenium WebDriver to render the search results into a browser, this is necessary because the searched images are only loaded when the request is interpreted by a browser.<br>
With that being said, the program needs a `selenium-server.jar` file stored in `.google_images_scraper` folder on `Home` directory. You can download it [**here**](https://www.selenium.dev/downloads/).<br>
After the download, just create the `.google_images_scraper` folder on your `Home` directory and then drop the .jar file.<br>

## installation
### Linux
To install this program you'll need Rust and Cargo installed in your machine.<br>
You can install both [**here**](https://www.rust-lang.org/).<br>

Then run:

``` shell
$ cargo install --git https://github.com/EduardoPD1921/google_images_scraper-rs
```


### Usage:

```
$ google_images_scraper-rs [SEARCH_PARAMETER] [OPTIONS]
```

Args:
- `search_parameter <string>`: (required) String that's going to be inserted on Google Images text input.
- Options:
  - `--download-limit <integer>`: (optional) Limit of downloads per scraping.
  - `--folder-name <string>`: (optional) Custom folder name with downloaded images.
  
### Known issues

Until now, the scraper has a limitation of the quantity of images that can be downloaded in a single scraping. This occurs because the browser only load 1 page of results.
