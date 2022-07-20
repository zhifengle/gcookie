use rusqlite::Result as SqlResult;
use std::path::PathBuf;

pub struct Chromium {
    pub name: String,
    profile_path: PathBuf,
}

// @TODO
impl From<&str> for Chromium {
    fn from(name: &str) -> Self {
        let home_dir = dirs::home_dir().unwrap();
        match name {
            "Chrome" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join("AppData/Local/Google/Chrome/User Data/Default/"),
            },
            "Chrome Beta" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join("AppData/Local/Google/Chrome Beta/User Data/Default/"),
            },
            "Chromium" => Chromium {
                name: name.to_string(),
                profile_path: home_dir.join("AppData/Local/Google/Chromium/User Data/Default/"),
            },
            "Edge" => Chromium {
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
        let a = vec![];
        Ok(a)
    }
    pub fn get_site_cookie(&self, host: &str) -> SqlResult<String> {
        println!("{:?}", self.profile_path);
        unimplemented!("{}", host)
    }
}
