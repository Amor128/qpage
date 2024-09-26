use clap::{Arg, ArgGroup, ArgMatches, Command};

const ARTIFACTS_RELEASE_URL: &str = "http://172.31.3.252:8082";
const QINCE_PAGE_RELEASE_PATH: &str = "/qince/?C=M;O=D";
const DINGHUO_PAGE_RELEASE_PATH: &str = "/dinghuo/?C=M;O=D";

mod options {
    pub const TERM: &str = "term";
    pub const MODULE: &str = "module";
    pub const VERSION: &str = "version";
    pub const APP: &str = "app";
    pub const URL: &str = "url";
}


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
    }.expect("Failed to execute command with given arguments");
    Ok(())
}

fn read_zip(zip_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn command() -> Command {
    Command::new("qpage")
        .about("all things pages")
        .arg(
            Arg::new(options::TERM)
                .short('t')
        )
        .arg(
            Arg::new(options::MODULE)
                .short('m')
                .help("name of page build output")
        )
        .arg(
            Arg::new(options::VERSION)
                .short('v')
                .help("version of page")
        )
        .arg(
            Arg::new(options::URL)
                .short('u')
                .help("url of page")
        )
        .arg(
            Arg::new(options::APP)
                .short('a')
                .help("local app name")
                .required(true)
        )
        .group(ArgGroup::new("located_by_term_module_version")
            .args(&[options::TERM, options::MODULE, options::VERSION])
            .conflicts_with("located_by_url")
        ).group(ArgGroup::new("located_by_url")
        .args(&[options::URL])
        .conflicts_with("located_by_term_module_version"))
}

fn execute_url(url: Option<&String>, app: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn execute_locator(term: Option<&String>, module: Option<&String>, version: Option<&String>, app: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    // check if term, module and version are provided
    if term.is_none() || module.is_none() || version.is_none() {
        panic!("term, module and version are required");
    }
    let html_text = fetch_html_text(&format!("{}{}", ARTIFACTS_RELEASE_URL, QINCE_PAGE_RELEASE_PATH))?;
    let page_url = extract_page_url(&html_text, term.unwrap(), module.unwrap(), version.unwrap())?;
    execute_url(Some(&page_url), app)?;
    Ok(())
}

fn fetch_html_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let body = response.text()?;
    Ok(body)
}

fn extract_page_url(html_text: &str, term: &str, module: &str, version: &str) -> Result<String, Box<dyn std::error::Error>> {
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

