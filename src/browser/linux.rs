use rusqlite::Result as SqlResult;
use std::path::PathBuf;

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
        let a = vec![];
        Ok(a)
    }
    pub fn get_site_cookie(&self, host: &str) -> SqlResult<String> {
        println!("{:?}", self.profile_path);
        unimplemented!("{}", host)
    }
}
