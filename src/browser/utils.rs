pub fn get_site(site: &str) -> Result<String, url::ParseError> {
    if site.starts_with("http") {
        let url_obj = url::Url::parse(&site)?;
        Ok(url_obj.host_str().unwrap().to_string())
    } else {
        Ok(site.to_string())
    }
}