use rusqlite::{Connection, Result as SqlResult, Row};
use std::{convert::TryInto, path::PathBuf};

use crate::cookie::{Cookie, SiteCookie};

const KEY_LEN: usize = 16;

pub struct Chromium {
    pub name: String,
    profile_path: PathBuf,
}

impl From<&str> for Chromium {
    fn from(name: &str) -> Self {
        let home_dir = dirs::home_dir().unwrap();
        match name {
            "Chrome" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join(".config/google-chrome/Default"),
            },
            "Chrome Beta" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join(".config/google-chrome-beta/Default"),
            },
            "Chromium" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join(".config/chromium/Default"),
            },
            "Edge" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join(".config/microsoft-edge/Default"),
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
    pub fn get_key(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut key: Vec<u8> = Vec::with_capacity(KEY_LEN);
        let pw = b"peanuts";
        let salt = b"saltysalt";
        pbkdf2::pbkdf2::<hmac::Hmac<sha1::Sha1>>(pw, salt, 1, &mut key);
        Ok(key)
    }
    pub fn get_site_cookie(&self, host: &str) -> SqlResult<String> {
        let path = self.profile_path.join("Cookies");
        let conn = Connection::open(&path).expect(&format!(
            "invalid cookie path: {}",
            path.display().to_string()
        ));
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
            cookie.value =
                String::from_utf8(aes128_cbc_decrypt(&cookie.encrypted_value[3..], &key))
                    .expect(&format!("parse cookie: {} err", cookie.name));
            site_cookie.push(cookie);
        }
        Ok(site_cookie.to_string())
    }
}

fn aes128_cbc_decrypt(encrypted: &[u8], key: &[u8]) -> Vec<u8> {
    let iv: [u8; 16] = [32; KEY_LEN];
    let cipher = libaes::Cipher::new_128(key.try_into().unwrap());
    cipher.cbc_decrypt(&iv, encrypted)
}
