use super::utils::get_site;
use std::{error::Error, path::PathBuf};

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

/// Get cookie from site
///
/// # Examples
///
/// ```no_run
/// let site = "http://bing.com";
///  
/// let cookie =  match gcookie::get_cookies("chrome", site) {
///     Ok(cookie) => cookie,
///     Err(err) => panic!("An error occurred when get cookie '{}': {}", site, err),
/// };
/// ```
pub fn get_cookies(browser: &str, site: &str) -> Result<String, Box<dyn Error>> {
    let site = get_site(site)?;
    let domains = Some(vec![site]);
    let cookies = match browser.to_lowercase().as_str() {
        "firefox" => rookie::firefox(domains),
        "chrome" => rookie::chrome(domains),
        "chromium" => rookie::chromium(domains),
        "edge" => rookie::edge(domains),
        _ => {
            return Err(Box::from(format!(
                "Unsupported browser: {}; please use firefox, chrome, chromium or edge",
                browser
            )))
        }
    }?;

    Ok(gen_cookies_string(cookies))
}

/// Get cookie from site by Chrome with path.
///
/// # Examples
///
/// ```no_run
/// let site = "https://google.com";
///
/// let mut path = PathBuf::new();
/// path.push(r"C:\my_chrome\user data\default");
///
/// let cookie =  match gcookie::get_chrome_cookies_by_path(site, &path) {
///     Ok(cookie) => cookie,
///     Err(err) => panic!("An error occurred when get cookie '{}': {}", site, err),
/// };
/// ```
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

/// Get cookie from site by Firefox with path.
///
/// # Examples
///
/// ```no_run
/// let site = "https://www.mozilla.org/";
///
/// let mut path = PathBuf::new();
/// path.push(r"C:\my_firefox\profile");
///
/// let cookie =  match gcookie::get_firefox_cookies_by_path(site, &path) {
///     Ok(cookie) => cookie,
///     Err(err) => panic!("An error occurred when get cookie '{}': {}", site, err),
/// };
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rookie_chrome_ok() {
        let site = "google.com";
        let browser = "chrome";

        let cookie = get_cookies(browser, site);
        assert!(cookie.is_ok());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn rookie_edge_path_ok() {
        let site = "bing.com";
        let home_dir = dirs::home_dir().unwrap();
        let profile_path = home_dir.join("AppData/Local/Microsoft/Edge/User Data/Default/");
        let cookie = get_chrome_cookies_by_path(site, &profile_path);
        assert!(cookie.is_ok());
        // println!("{}", cookie.unwrap());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn rookie_firefox_windows_ok() {
        let site = "https://www.mozilla.org/";
        use std::fs;
        let profiles_path = dirs::home_dir()
            .unwrap()
            .join("AppData/Roaming/Mozilla/Firefox/Profiles/");
        let mut profile = profiles_path.clone();
        for (i, p) in fs::read_dir(profiles_path).unwrap().enumerate() {
            if i == 0 {
                profile = p.unwrap().path();
                break;
            }
        }
        let cookie = get_firefox_cookies_by_path(site, &profile);
        assert!(cookie.is_ok());
    }
}
