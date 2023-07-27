# google_images_scraper-rs

This is a command-line app that scrapes the results of a Google Images search.<br>
The app uses a Selenium WebDriver to render the search results into a browser, this is necessary because the searched images are only loaded when the request is interpreted by a browser.<br>
With that being said, the program needs a `selenium-server.jar` file stored in `.google_images_scraper` folder on `Home` directory. You can download it [**here**](https://www.selenium.dev/downloads/).<br>
After the download, just create the `.google_images_scraper` folder on your `Home` directory and then drop the .jar file.<br>

### Usage:

```
$ google_images_scraper-rs [SEARCH_PARAMETER] [OPTIONS]
```

Args:
- `search_parameter <string>`: (required) String that's going to be inserted on Google Images text input.
- OPTIONS:
  - `--download-limit <integer>`: (optional) Limit of downloads per scraping.
  - `--folder-name <string>`: (optional) Custom folder name with downloaded images.
