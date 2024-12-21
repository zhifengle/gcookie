use rusqlite::{Connection, Result as SqlResult, Row};
use std::fs::remove_file;
use std::path::PathBuf;
use base64::{engine::general_purpose, Engine as _};
use sha2::{Sha256, Digest};


use super::cookie::{Cookie, SiteCookie};
use crate::windows::{
    aes_gcm_decrypt, crypt_unprotect_data, is_elevated, rawcopy, release_file_lock,
};

pub struct Chromium {
    pub name: String,
    profile_path: PathBuf,
}

impl From<&str> for Chromium {
    fn from(name: &str) -> Self {
        let home_dir = dirs::home_dir().unwrap();
        match name.to_lowercase().as_ref() {
            "chrome" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join("AppData/Local/Google/Chrome/User Data/Default/"),
            },
            "chrome beta" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join("AppData/Local/Google/Chrome Beta/User Data/Default/"),
            },
            "chromium" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join("AppData/Local/Chromium/User Data/Default/"),
            },
            "edge" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join("AppData/Local/Microsoft/Edge/User Data/Default/"),
            },
            _ => panic!("invalid browser"),
        }
    }
}

impl Chromium {
    pub fn new(path: PathBuf) -> Self {
        Self {
            name: "Chrome".to_string(),
            profile_path: path,
        }
    }
    pub fn is_v10(&self) -> bool {
        let file = std::fs::File::open(self.profile_path.join("../").join("Local State")).expect("cannot open Local State");
        let json: serde_json::Value =
            serde_json::from_reader(file).expect("Local State should be JSON");
        let v = &json["os_crypt"]["encrypted_key"];
        let app_bound_encrypted_key = &json["os_crypt"]["app_bound_encrypted_key"];
        println!("{:?}", app_bound_encrypted_key);
        return !v.is_null() && app_bound_encrypted_key.is_null();
    }
    pub fn get_key(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(self.profile_path.join("../").join("Local State"))?;
        let json: serde_json::Value =
            serde_json::from_reader(file).expect("Local State should be JSON");
        let v = &json["os_crypt"]["encrypted_key"];
        let v = general_purpose::STANDARD.decode(v.as_str().unwrap())?;
        Ok(crypt_unprotect_data(&v[5..])?)
    }
    pub fn get_app_bound_encrypted_key(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(self.profile_path.join("../").join("Local State"))?;
        let json: serde_json::Value = serde_json::from_reader(file)?;
        let app_bound_encrypted_key = json["os_crypt"]["app_bound_encrypted_key"].as_str().unwrap();
        if !app_bound_encrypted_key.starts_with("APPB") {
            return Err("invalid app_bound_encrypted_key".into());
        }
        let v = general_purpose::STANDARD.decode(&app_bound_encrypted_key[..])?;
        Ok(crypt_unprotect_data(&v)?)
    }
    pub fn get_cookies_connection(&self) -> SqlResult<Connection> {
        let path = self.profile_path.join("Network/Cookies");
        if !path.exists() {
            return Err(rusqlite::Error::InvalidPath(path));
        }
        if is_elevated() {
            let tmp_cookie_path = self.get_temp_cookies_path(&path);
            return Connection::open(&tmp_cookie_path);
        }
        unsafe {
            release_file_lock(path.as_os_str().to_str().unwrap());
        }
        let conn_result = Connection::open(&path);
        if conn_result.is_ok() {
            return conn_result;
        }
        let err = conn_result.unwrap_err();
        if let rusqlite::Error::SqliteFailure(e, _) = err {
            if rusqlite::ErrorCode::CannotOpen == e.code {
                if !is_elevated() {
                    return Err(rusqlite::Error::SqliteFailure(
                        e,
                        Some("Browser has locked cookie, please run as administrator".to_string()),
                    ));
                }
                let tmp_cookie_path = self.get_temp_cookies_path(&path);
                return Connection::open(&tmp_cookie_path);
            }
        }
        Err(err)
    }
    fn get_temp_cookies_path(&self, path: &PathBuf) -> PathBuf {
        let tmp_dir = std::env::temp_dir();
        let p = path.as_os_str().to_str().unwrap();
        let tmp_cookie_path = tmp_dir.join("Cookies");
        if tmp_cookie_path.exists() {
            remove_file(&tmp_cookie_path).unwrap();
        }
        // need administrator permission
        rawcopy(p, tmp_dir.as_os_str().to_str().unwrap()).unwrap();
        tmp_cookie_path
    }
    pub fn get_site_cookie(&self, host: &str) -> SqlResult<String> {
        let conn = self.get_cookies_connection()?;

        let key = self.get_key().expect("cannot get key");
        let statement = format!("SELECT host_key, path, name, value, encrypted_value FROM cookies where host_key = '{host}' or host_key = '.{host}'");

        let mut stmt = conn.prepare(&statement)?;
        let rows = stmt.query_map([], |row: &Row| {
            Ok(Cookie {
                host: row.get(0)?,
                path: row.get(1)?,
                name: row.get(2)?,
                value: row.get(3)?,
                encrypted_value: row.get(4)?,
            })
        })?;
        let mut site_cookie = SiteCookie::new();
        for cookie in rows {
            if cookie.is_err() {
                continue;
            }
            let mut cookie = cookie?;
            let hash = Sha256::digest(cookie.host.as_bytes());
            let value = &cookie.encrypted_value[15..];
            let nonce = &cookie.encrypted_value[3..15];
            let value = aes_gcm_decrypt(value, &key, nonce);
            if value.starts_with(hash.as_ref()) {
                cookie.value = String::from_utf8(value[hash.len()..].to_vec())
                    .expect(&format!("parse hash cookie: {} err", cookie.name));
            } else {
                cookie.value = String::from_utf8(value)
                    .expect(&format!("parse cookie: {} err", cookie.name));
            }
            site_cookie.push(cookie);
        }
        Ok(site_cookie.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_key_ok() {
        let edge: Chromium = "Edge".into();
        assert!(edge.get_key().is_ok());
        let chrome: Chromium = "Chrome".into();
        assert!(chrome.get_key().is_ok());
    }
    #[test]
    fn is_v10_ok() {
        // let p = PathBuf::from(r"C:\Users\xxx\Documents\test\test-chrome\profiles\Default\");
        // let chrome: Chromium = Chromium::new(p);
        // assert!(chrome.is_v10());
    }
    #[test]
    fn chrome_connect_sql_ok() {
        let chrome: Chromium = "Chrome".into();
        let res = chrome.get_site_cookie("example.com");
        assert!(res.is_ok());
        // println!("{}", res.unwrap());
    }
    #[test]
    fn edge_get_app_bound_encrypted_key_ok() {
        let chrome: Chromium = "edge".into();
        let key = chrome.get_app_bound_encrypted_key().unwrap();
        println!("{:?}", key);
    }
    #[test]
    fn edge_connect_sql_ok() {
        let chrome: Chromium = "edge".into();
        let res = chrome.get_site_cookie("bing.com");
        assert!(res.is_ok());
        // println!("{}", res.unwrap());
    }
}
