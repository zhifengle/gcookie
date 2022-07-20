use std::{error::Error, path::PathBuf};

use clap::{arg, Command};
type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
fn run() -> MyResult<()> {
    let app = build_app();
    // let matches = app.get_matches();
    let matches = app.get_matches_from(vec!["gcookie", "-f", "c:\\", "bgm.tv"]);
    let site = matches.get_one::<String>("site").unwrap();
    let firefox = matches.get_one::<PathBuf>("firefox");
    let chrome = matches.get_one::<String>("chrome").unwrap();
    println!("{}; {:?}; {:?}", site, firefox, chrome);

    Ok(())
}

fn build_app() -> Command<'static> {
    let app = Command::new("gcookie")
        .version("0.0.1")
        .about("get site cookie string")
        .arg(arg!(-c --chrome [chrome] "Chrome's name. Chrome, Chromium, Chrome Beta or Edge is OK.")
        // default
        .default_value("Chrome").conflicts_with("firefox"))
        .arg(arg!(firefox: -f --firefox [firefox] "path of firefox profile")
        .value_parser(clap::value_parser!(PathBuf))
        .conflicts_with("chrome"))
        .arg(arg!(<site> "URL of the site or host of the site"));

    app
}
