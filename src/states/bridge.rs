extern crate serde_json;
use std::fs;

pub fn read_file(dir: &'static str) -> String {
    fs::read_to_string(dir).expect("Error reading ip.")
}

pub fn url(ip: String, key: String) -> String {
    format!("http://{}/api/{}/lights/", ip, key)
}

pub fn http_get(url: String) -> ureq::SerdeValue {
    let get = ureq::get(&url).call(); // calls the api
    let data = get.into_json().unwrap(); // saves data (ip)
    data
}

pub fn data(ip: &'static str, key: &'static str) -> serde_json::value::Value {
    let ip = read_file(ip);
    let key = read_file(key);
    http_get(url(ip, key))
}