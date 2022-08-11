use serde::Deserialize;
use std::fs;

pub fn get_secrets() -> Secrets {
    toml::from_str(&fs::read_to_string("secrets.toml").unwrap()).unwrap()
}

#[derive(Deserialize)]
pub struct Secrets {
    pub cookie_source_url: String,
    pub cookie_name: String,
    pub api_root: String,
}
