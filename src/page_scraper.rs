pub async fn scrape_cookie(source_url: &str, cookie_name: &str) -> Result<String, CookieError> {
    reqwest::get(source_url)
        .await
        .map_err(|_| CookieError::Website)
        .and_then(|mut r| {
            r.headers_mut()
                .iter()
                .find(|(name, value)| {
                    name == &"set-cookie"
                        && value.to_str().unwrap_or_default().starts_with(cookie_name)
                })
                .and_then(|(_, v)| v.to_str().map(|e| e.to_string()).ok())
                .and_then(|e| e.split(";").next().map(|s| s.to_string()))
                .ok_or(CookieError::Header)
        })
}

#[derive(Debug)]
pub enum CookieError {
    Website,
    Header,
}
