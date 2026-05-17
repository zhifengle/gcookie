use clap::{arg, crate_version, ArgMatches, Command};
use gcookie::browser::cookie::{OutputFormat, SiteCookie};
use gcookie::browser::{get_cookie_entries_for_query, Browser, CookieQuery};
use std::{error::Error, path::PathBuf};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
fn run() -> MyResult<()> {
    let matches = build_app().get_matches();
    let query = build_cookie_query(&matches)?;
    let format = get_output_format(&matches)?;
    let cookies = get_cookie_entries_for_query(&query)?;

    print!("{}", SiteCookie::from(cookies).format(format)?);
    Ok(())
}

fn build_cookie_query(matches: &ArgMatches) -> MyResult<CookieQuery> {
    let site = matches.get_one::<String>("site").unwrap();

    if let Some(firefox) = matches.get_one::<PathBuf>("firefox") {
        return Ok(CookieQuery::new(Browser::Firefox, site).with_profile_path(firefox.clone()));
    }

    if let Some(chrome_path) = matches.get_one::<PathBuf>("chrome_path") {
        return Ok(CookieQuery::new(Browser::Chrome, site).with_profile_path(chrome_path.clone()));
    }

    let browser = matches.get_one::<String>("chrome").unwrap();
    Ok(CookieQuery::new(browser.parse::<Browser>()?, site))
}

fn get_output_format(matches: &ArgMatches) -> MyResult<OutputFormat> {
    Ok(matches
        .get_one::<String>("format")
        .unwrap()
        .parse::<OutputFormat>()?)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn matches(args: &[&str]) -> ArgMatches {
        build_app().try_get_matches_from(args).unwrap()
    }

    #[test]
    fn builds_default_chrome_query() {
        let matches = matches(&["gcookie", "example.com"]);
        let query = build_cookie_query(&matches).unwrap();

        assert_eq!(query, CookieQuery::new(Browser::Chrome, "example.com"));
    }

    #[test]
    fn builds_named_browser_query() {
        let matches = matches(&["gcookie", "-c", "Edge", "bing.com"]);
        let query = build_cookie_query(&matches).unwrap();

        assert_eq!(query, CookieQuery::new(Browser::Edge, "bing.com"));
    }

    #[test]
    fn builds_firefox_profile_query() {
        let matches = matches(&["gcookie", "-f", "profile-dir", "mozilla.org"]);
        let query = build_cookie_query(&matches).unwrap();

        assert_eq!(
            query,
            CookieQuery::new(Browser::Firefox, "mozilla.org")
                .with_profile_path(PathBuf::from("profile-dir"))
        );
    }

    #[test]
    fn builds_chrome_profile_query() {
        let matches = matches(&["gcookie", "-p", "profile-dir", "google.com"]);
        let query = build_cookie_query(&matches).unwrap();

        assert_eq!(
            query,
            CookieQuery::new(Browser::Chrome, "google.com")
                .with_profile_path(PathBuf::from("profile-dir"))
        );
    }

    #[test]
    fn reads_output_format() {
        let matches = matches(&["gcookie", "--format", "json", "example.com"]);

        assert_eq!(get_output_format(&matches).unwrap(), OutputFormat::Json);
    }
}
