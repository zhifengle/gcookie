use rusqlite::{Connection, Result as SqlResult, Row};
use std::path::PathBuf;

use super::cookie::{Cookie, SiteCookie};
pub struct Firefox {
    profile_path: PathBuf,
}

impl Firefox {
    pub fn new(path: PathBuf) -> Self {
        Self { profile_path: path }
    }
    pub fn get_site_cookie(&self, host: &str) -> SqlResult<String> {
        Ok(SiteCookie::from(self.get_site_cookies(host)?).to_string())
    }

    pub fn get_site_cookies(&self, host: &str) -> SqlResult<Vec<Cookie>> {
        let path = self.profile_path.join("cookies.sqlite");
        let conn = Connection::open(&path).expect(&format!(
            "invalid cookie path: {}",
            path.display().to_string()
        ));
        let statement = format!(
            "SELECT host, path, name, value, isSecure, isHttpOnly, expiry, sameSite FROM moz_cookies where host = '{host}' or host = '.{host}'"
        );

        let mut stmt = conn.prepare(&statement)?;
        let rows = stmt.query_map([], |row: &Row| {
            Ok(Cookie {
                domain: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                value: row.get(3)?,
                secure: row.get::<_, i64>(4)? != 0,
                http_only: row.get::<_, i64>(5)? != 0,
                expires: row.get(6)?,
                same_site: row.get(7)?,
                encrypted_value: vec![],
            })
        })?;
        let mut cookies = Vec::new();
        for cookie in rows {
            if cookie.is_err() {
                continue;
            }
            let cookie = cookie?;
            cookies.push(cookie);
        }
        Ok(cookies)
    }
}

#[cfg(target_os = "windows")]
#[test]
fn firefox_connect_sql_ok() {
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
    let firefox = Firefox {
        profile_path: profile,
    };
    let res = firefox.get_site_cookie("bing.com");
    assert!(res.is_ok());
}
