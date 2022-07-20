use std::{fmt, ops};

pub struct Cookie {
    pub host: String,
    pub path: String,
    pub name: String,
    pub value: String,
    pub encrypted_value: Vec<u8>,
}

pub struct SiteCookie(Vec<Cookie>);

impl SiteCookie {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl ops::Deref for SiteCookie {
    type Target = Vec<Cookie>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for SiteCookie {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for SiteCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        self.iter().for_each(|cookie| {
            str.push_str(&format!("{}={}; ", cookie.name, cookie.value));
        });
        str.pop();
        str.pop();
        write!(f, "{}", str)
    }
}
