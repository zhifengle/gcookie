use crate::cookie::{Cookie, SiteCookie};
use rusqlite::{Connection, Result as SqlResult, Row};
use std::path::PathBuf;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(other)]
mod other;
#[cfg(other)]
pub use self::other::*;

pub struct Firefox {
    profile_path: PathBuf,
}

impl Firefox {
    pub fn new(path: PathBuf) -> Self {
        Self { profile_path: path }
    }
    pub fn get_site_cookie(&self, host: &str) -> SqlResult<String> {
        let path = self.profile_path.join("cookies.sqlite");
        let conn = Connection::open(&path).expect(&format!(
            "invalid cookie path: {}",
            path.display().to_string()
        ));
        let statement = format!(
            "SELECT host, path, name, value FROM moz_cookies where host = '{host}' or host = '.{host}'"
        );

        let mut stmt = conn.prepare(&statement)?;
        let rows = stmt.query_map([], |row: &Row| {
            Ok(Cookie {
                host: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                value: row.get(3)?,
                encrypted_value: vec![],
            })
        })?;
        let mut site_cookie = SiteCookie::new();
        for cookie in rows {
            if cookie.is_err() {
                continue;
            }
            let cookie = cookie?;
            site_cookie.push(cookie);
        }
        Ok(site_cookie.to_string())
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
    let res = firefox.get_site_cookie("bgm.tv");
    assert!(res.is_ok());
}
