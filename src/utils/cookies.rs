use std::{collections::HashMap, env};
use axum::http::HeaderMap;

pub struct Cookies {}

impl Cookies {
    pub fn from(headers: &HeaderMap) -> HashMap<String, String> {
        let mut cookies = HashMap::new();

        if let Some(cookie_header) = headers.get("Cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let mut parts = cookie.trim().split('=');
                    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                        cookies.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }

        cookies
    }

    pub fn new(key: &str, value: &str) -> String {
        format!(
            "{}={}; Path=/; Domain={}; HttpOnly; Secure; SameSite=None",
            key,
            value,
            env::var("DOMAIN_NAME").unwrap()
        )
    }
}