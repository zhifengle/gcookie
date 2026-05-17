use std::{error::Error, path::PathBuf};
pub mod cookie;
mod provider;
pub use provider::{
    get_cookie_entries as get_cookie_entries_for_query, Browser, CookieProvider, CookieQuery,
};
pub mod utils;

mod rookie_utils;
pub use rookie_utils::*;

#[cfg(target_os = "windows")]
mod chromium_windows;
#[cfg(target_os = "windows")]
pub use chromium_windows::*;

mod firefox;

pub mod gcookie_utils;

pub fn gcookie_chrome_by_path(site: &str, path: &PathBuf) -> Result<String, Box<dyn Error>> {
    Ok(cookie::SiteCookie::from(gcookie_chrome_entries_by_path(site, path)?).to_string())
}

pub fn gcookie_chrome_entries_by_path(
    site: &str,
    path: &PathBuf,
) -> Result<Vec<cookie::Cookie>, Box<dyn Error>> {
    let query = CookieQuery::new(Browser::Chrome, site).with_profile_path(path.clone());
    provider::get_cookie_entries(&query)
}
