use clap::{arg, crate_version, Command};
use gcookie::browser::cookie::{OutputFormat, SiteCookie};
use gcookie::browser::{
    gcookie_chrome_entries_by_path, get_cookie_entries, get_firefox_cookie_entries_by_path,
};
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
    let format = matches
        .get_one::<String>("format")
        .unwrap()
        .parse::<OutputFormat>()?;
    let firefox = matches.get_one::<PathBuf>("firefox");
    if let Some(firefox) = firefox {
        let cookies = get_firefox_cookie_entries_by_path(site, firefox)?;
        print!("{}", SiteCookie::from(cookies).format(format)?);
        return Ok(());
    }
    let chrome_path = matches.get_one::<PathBuf>("chrome_path");
    if let Some(p) = chrome_path {
        let cookies = gcookie_chrome_entries_by_path(site, p)?;
        print!("{}", SiteCookie::from(cookies).format(format)?);
        return Ok(());
    }
    let browser = matches.get_one::<String>("chrome").map(|s| s.as_str());
    let cookies = get_cookie_entries(browser.unwrap(), site)?;
    print!("{}", SiteCookie::from(cookies).format(format)?);
    Ok(())
}

fn build_app() -> Command {
    let app = Command::new("gcookie")
        .version(crate_version!())
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
        .arg(
            arg!(--format <format> "Output format: string, json or netscape")
                .default_value("string")
                .value_parser(["string", "json", "netscape"]),
        )
        .arg(arg!(<site> "URL of the site or host of the site"));

    app
}
