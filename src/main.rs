use clap::{arg, Command};
use gcookie::{gcookie_chrome, gcookie_firefox};
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
        let res = gcookie_firefox(site, firefox.unwrap())?;
        print!("{}", res);
        return Ok(());
    }
    let chrome_path = matches.get_one::<PathBuf>("chrome_path");
    let chrome = matches.get_one::<String>("chrome").map(|s| s.as_str());
    let res = gcookie_chrome(site, chrome, chrome_path)?;
    print!("{}", res);
    Ok(())
}

fn build_app() -> Command<'static> {
    let app = Command::new("gcookie")
        .version("0.0.3")
        .about("get site cookie string")
        .arg(arg!(-c --chrome [chrome] "Chrome's name. Chrome, Chromium, Chrome Beta or Edge is OK.")
        .default_value("Chrome"))
        .arg(
            arg!(chrome_path: -p --"chrome-path" [chrome_path] "the use data path of Chrome")
            .value_parser(clap::value_parser!(PathBuf))
            .conflicts_with("firefox")
        )
        .arg(
            arg!(firefox: -f --firefox [firefox] "path of firefox profile")
                .value_parser(clap::value_parser!(PathBuf)
        )
        .conflicts_with("chrome_path"))
        .arg(arg!(<site> "URL of the site or host of the site"));

    app
}
