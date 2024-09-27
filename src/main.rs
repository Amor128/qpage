mod model;

use crate::model::Config;
use clap::{Arg, ArgMatches, Command};
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

const ARTIFACTS_RELEASE_URL: &str = "http://172.31.3.252:8082";
const QINCE_PAGE_RELEASE_PATH: &str = "/qince/?C=M;O=D";

mod options {
    pub const TERM: &str = "term";
    pub const MODULE: &str = "module";
    pub const VERSION: &str = "version";
    pub const APP: &str = "app";
    pub const URL: &str = "url";
}

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let config_file_path = home_dir.join(".config/qcl/config.toml");
    let config: Config = toml::from_str(
        &std::fs::read_to_string(config_file_path.as_path()).expect("Failed to read config file"),
    )
    .expect("Failed to parse config file");
    config
});

fn main() -> Result<(), std::io::Error> {
    let matches: ArgMatches = command().get_matches();
    let url: Option<&String> = matches.get_one::<String>(options::URL);
    let term: Option<&String> = matches.get_one::<String>(options::TERM);
    let module: Option<&String> = matches.get_one::<String>(options::MODULE);
    let version: Option<&String> = matches.get_one::<String>(options::VERSION);
    let app: Option<&String> = matches.get_one::<String>(options::APP);

    if url.is_some() {
        execute_url(url, app)
    } else {
        execute_locator(term, module, version, app)
    }
    .expect("Failed to execute command with given arguments");
    Ok(())
}

fn command() -> Command {
    Command::new("qpage")
        .about("all things pages")
        .arg(
            Arg::new(options::APP)
                .short('a')
                .help("local app name")
                .required(true),
        )
        .arg(Arg::new(options::TERM).short('t').help("web, h5, dinghuo"))
        .arg(
            Arg::new(options::MODULE)
                .short('m')
                .help("name of page build output"),
        )
        .arg(
            Arg::new(options::VERSION)
                .short('v')
                .help("version of page"),
        )
        .arg(Arg::new(options::URL).short('u').help("url of page"))
}

fn execute_url(
    url: Option<&String>,
    app: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if url.is_none() {
        panic!("url is required");
    }
    let page_file_path: Vec<PathBuf> = download_page_and_extract_html_file(url.unwrap())?;
    install_page(&page_file_path, app.unwrap())?;
    Ok(())
}

fn execute_locator(
    term: Option<&String>,
    module: Option<&String>,
    version: Option<&String>,
    app: Option<&String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // check if term, module and version are provided
    if term.is_none() || module.is_none() || version.is_none() {
        panic!("term, module and version are required");
    }
    let html_text = fetch_html_text(&format!(
        "{}{}",
        ARTIFACTS_RELEASE_URL, QINCE_PAGE_RELEASE_PATH
    ))?;
    let page_url = extract_page_url(&html_text, term.unwrap(), module.unwrap(), version.unwrap())?;
    execute_url(Some(&page_url), app)?;
    Ok(())
}

fn fetch_html_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let body = response.text()?;
    Ok(body)
}

fn extract_page_url(
    html_text: &str,
    term: &str,
    module: &str,
    version: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(html_text);
    let selector = scraper::Selector::parse("a").unwrap();
    let mut url = String::new();
    for element in document.select(&selector) {
        let href = element.value().attr("href").unwrap();
        if href.contains(term) && href.contains(module) && href.contains(version) {
            url = href.to_string();
            break;
        }
    }
    Ok(url)
}

fn download_page_and_extract_html_file(
    url: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let cache_dir = dirs::data_local_dir().unwrap().join("qpage");
    std::fs::create_dir_all(&cache_dir)?;

    let mut response = reqwest::blocking::get(url)?;
    let tar_gz_file_name = url.split("/").last().unwrap();
    let mut tar_gz_file: File = File::create(cache_dir.join(tar_gz_file_name))?;
    copy(&mut response, &mut tar_gz_file)?;

    let tar_gz = File::open(cache_dir.join(tar_gz_file_name))?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);

    let mut res: Vec<PathBuf> = Vec::new();
    for entry in archive.entries()? {
        let mut entry = entry?;
        // if is file
        if entry.header().entry_type().is_file() {
            let mut file = File::create(cache_dir.join(entry.path()?.file_name().unwrap()))?;
            std::io::copy(&mut entry, &mut file)?;
            res.push(cache_dir.join(entry.path()?.file_name().unwrap()));
        }
    }

    Ok(res)
}

fn install_page(
    page_file_paths: &Vec<PathBuf>,
    app: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let app_config = CONFIG
        .apps
        .iter()
        .find(|a| a.name == app)
        .expect("App not found");

    for page_file_path in page_file_paths {
        let page_file_name = page_file_path.file_name().unwrap().to_str().unwrap();
        if page_file_name.contains("h5") {
            let src_target_dir_path =
                PathBuf::from(&app_config.home_path).join("src/main/webapp/sysapp/h5");
            let target_target_dir_path =
                PathBuf::from(&app_config.exploed_war_path).join("sysapp/h5");
            std::fs::copy(page_file_path, src_target_dir_path.join(page_file_name))?;
            std::fs::copy(page_file_path, target_target_dir_path.join(page_file_name))?;
        } else {
            let src_target_dir_path =
                PathBuf::from(&app_config.home_path).join("src/main/webapp/sysapp/react/web");
            let target_target_dir_path =
                PathBuf::from(&app_config.exploed_war_path).join("sysapp/react/web");
            std::fs::copy(page_file_path, src_target_dir_path.join(page_file_name))?;
            std::fs::copy(page_file_path, target_target_dir_path.join(page_file_name))?;
        }
    }
    Ok(())
}
