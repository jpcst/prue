// use std::any::Any;
use std::fs;
use std::io::{self, Write};

// use json::object::Object;
// use serde_json::{json, to_string, value};
use serde_json::json;
// use serde_json::Error;
extern crate serde_json;
extern crate ureq;
use std::io::prelude::*;
// use json::JsonValue::Object
// use serde::{Serialize, Deserialize};

fn pause() { // press any key to continue
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn send_get_request(url: &str) -> ureq::SerdeValue {
    let get = ureq::get(&url).call(); // calls the api
    let data = get.into_json().unwrap(); // saves data (ip)
    data
}

// #[derive(Debug)]
// enum InvalidData {}

// #[derive(Debug)]
// struct ApiError {
//     kind: InvalidData,
//     error: String,
// }

// fn send_get_request(url: &str) -> Result<ureq::SerdeValue, Error> {
//     let get = ureq::get(&url).call(); // calls the api
//     // let data = get.into_json().unwrap(); // saves data (ip)
//     // Ok(data)
//     let response = match get {
//         Ok(res) => res,
//         Err(err) => return Err(err),
//     };
//     let data = match response.into_json() {
//         Ok(json) => json,
//         Err(err) => return Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, format!("GET Request Error (wrong IP) {}", err))))
//     };
// }

// to-do: adicionar Result<String,error> em caso de too many requests
// thread 'main' panicked at src/main.rs:25:32:
// called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: "Failed to read JSON: EOF while parsing a value at line 1 column 0" }
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
fn find_ip() -> String { // verificar numero do array - automatizar
    let discovery_addr = "https://discovery.meethue.com/"; // adicionar too many requests - error handle
    let ip_addr = send_get_request(discovery_addr);
    //println!("{:?}",ip_addr);
    ip_addr[0]["internalipaddress"].to_string().replace("\"", "")
}

fn read_ip_file(path: String) -> io::Result<String> {
    match fs::read_to_string(&path) {
        Ok(ip_from_txt) => {
            println!("\nIP Path   : {}", path);
            println!("IP Output : {}", ip_from_txt);
            Ok(ip_from_txt)
        },
        Err(_) => {
            println!("\nFile 'ip.txt' Not Found");
            println!("IP Path   : {}", path);
            let mut file = fs::File::create(&path)?;
            let ip = find_ip();
            file.write_all(ip.as_bytes())?;
            println!("IP {} found and saved", ip);
            //println!("{:?}", find_ip);
            Ok(ip)
        }
    }
}

fn create_key(ip: String, name: &str) -> Result<ureq::SerdeValue, std::io::Error> {
    let url = format!("http://{}/api/", ip);
    let cmd = json!({
        "devicetype": name, 
    });
    let data = ureq::post(&url).send_json(cmd);
    data.into_json()
}

// fn create_key(ip: String, name: &str) -> String {
//     let url = format!("http://{}/api/", ip);
//     let cmd = json!({
//         "devicetype": name, 
//     });
//     let data = ureq::post(&url).send_json(cmd);
//     let data_to_string = data.into_json().unwrap()[0]["error"]["description"].to_string();
//     // data.into_json().unwrap()[0]["error"]["description"].to_string()
//     if data_to_string == "link button not pressed" {
//         println!("!!!!");
//     }
//     data_to_string
// }

fn read_key_file(path: &String) -> io::Result<String> {
    // println!("sd");
    match fs::read_to_string(&path) {
        Ok(key_from_txt) => {
            println!("\nKey Path  : {}", path);
            println!("Key Output: {}", key_from_txt);
            Ok(key_from_txt)
        },
        Err(_) => {
            // println!("\nFile 'key.txt' Not Found");
            // println!("\nKey Path  : {}", path);
            // let mut file = fs::File::create(&path)?;

            let name = "arue-Test-0.2"; 
            // let error_key = create_key("192.168.15.7".to_string(), &name).unwrap()[0]["error"]["description"].to_string().replace("\"", "");
            
            // let key = &create_key("192.168.15.7".to_string(), &name).unwrap()[0];
            // if key["error"]["description"] == "link button not pressed".to_string() {
            //     println!("\nPush button on the Bridge");
            //     // file.write_all("!".as_bytes())?;
            //     pause();
            //     println!("\nSending request to generate new key...");
            //     // let mut input = String::new();
            //     // io::stdin().read_line(&mut input).expect("panic");
            //     // file.write_all(key);
            //     return read_key_file(path);
            // } else {
            //     let gen_key = &create_key("192.165.15.7".to_string(), &name).unwrap()[0]["success"]["username"].to_string();
            //     let mut file = fs::File::create(&path)?;
            //     file.write(gen_key.as_bytes());
            // }

            // let key = &create_key("192.168.15.16".to_string(), &name).unwrap()[0];
            // println!("db");
            // let ip = find_ip();
            // println!("{:?}", find_ip());
            let key = &create_key(find_ip(), &name).unwrap()[0];
            println!("\n{:?}", key);
            if key["error"]["description"] != "link button not pressed".to_string() {
                let gen_key = key["success"]["username"].to_string().replace("\"", "");
                let mut file = fs::File::create(&path)?;
                file.write(gen_key.as_bytes()).unwrap();
                println!("Key {:?} generated and saved", gen_key);
                Ok(gen_key)
            }
            // else if 
            else {
                println!("Push button on the Bridge to generate new API Key");
                pause();
                // let mut file = fs::File::create(&path)?l
                return read_key_file(path);
            }
            // let gen_key = &create_key("192.165.15.7".to_string(), &name).unwrap()[0]["success"]["username"].to_string();
            // let mut file = fs::File::create(&path)?;
            // file.write(gen_key.as_bytes());
            // println!("aki");
            // let gen_key = &create_key("192.165.15.7".to_string(), &name).unwrap()[0]["success"]["username"].to_string();
            // println!("{:?}", gen_key);
            // file.write(gen_key.as_bytes())?;
            // let found_key = "*1234567890*";
            // file.write_all(found_key.as_bytes())?;
            // println!("Key Generated   : {:?}", key);
            // Ok(key["success"]["username"].to_string().replace("\"", ""))
            // Ok(gen_key.replace("\"",""))
        }
    }
}

fn check_connection(ip: &String, key: &String, path: &String) -> Result<ureq::SerdeValue, std::io::Error> {
    let url = format!("http://{}/api/{}/config", ip, key);
    // return Ok(send_get_request(url));
    // let r = &send_get_request(url.as_str())[0]["error"]["description"].to_string();
    let request = &send_get_request(url.as_str());
    // println!("!!!{:?}",request);
    let error_request = request["error"]["description"].to_string().replace("\"","");
    // println!(">> {:?}",error_request);
    if error_request != "null" {
        println!("\nKey not found. Trying to generate a new one...");
        let name = "arue-tests1";
        let new_key = create_key(ip.to_string(), name);
        // println!("{:?}", new_key);
        // fs::remove_file(path);
        // read_key_file(path);
        match new_key {
            Ok(_) => {
                fs::remove_file(path).unwrap();
                read_key_file(path).unwrap();
            },
            Err(_) => {
                println!("Invalid IP to generate new Key");
            }
        }
    } else {
        // println!("CONNECTION OK (GET return is {:?})", request);
        println!("IP CONNECTION OK.");
        let error_user = &request[0]["error"]["description"].to_string().replace("\"","");
        println!("{:?}", request);
        if error_user == "unauthorized user" {
            println!("\nUnauthorized Key. Trying to generate a new one...");
            let name = "arue-tests1";
            let new_key = create_key(ip.to_string(), name);
            // println!("{:?}", new_key);
            // fs::remove_file(path);
            // read_key_file(path);
            match new_key {
                Ok(_) => {
                    fs::remove_file(path).unwrap();
                    read_key_file(path).unwrap();
                },
                Err(_) => {
                    println!("Invalid IP to generate new Key");
                }
            }
        }
    }
    // println!("{:?}", r);
    Ok(().into())
}

fn main() {
    //let x = find_ip();
    //println!("{}",x);

    let ip_path = "/Users/jpcst/Documents/rust/arue/src/ip.txt".to_string();
    let key_path = "/Users/jpcst/Documents/rust/arue/src/key.txt".to_string();

    let ip = read_ip_file(ip_path).unwrap();
    let key = read_key_file(&key_path).unwrap();
    // let username:Vec<&String> = vec![&ip, &key];

    println!("\n====* MAIN FN STARTS HERE *====\n");
    println!("ip {:?}", ip);
    println!("key {:?}", key);
    // println!("{:?}", username);
    println!("debug aqui");
    let c = check_connection(&ip, &key, &key_path).unwrap(); // null is good
    println!("check = {:?}", c);
    // let name = "arue4-Test"; 
    // let y = create_key("192.168.15.7".to_string(), &name).unwrap()[0]["error"]["description"].to_string().replace("\"", "");
    // let z = &create_key("192.168.15.7".to_string(), &name).unwrap()[0];
    // println!("{:?}", y.unwrap()[0]["error"]["description"].to_string().replace("\"", ""));
    // println!("{:?}", y.unwrap()[0]["error"]["description"].to_string().replace("\"",""));
    // if y == "link button not pressed".to_string() {
    //     println!("Press the button on the Bridge to continue...");
    // } else {
    //     println!("key {:?}", z);
    // }
    // println!("{:?}", read_key_file(&key_path.clone()));
    // let v = read_key_file(&key_path.clone());

    // if y.unwrap()[0] = serde_json::from_str("error").unwrap() {
        // println!("f")
// ;    }
    // let z = y.unwrap()[0];
    // match z {
        // Object("error")=> println!("dsd"),
    // }




    //let y = send_get_request("https://discovery.mßeethue.com/");
    //print!("{:?}", y[1].as_object().unwrap()["internalipaddress"]);
}
