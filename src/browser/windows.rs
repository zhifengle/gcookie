use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use rusqlite::{Connection, Result as SqlResult, Row};
use std::ffi::c_void;
use std::fs::remove_file;
use std::path::PathBuf;
use windows::Win32::Foundation::{CloseHandle, LocalFree, HANDLE, HLOCAL};
use windows::Win32::Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

use crate::cookie::{Cookie, SiteCookie};
use crate::copy::rawcopy;

fn is_elevated() -> bool {
    let mut result = false;
    let mut handle: HANDLE = HANDLE(0);
    unsafe {
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle).is_ok() {
            let elevation = TOKEN_ELEVATION::default();
            let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            let mut ret_size = size;
            let raw_ptr = &elevation as *const _ as *mut c_void;
            if GetTokenInformation(
                handle,
                TokenElevation.into(),
                Some(raw_ptr),
                size,
                &mut ret_size,
            )
            .is_ok()
            {
                result = elevation.TokenIsElevated != 0;
            }
        }
        let _ = CloseHandle(handle);
    }
    result
}

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
                profile_path: home_dir.join("AppData/Local/Google/Chromium/User Data/Default/"),
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
    pub fn get_key(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(self.profile_path.join("../").join("Local State"))?;
        let json: serde_json::Value =
            serde_json::from_reader(file).expect("Local State should be JSON");
        let v = &json["os_crypt"]["encrypted_key"];
        let v = base64::decode(v.as_str().unwrap()).unwrap();
        Ok(crypt_unprotect_data(&v[5..])?)
    }
    pub fn get_cookies_connection(&self) -> SqlResult<Connection> {
        let path = self.profile_path.join("Network/Cookies");
        if !path.exists() {
            return Err(rusqlite::Error::InvalidPath(path));
        }
        let tmp_dir = std::env::temp_dir();
        let mut tmp_cookie: Option<PathBuf> = None;
        let conn_result = Connection::open(&path);
        if conn_result.is_ok() {
            return conn_result;
        }
        let err = conn_result.unwrap_err();
        if let rusqlite::Error::SqliteFailure(e, Some(_)) = err {
            if rusqlite::ErrorCode::CannotOpen == e.code {
                if !is_elevated() {
                    return Err(rusqlite::Error::SqliteFailure(
                        e,
                        Some("Browser has locked cookie, please run as administrator".to_string()),
                    ));
                }
                let p = path.as_os_str().to_str().unwrap();
                let tmp_cookie_path = tmp_dir.join("Cookies");
                if tmp_cookie_path.exists() {
                    remove_file(&tmp_cookie_path).unwrap();
                }
                rawcopy(p, tmp_dir.as_os_str().to_str().unwrap()).unwrap();
                tmp_cookie = Some(tmp_cookie_path);
            }
        }
        Connection::open(&tmp_cookie.unwrap())
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
            let value = &cookie.encrypted_value[15..];
            let nonce = &cookie.encrypted_value[3..15];
            cookie.value = String::from_utf8(aes_gcm_decrypt(value, &key, nonce))
                .expect(&format!("parse cookie: {} err", cookie.name));
            site_cookie.push(cookie);
        }
        Ok(site_cookie.to_string())
    }
}

fn crypt_unprotect_data(crypted_bytes: &[u8]) -> windows::core::Result<Vec<u8>> {
    let len = crypted_bytes.len();
    let mut bytes = Vec::from(crypted_bytes);
    let pb = bytes.as_mut_ptr();
    let mut blob = CRYPT_INTEGER_BLOB {
        pbData: pb,
        cbData: len as u32,
    };
    let mut out = Vec::with_capacity(len);
    let mut blob_out = CRYPT_INTEGER_BLOB {
        pbData: out.as_mut_ptr(),
        cbData: out.len() as u32,
    };
    unsafe {
        CryptUnprotectData(&mut blob, None, None, None, None, 0, &mut blob_out).ok();

        let slice = std::slice::from_raw_parts(blob_out.pbData, blob_out.cbData as usize);
        // LocalFree(blob.pbData as isize);
        LocalFree(HLOCAL(blob_out.pbData.cast()));
        Ok(slice.to_vec())
    }
}

fn aes_gcm_decrypt(value: &[u8], key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce, value)
        .expect("decryption aes_gcm value failure!");
    plaintext
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
    fn chrome_connect_sql_ok() {
        let chrome: Chromium = "Chrome".into();
        let res = chrome.get_site_cookie("example.com");
        assert!(res.is_ok());
        // println!("{}", res.unwrap());
    }
    #[test]
    fn is_elevated_ok() {
        // run with admin
        println!("---- run with admin ----- {}", is_elevated());
        // assert!(is_elevated());
    }
}
