use clap::{arg, Command};
use gcookie::browser::{get_chrome_cookies_by_path, get_cookies, get_firefox_cookies_by_path};
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
