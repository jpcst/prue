    // TO-DO: checar se ip no txt é o correto se nao for corrigir

extern crate serde_json;
use ureq::json;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io;

fn main() {
    let dir: &'static str = "C:/Rust/ip.txt";
    let dir_key: &'static str = "C:/Rust/key.txt";
    // let ipaddr = find_ip();
    // write_file(dir, ipaddr);
    // print!("Bridge ip ok ");
    let ip = read_file(dir).unwrap();
    let key = create_key(ip.clone(), "Rust_1".to_string());

    // println!("{:?}\n", key.as_ref().unwrap()[0]);

    if key.as_ref().unwrap()[0]["error"]["description"].to_string().replace("\"", "") == "link button not pressed" {
        println!("Key not found\nPush button on Bridge and press enter to continue");
        io::stdin().read_line(&mut String::new()).expect("Failed to read input");
        main();
    } else {
        let key_from_file = read_file(dir_key);
        println!("{:?}\n", key_from_file);
        match key_from_file {
            Ok(_) => println!("Valid key found"),
            Err(ref err) => if err.kind() == ErrorKind::NotFound {
                println!("Creating file\nSaving new key {:?}", key.as_ref().unwrap()[0]["success"]["username"].to_string().replace("\"", ""));
                write_file(dir_key, key.as_ref().unwrap()[0]["success"]["username"].to_string().replace("\"", ""));
            },
        }
        // println!("Created new key {:?}", key.as_ref().unwrap()[0]["success"]["username"].to_string().replace("\"", ""));
        // write_file(dir_key, key.as_ref().unwrap()[0]["success"]["username"].to_string().replace("\"", ""));
    }

    let u = http_get(url(ip, key.as_ref().unwrap()[0]["success"]["username"].to_string().replace("\"", "")));
    // println!("{:?}", u[0]["error"]["description"].to_string().replace("\"", ""));
    if u[0]["error"]["description"].to_string().replace("\"", "") != "unauthorized user" {
        println!("\nNew key ok\nConnected to API\nPress enter to continue");
        io::stdin().read_line(&mut String::new()).expect("Failed to read input");
    } else {
        println!("\nNew key error\nNot connected to API\n");
        io::stdin().read_line(&mut String::new()).expect("Failed to read input");
        panic!("Failed to connect");
    }
}

pub fn read_file(dir: &'static str) -> Result<String, std::io::Error> {
    let file = fs::read_to_string(dir);
    match file {
        Ok(ref f) => println!("Bridge ip ok {:?}", f),
        Err(ref err) => if err.kind() == ErrorKind::NotFound {
            println!("Creating file at {:?}", dir);
            File::create(dir).unwrap_or_else(|error| {
                println!("Couldn't create file");
                panic!("Coudn't create file: {:?}", error);
            });
            write_file(dir, find_ip()) 
        } else {
            println!("Couldn't open file");
            panic!("Couldn't open file");
        },
    };
    file
}

pub fn write_file(dir: &'static str, data: String) {
    fs::write(dir, data).expect("Error writing file.");
    // let file = fs::write(dir, data);
    // match file {
    //     Ok(ref f) => println!("{:?}", f),
    //     Err(ref err) => if err.kind() == ErrorKind::NotFound {
    //         File::create(dir).unwrap_or_else(|erorr| {
    //             println!("Couldn't create file");
    //             panic!("Couldn't create file: {:?}", error);
    //         });
    //         fs::write(dir, data);
    //     } else {
    //         println!("Couldn't open file");
    //         panic!("Couldn't open file");
    //     },
    // };
    // match ipaddr {
    //     Ok(ip) => println!("Saving {:?} to {:?}", ip, dir),
    //     Err(err) => panic!("ERROR!"),
    // };
}

pub fn url(ip: String, key: String) -> String {
    format!("http://{}/api/{}/lights/", ip, key)
}

// TO-DO: InvalidData
pub fn http_get(url: String) -> ureq::SerdeValue {
    let get = ureq::get(&url).call();
    // let data = get.into_json().unwrap();
    // match data {
    //     Ok(ref d) => println!("{:?}", d),
    //     Err(ref err) => if err.kind() == ErrorKind::InvalidData {
    //         println!("Too many requests, wait before trying again");
    //         panic!("Invalid data");
    //     },
    // };
    let data = get.into_json().unwrap();
    data
}

// pub fn data(ip: String, key: &'static str) -> serde_json::value::Value {
    // let ip = read_file(ip);
    // let key = read_file(key);
    // http_get(url(ip, key))
// }

pub fn create_key(ipaddr: String, name: String) -> Result<ureq::SerdeValue, std::io::Error> {
    let url = format!("http://{}/api/", ipaddr);
    // let get = ureq::get(&url).call();
    // let data = get.into_json().unwrap();

    let cmd = json!({
        "devicetype": name, 
    });
    let data = ureq::post(&url).send_json(cmd);
    data.into_json()
}

pub fn press_button(ipaddr: String, name: String) {
    let key = create_key(ipaddr.clone(), name.clone());
    if key.unwrap()[0]["error"]["description"].to_string().replace("\"", "") == "link button not pressed".to_string() {
        println!("Press the button on the Bridge to continue...\n");
        // io::stdin().read_line(&mut String::new()).expect("Failed to read");
    } else {
        println!("Working");
    }
    // create_key(ipaddr, name); 
}

pub fn find_ip() -> String {
    let discovery_addr = "https://discovery.meethue.com/".to_string();
    let ipaddr = http_get(discovery_addr);
    ipaddr[0]["internalipaddress"].to_string().replace("\"", "")
}

