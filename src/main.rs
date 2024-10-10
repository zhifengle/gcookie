use clap::{arg, Command};
use gcookie::get_site;
use std::{error::Error, path::PathBuf};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
fn run() -> MyResult<()> {
    let app = build_app();
    let matches = app.get_matches();
    let site = matches.get_one::<String>("site").unwrap();
    let firefox = matches.get_one::<PathBuf>("firefox");
    if firefox.is_some() {
        let res = get_firefox_cookies_by_path(site, firefox.unwrap())?;
        print!("{}", res);
        return Ok(());
    }
    let chrome_path = matches.get_one::<PathBuf>("chrome_path");
    if let Some(p) = chrome_path {
        let res = get_chrome_cookies_by_path(site, p)?;
        print!("{}", res);
        return Ok(());
    }
    let browser = matches.get_one::<String>("chrome").map(|s| s.as_str());
    let res = get_cookies(browser.unwrap(), site)?;
    print!("{}", res);
    Ok(())
}

fn build_app() -> Command {
    let app = Command::new("gcookie")
        .version("0.1.0")
        .about("get site cookie string")
        .arg(
            arg!(-c --chrome [chrome] "Browser's name. Chrome, Chromium, Edge or Firefox is OK.")
                .default_value("Chrome"),
        )
        .arg(
            arg!(chrome_path: -p --"chrome-path" [chrome_path] "the use data path of Chrome")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with("firefox"),
        )
        .arg(
            arg!(firefox: -f --firefox [firefox] "path of firefox profile")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(arg!(<site> "URL of the site or host of the site"));

    app
}

// cookies to string
fn gen_cookies_string(cookies: Vec<rookie::enums::Cookie>) -> String {
    let mut cookie_str = String::new();
    cookies.iter().for_each(|cookie| {
        cookie_str.push_str(&format!("{}={}; ", cookie.name, cookie.value));
    });
    cookie_str.pop();
    cookie_str.pop();
    cookie_str
}

fn get_cookies(browser: &str, site: &str) -> Result<String, Box<dyn Error>> {
    let site = get_site(site)?;
    let domains = Some(vec![site]);
    let cookies = match browser.to_lowercase().as_str() {
        "firefox" => rookie::firefox(domains),
        "chrome" => rookie::chrome(domains),
        "chromium" => rookie::chromium(domains),
        "edge" => rookie::edge(domains),
        _ => panic!(
            "Unsupported browser: {}; please use firefox, chrome, chromium or edge",
            browser
        ),
    }?;

    Ok(gen_cookies_string(cookies))
}

fn get_chrome_cookies_by_path(site: &str, path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let cookies_path = path.join("Network/Cookies");
    if !cookies_path.exists() {
        panic!("{} not exists", cookies_path.display());
    }
    let key_path = path.join("../Local State");
    let site = get_site(site)?;
    let domains = Some(vec![site]);
    let cookies = rookie::any_browser(
        cookies_path.to_str().unwrap(),
        domains,
        Some(key_path.to_str().unwrap()),
    )?;

    Ok(gen_cookies_string(cookies))
}

fn get_firefox_cookies_by_path(site: &str, path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let site = get_site(site)?;
    let cookies_path = path.join("cookies.sqlite");
    if !cookies_path.exists() {
        panic!("{} not exists", cookies_path.display());
    }
    let domains = Some(vec![site]);
    let cookies = rookie::any_browser(cookies_path.to_str().unwrap(), domains, None)?;

    Ok(gen_cookies_string(cookies))
}
