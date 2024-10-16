use super::{firefox::Firefox, utils::get_site};
use std::{error::Error, path::PathBuf};

/// Get cookie from site by Chromium. Only working in Windows.
///
/// # Examples
///
/// ```no_run
/// let site = "http://cn.bing.com";
///  
/// let cookie =  match gcookie::gcookie_chrome(site, None, None) {
///     Ok(cookie) => cookie,
///     Err(err) => panic!("An error occurred when get cookie '{}': {}", site, err),
/// };
/// ```
#[cfg(target_os = "windows")]
pub fn gcookie_chrome(
    site: &str,
    browser: Option<&str>,
    chrome_path: Option<&PathBuf>,
) -> Result<String, Box<dyn Error>> {
    use super::Chromium;

    let os = std::env::consts::OS;
    // if !(os == "windows" || os == "linux") {
    if os != "windows" {
        panic!("Chrome not supported in {}", os);
    }
    let site = get_site(site)?;
    let chromium = match chrome_path {
        Some(path) => Chromium::new(PathBuf::from(path)),
        None => browser.unwrap_or("Chrome").into(),
    };
    Ok(chromium.get_site_cookie(&site)?)
}

/// Get cookie from site by Firefox.
///
/// # Examples
///
/// ```no_run
/// let site = "http://cn.bing.com";
///
/// let mut path = PathBuf::new();
/// path.push(r"C:\my_firefox\profile");
///
/// let cookie =  match gcookie::gcookie_firefox(site, &path) {
///     Ok(cookie) => cookie,
///     Err(err) => panic!("An error occurred when get cookie '{}': {}", site, err),
/// };
/// ```
pub fn gcookie_firefox(site: &str, path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let site = get_site(site)?;
    let firefox = Firefox::new(PathBuf::from(path));
    Ok(firefox.get_site_cookie(&site)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcookie_chrome_ok() {
        let site = "http://cn.bing.com";

        let cookie = gcookie_chrome(site, None, None);
        assert!(cookie.is_ok());
    }
    #[test]
    fn gcookie_chrome_edge_ok() {
        let site = "bing.com";
        let browser = Some("Edge");

        let cookie = gcookie_chrome(site, browser, None);
        assert!(cookie.is_ok());
    }
}