use super::cookie::Cookie;
use super::rookie_utils::RookieProvider;
use std::{error::Error, path::PathBuf, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Browser {
    Chrome,
    Chromium,
    Edge,
    Firefox,
}

impl Browser {
    pub fn as_name(&self) -> &'static str {
        match self {
            Self::Chrome => "Chrome",
            Self::Chromium => "Chromium",
            Self::Edge => "Edge",
            Self::Firefox => "Firefox",
        }
    }

    pub fn is_chromium_based(&self) -> bool {
        matches!(self, Self::Chrome | Self::Chromium | Self::Edge)
    }
}

impl FromStr for Browser {
    type Err = String;

    fn from_str(browser: &str) -> Result<Self, Self::Err> {
        match browser.to_lowercase().as_str() {
            "chrome" => Ok(Self::Chrome),
            "chromium" => Ok(Self::Chromium),
            "edge" => Ok(Self::Edge),
            "firefox" => Ok(Self::Firefox),
            _ => Err(format!(
                "Unsupported browser: {browser}; please use firefox, chrome, chromium or edge"
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CookieQuery {
    pub browser: Browser,
    pub site: String,
    pub profile_path: Option<PathBuf>,
}

impl CookieQuery {
    pub fn new(browser: Browser, site: impl Into<String>) -> Self {
        Self {
            browser,
            site: site.into(),
            profile_path: None,
        }
    }

    pub fn with_profile_path(mut self, profile_path: PathBuf) -> Self {
        self.profile_path = Some(profile_path);
        self
    }
}

pub trait CookieProvider {
    fn get_cookies(&self, query: &CookieQuery) -> Result<Vec<Cookie>, Box<dyn Error>>;
}

pub fn get_cookie_entries(query: &CookieQuery) -> Result<Vec<Cookie>, Box<dyn Error>> {
    if should_use_native_chromium_windows(query) {
        #[cfg(target_os = "windows")]
        {
            let provider = super::chromium_windows::NativeChromiumWindowsProvider;
            return provider.get_cookies(query);
        }
    }

    let provider = RookieProvider;
    provider.get_cookies(query)
}

#[cfg(target_os = "windows")]
fn should_use_native_chromium_windows(query: &CookieQuery) -> bool {
    if !query.browser.is_chromium_based() {
        return false;
    }

    let Some(profile_path) = &query.profile_path else {
        return false;
    };

    super::Chromium::new(profile_path.clone()).is_v10()
}

#[cfg(not(target_os = "windows"))]
fn should_use_native_chromium_windows(_query: &CookieQuery) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_browsers() {
        assert_eq!("chrome".parse::<Browser>().unwrap(), Browser::Chrome);
        assert_eq!("Chromium".parse::<Browser>().unwrap(), Browser::Chromium);
        assert_eq!("EDGE".parse::<Browser>().unwrap(), Browser::Edge);
        assert_eq!("firefox".parse::<Browser>().unwrap(), Browser::Firefox);
    }

    #[test]
    fn rejects_unsupported_browser() {
        assert!("safari".parse::<Browser>().is_err());
    }

    #[test]
    fn builds_query_with_profile_path() {
        let query =
            CookieQuery::new(Browser::Chrome, "example.com").with_profile_path("Default".into());

        assert_eq!(query.browser, Browser::Chrome);
        assert_eq!(query.site, "example.com");
        assert!(query.profile_path.is_some());
    }
}
