use std::{error::Error, path::PathBuf};
pub mod cookie;
pub mod utils;

mod rookie_utils;
pub use rookie_utils::*;

#[cfg(target_os = "windows")]
mod chromium_windows;
#[cfg(target_os = "windows")]
pub use chromium_windows::*;

mod firefox;

pub mod gcookie_utils;

#[cfg(target_os = "windows")]
pub fn gcookie_chrome_by_path(site: &str, path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let browser = Chromium::new(path.clone());
    if browser.is_v10() {
        return gcookie_utils::gcookie_chrome(site, None, Some(path));
    } else {
        return get_chrome_cookies_by_path(site, path);
    }
}

#[cfg(not(target_os = "windows"))]
pub fn gcookie_chrome_by_path(site: &str, path: &PathBuf) -> Result<String, Box<dyn Error>> {
    return get_chrome_cookies_by_path(site, path);
}
