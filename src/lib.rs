pub mod browser;

#[cfg(target_os = "windows")]
pub mod windows;

pub use browser::get_cookies;
pub use browser::get_firefox_cookies_by_path;
pub use browser::get_chrome_cookies_by_path;