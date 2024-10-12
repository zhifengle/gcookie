use std::{error::Error, path::PathBuf};

fn get_site(site: &str) -> Result<String, url::ParseError> {
    if site.starts_with("http") {
        let url_obj = url::Url::parse(&site)?;
        Ok(url_obj.host_str().unwrap().to_string())
    } else {
        Ok(site.to_string())
    }
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

pub fn get_cookies(browser: &str, site: &str) -> Result<String, Box<dyn Error>> {
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

pub fn get_chrome_cookies_by_path(site: &str, path: &PathBuf) -> Result<String, Box<dyn Error>> {
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

pub fn get_firefox_cookies_by_path(site: &str, path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let site = get_site(site)?;
    let cookies_path = path.join("cookies.sqlite");
    if !cookies_path.exists() {
        panic!("{} not exists", cookies_path.display());
    }
    let domains = Some(vec![site]);
    let cookies = rookie::any_browser(cookies_path.to_str().unwrap(), domains, None)?;

    Ok(gen_cookies_string(cookies))
}