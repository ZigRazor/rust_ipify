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

pub fn get_ip4_i64() -> Result<i64, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://api.ipify.org")?.text()?;
    let result = ip_to_int(body);
    Ok(result)
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

fn ip_to_int(ip :String) -> i64 {
    let mut result : i64 = 0;
    let splitted_ip : Vec<&str> = ip.split('.').collect();
    if splitted_ip.len() == 4 {
        let first_block : i64 = splitted_ip[0].parse::<i64>().unwrap();
        let second_block : i64 = splitted_ip[1].parse::<i64>().unwrap();
        let third_block : i64 = splitted_ip[2].parse::<i64>().unwrap();
        let fourth_block : i64 = splitted_ip[3].parse::<i64>().unwrap();       
        
        result = (first_block<<24)+(second_block<<16)+(third_block<<8)+(fourth_block<<0);
    }
    result
}
