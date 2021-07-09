extern crate reqwest;
use std::collections::HashMap;
use regex::Regex;

pub fn get_ip4_json() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://api.ipify.org?format=json")?
        .json::<HashMap<String, String>>()?;
    Ok(body)
}

pub fn get_ip4_string() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://api.ipify.org")?.text()?;
    Ok(body)
}

pub fn get_ip4_i32() -> Result<i32, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://api.ipify.org")?.text()?;
    let result = ipv4_to_int(body);
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

pub fn get_ip_i128() -> Result<i128, Box<dyn std::error::Error>> {
    let result : i128;
    let body = reqwest::blocking::get("https://api.ipify.org")?.text()?;
    let re = Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    if re.is_match(body.as_str()){
        result = ipv4_to_int(body) as i128;
    }else{
        result = ipv6_to_int(body);
    }
    Ok(result)
}


pub fn ipv4_to_int(ip :String) -> i32 {
    let mut result : i32 = 0;
    let splitted_ip : Vec<&str> = ip.split('.').collect();
    if splitted_ip.len() == 4 {
        let first_block : i32 = splitted_ip[0].parse::<i32>().unwrap();
        let second_block : i32 = splitted_ip[1].parse::<i32>().unwrap();
        let third_block : i32 = splitted_ip[2].parse::<i32>().unwrap();
        let fourth_block : i32 = splitted_ip[3].parse::<i32>().unwrap();       
        
        result = (first_block<<24)+(second_block<<16)+(third_block<<8)+(fourth_block<<0);
    }
    result
}

pub fn ipv6_to_int(ip :String) -> i128 {
    let mut result : i128 = 0;
    let splitted_ip : Vec<&str> = ip.split('.').collect();

    let i : i32 = 7;
    for token in splitted_ip.iter() {
        let token_int = i128::from_str_radix(token, 16);
        match token_int{
            Ok(o) => {result += o<<16*i;},
            Err(_e) => {},
        }
    }
    result
}
