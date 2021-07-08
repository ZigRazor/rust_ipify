extern crate reqwest;
use std::collections::HashMap;

pub fn get_ip4_json() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://api.ipify.org?format=json")?
        .json::<HashMap<String, String>>()?;
    Ok(body)
}

pub fn get_ip4_string() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://api.ipify.org")?.text()?;
    Ok(body)
}

pub fn get_ip_json() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://api.ipify.org?format=json")?
        .json::<HashMap<String, String>>()?;
    Ok(body)
}

pub fn get_ip_string() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://api.ipify.org")?.text()?;
    Ok(body)
}
