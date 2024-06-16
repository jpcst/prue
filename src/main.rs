use std::fs;
use std::io::{self, Write};
use std::io::prelude::*;
use std::sync::Once;
use serde_json::json;
extern crate serde_json;
extern crate ureq;

const IP_PATH: &str = "/Users/jpcst/Documents/rust/arue/src/ip.txt";
const KEY_PATH: &str = "/Users/jpcst/Documents/rust/arue/src/key.txt";

#[derive(Debug)]
struct Double(f32, f32);

fn lights_list() -> Vec<u8> {
    let data = data(IP_PATH, KEY_PATH);
    let mut lights: Vec<u8> = vec![]; // vector that will contain lights
    let mut i: u8 = 1;

    loop {
        let js = serde_json::to_string(&data[format!("{}", i)]["state"]["reachable"]).unwrap(); // scrapes N lights from api
        // println!("{:?}", js);

        if &js == "null" && i == 1{ // light is out of range from api -> doesn't exist
            i += 1;
        }
        else if &js == "null" && i > 2 {
            break;
        }

        lights.push(i); // appends light to vector
        i += 1;
    }
    lights
}

fn is_on(list: Vec<u8>) -> Vec<bool> { // returns a vector with light states (on->true/off->false)
    let data = data(IP_PATH, KEY_PATH);
    let size = list.len(); // number of lights
    let mut state: Vec<bool> = vec![]; // vector to store states

    for i in 0..size {
        let js = serde_json::to_string(&data[format!("{}", i+2)]["state"]["on"]).unwrap();
        if &js == "true" { // if on -> append true
            state.push(true);
        } else {
            state.push(false);
        }
    }
    state
}

fn brightness(list: Vec<u8>) -> Vec<u8> { // returns a vector with light states (on->true/off->false)
    let data = data(IP_PATH, KEY_PATH);
    let size = list.len(); // number of lights
    let mut state: Vec<u8> = vec![]; // vector to store states

    for i in 0..size {
        if list[i] == 1 {
            let js = serde_json::to_string(&data[format!("{}", i+2)]["state"]["bri"]).unwrap();
            state.push(js.parse().unwrap());
        }
    }
    state
}

// pub fn get_color(list: Vec<u8>) -> Vec<Double> { // returns a vector with light states (on->true/off->false)
//     let data = data(IP_PATH, KEY_PATH);
//     let size = list.len(); // number of lights
//     // let mut state: Vec<f32> = vec![]; // vector to store states
//     let mut v: Vec<Double> = Vec::new();

//     for i in 0..size {
//         if list[i] == 1 {
//             let js0 = serde_json::to_string(&data[format!("{}", i+2)]["state"]["xy"][0]).unwrap();
//             let js1 = serde_json::to_string(&data[format!("{}", i+2)]["state"]["xy"][1]).unwrap();
//             // state.push(js0.parse().unwrap());
//             // state.push(js1.parse().unwrap());   
//             v.push(Double(js0.parse().unwrap(), js1.parse().unwrap()));         
//         }
//     }
//     // println!("{:?}", state);
//     return v;
// }

fn names(list: Vec<u8>) -> Vec<String> {
    let data = data(IP_PATH, KEY_PATH);
    let size = list.len(); // number of lights
    let mut names: Vec<String> = vec![];

    for i in 0..size {
        let js = serde_json::to_string(&data[format!("{}", i+2)]["name"]).unwrap();
        // let mut j: String = js.parse().unwrap();
        // names.push(j.replace(r#"\"#,".").replace(r#"\"""#,"."));
        names.push(js);
    }
    names
}

fn do_light(bri: u8, tt: u8, list: Vec<u8>) {
    let ip = read_file(IP_PATH);
    let key = read_file(KEY_PATH);
    let url = url(ip, key);
    let lights = lights_list();
    
    let size = list.len();
    let mut change: Vec<usize> = Vec::new();
    for i in 0..size {
        if list[i] == 1{
            change.push(lights[i].into());
        }
    }
    let csize = change.len();
    // let omega = is_on(change.clone());
    // println!("TO CHANGE {:?}\n{:?}\nN = {}\n{:?}\n\n", list, change, csize, omega);

    for i in 0..csize {
        let on = is_on(list.clone());
        let url: String = format!("{}{}/state", url, change.clone()[i]);
        // println!("{:?}", url);
        let k:usize = change[i] - 2;
        // println!("{}", on[k as usize]);
        // println!("{} {}", i, k);

        if on[k] == false {
            // println!("{}false", k);
            let cmd = json!({
                "on": true,
                "bri": bri,
                "transitiontime": tt
            });
            ureq::put(&url).send_json(cmd);
            // println!("{} => {} -> {}", names(lights.clone())[k], on[k], !!!(on[k]))
        }
        else {
            // println!("true");
            let cmd = json!({
                "on": false,
                "bri": bri,
                "transitiontime": tt
            });
            ureq::put(&url).send_json(cmd);
            // println!("{} => {} -> {}", names(lights.clone())[k], on[k], !!!(on[k]))
        }  
    }
}

fn change_color(list: Vec<u8>, xy: &Double) {
    let ip = read_file(IP_PATH);
    let key = read_file(KEY_PATH);
    let url = url(ip, key);
    let lights = lights_list();

    let size = list.len();
    let mut change: Vec<usize> = Vec::new();
    for i in 0..size {
        if list[i] == 1{
            change.push(lights[i].into());
        }
    }
    let csize = change.len();

    for i in 0..csize {
        let url: String = format!("{}{}/state", url, change.clone()[i]);
        // let k: usize = change[i] - 2;
        let cmd = json!({
            "xy": [xy.0, xy.1]
        });
        ureq::put(&url).send_json(cmd);
    }
}

fn change_bri(bri: u8, tt: u8, list: Vec<u8>) {
    let ip = read_file(IP_PATH);
    let key = read_file(KEY_PATH);
    let url = url(ip, key);
    let lights = lights_list();
    
    let size = list.len();
    let mut change: Vec<usize> = Vec::new();
    for i in 0..size {
        if list[i] == 1{
            change.push(lights[i].into());
        }
    }
    let csize = change.len();

    for i in 0..csize {
        let on = is_on(list.clone());
        let url: String = format!("{}{}/state", url, change.clone()[i]);
        // println!("{:?}", url);
        let k: usize = change[i] - 2;
        // println!("{}", on[k as usize]);
        // println!("{} {}", i, k);

        if on[k] == true {
            let cmd = json!({
                "bri": bri,
                "transitiontime": tt
            });
            ureq::put(&url).send_json(cmd);
            // println!("{} => {} -> {}", names(lights.clone())[k], on[k], !!!(on[k]))
        }  
    }
}

////////////////////////////////////////////////////////////////////////

fn read_file(dir: &'static str) -> String {
    fs::read_to_string(dir).expect("Error reading ip.")
}

fn url(ip: String, key: String) -> String {
    format!("http://{}/api/{}/lights/", ip, key)
}

fn send_get_request(url: String) -> ureq::SerdeValue {
    let get = ureq::get(&url).call(); // calls the api
    let data = get.into_json().unwrap(); // saves data (ip)
    data
}

fn data(ip: &'static str, key: &'static str) -> serde_json::value::Value {
    let ip = read_file(ip);
    let key = read_file(key);
    send_get_request(url(ip, key))
}

///////////////////////////////////////////////////////////////////////////

fn pause() { // press any key to continue
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

// fn send_get_request(url: &str) -> ureq::SerdeValue {
//     let get = ureq::get(&url).call(); // calls the api
//     let data = get.into_json().unwrap(); // saves data (ip)
//     data
// }
// fn send_get_request(url: &str) -> Result<ureq::SerdeValue, Error> {
//     let get = ureq::get(url).call();
//     let data = get.into_json().unwrap(); // saves data (ip)
//     match get {
//         Ok(get) => {
//             data
//         },
//         Err(err) => {
            
//         }
//     }
    
// }

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
fn find_ip() -> String {
    let discovery_addr = "https://discovery.meethue.com/".to_string(); // adicionar too many requests - error handle
    let ip_addr = send_get_request(discovery_addr);
    let ip_array = ip_addr.as_array().unwrap();
    println!("{:?}", ip_array);
    if ip_array.len() == 1 {
        print!("\nSingle IP ");
        ip_addr[0]["internalipaddress"].to_string().replace("\"", "")
    }
    else {
        for i in 0..ip_array.len() {
            println!("({}) - {:?}", i+1, ip_array[i]);
        }
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();
        write!(stdout, "Multiple IPs found, choose one: ").unwrap();
        stdout.flush().unwrap();
        let number = stdin.read(&mut [0u8]).unwrap() - 1; // ainda tenho q testar
        println!("Connecting to {:?}", ip_array[number]);
        ip_addr[number]["internalipaddress"].to_string().replace("\"", "")
    }
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
            println!("{} found and saved\n", ip);
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

fn read_key_file(path: &String) -> io::Result<String> {
    match fs::read_to_string(&path) {
        Ok(key_from_txt) => {
            println!("Key Path  : {}", path);
            println!("Key Output: {}", key_from_txt);
            Ok(key_from_txt)
        },
        Err(_) => {
            let name = "arue-Main-1.0"; // 16-6-24
            let key = &create_key(find_ip(), &name).unwrap()[0];
            println!("\n{:?}", key);
            if key["error"]["description"] != "link button not pressed".to_string() {
                let gen_key = key["success"]["username"].to_string().replace("\"", "");
                let mut file = fs::File::create(&path)?;
                file.write(gen_key.as_bytes()).unwrap();
                println!("Key {:?} generated and saved", gen_key);
                Ok(gen_key)
            }
            else {
                println!("\nPush button on the Bridge to generate new API Key");
                pause();
                // let mut file = fs::File::create(&path)?
                return read_key_file(path);
            }
        }
    }
}

fn check_connection(ip: &String, key: &String, path: &String) -> Result<ureq::SerdeValue, std::io::Error> {
    let url = format!("http://{}/api/{}/config", ip, key);
    let request = &send_get_request(url);
    let error_request = request["error"]["description"].to_string().replace("\"","");
    // println!(">> {:?}",error_request);
    if error_request != "null" {
        println!("\nKey not found. Trying to generate a new one...");
        let name = "arue-tests1";
        let new_key = create_key(ip.to_string(), name);
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
        println!("API CONNECTION OK");
        let error_user = &request[0]["error"]["description"].to_string().replace("\"","");
        // println!("{:?}", request);
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

// fn check() {
//     let ip_path = "/Users/jpcst/Documents/rust/arue/src/ip.txt".to_string();
//     let key_path = "/Users/jpcst/Documents/rust/arue/src/key.txt".to_string();

//     let ip = read_ip_file(ip_path).unwrap();
//     let key = read_key_file(&key_path).unwrap();
//     println!("");    
//     let _ = check_connection(&ip, &key, &key_path).unwrap(); // null is good
//     println!("");
// }

fn check() {
    let ip = read_ip_file(IP_PATH.to_string()).unwrap();
    let key = read_key_file(&KEY_PATH.to_string()).unwrap();
    println!("");
    let _ = check_connection(&ip, &key, &KEY_PATH.to_string()).unwrap(); // null is good
    println!("");
}

fn main() {
    clearscreen::clear().expect("failed to clear screen");
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        check();
        // clearscreen::clear().expect("failed to clear screen");
    });

    terminal::Action::SetTerminalSize(50,50);
    let desk: Vec<u8> = vec![0,0,0,1,0,0];
    let ceil: Vec<u8> = vec![1,0,1,0,1,1];
    let bed:  Vec<u8> = vec![0,1,0,0,0,0];
    let all:  Vec<u8> = vec![1,1,1,1,1,1];
    let c1:   Vec<u8> = vec![1,0,0,0,0,0];
    let c2:   Vec<u8> = vec![0,0,1,0,0,0];
    let c3:   Vec<u8> = vec![0,0,0,0,1,0];
    let c4:   Vec<u8> = vec![0,0,0,0,0,1];
    let db:   Vec<u8> = vec![0,1,0,1,0,0];

    let lights = lights_list();
    let names = names(lights.clone());
    let is_on = is_on(lights.clone());
    let bri = brightness(all.clone());
    // println!("{:?}", bri[0]/254.0*100.0);
    println!("|----- NAME -----|-- STATE --|-- BRI --|");
    println!("|                |           |         |");
    for i in 0..lights.len() {
        println!("|   {:<12} |   {:<5}   |   {:<03}   |", names[i].replace("\"", ""), is_on[i], bri[i]);
    }
    println!("|                |           |         |");
    println!("|----------------|-----------|---------|\n");
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Error reading input");
    let ipt_vec: Vec<&str> = ipt.split_whitespace().collect();

    if ipt_vec[0] == "d" {
        do_light(255, 0, desk);
    }
    else if ipt_vec[0] == "b" {
        do_light(255, 0, bed);
    }
    else if ipt_vec[0] == "c" {
        do_light(255, 0, ceil);
    }
    else if ipt_vec[0] == "c1" {
        do_light(255, 0, c1);
    }
    else if ipt_vec[0] == "c2" {
        do_light(255, 0, c2);
    }
    else if ipt_vec[0] == "c3" {
        do_light(255, 0, c3);
    }
    else if ipt_vec[0] == "c4" {
        do_light(255, 0, c4);
    }
    else if ipt_vec[0] == "db" {
        do_light(255, 0, db);
    }
    else if ipt_vec[0] == "info" || ipt_vec[0] == "?" {
        check();
        pause();
    }
    else if ipt_vec[0] == "all" {
        for i in 0..is_on.len() {
            if is_on.clone()[i] == true {
                let change: Vec<u8> = bool_to_int(is_on.clone());
                do_light(255, 0, change);
            }
        }
    }

    else if ipt_vec[0] == "br" {
        for i in 0..is_on.len() {
            if is_on.clone()[i] == true {
                let change: Vec<u8> = bool_to_int(is_on.clone());
                change_color(change, &Double(0.3, 0.3));
            }
        }
        if is_on.clone()[1] == true {
            change_color(bed, &Double(0.5019, 0.4152));
        }
    }

    else if ipt_vec[0] == "am" {
        for i in 0..is_on.len() {
            if is_on.clone()[i] == true {
                let change: Vec<u8> = bool_to_int(is_on.clone());
                change_color(change, &Double(0.5019, 0.4152)); // .5203, .4141
            }
        }
    }

    else if ipt_vec[0] == "/" || ipt_vec[0] == "rgb" {
        let r: u8 = ipt_vec[1].parse().unwrap();
        let g: u8 = ipt_vec[2].parse().unwrap();
        let b: u8 = ipt_vec[3].parse().unwrap();
        // println!("{} {} {}", r,g, b);
        let xy = rgb_to_xy(r, g, b);
        // println!("{:?}", xy);
        change_color(bool_to_int(is_on.clone()), &xy);
        // io::stdin().read_line(&mut ipt).expect("Error reading input");
    }

    else if ipt_vec[0] == "-" || ipt_vec[0] == "bri" {
        let bri: u32 = ipt_vec[1].parse().unwrap();
        let bri = bri * 255 / 100;
        change_bri(bri as u8, 0, bool_to_int(is_on));
    }

    else {
        clearscreen::clear().expect("failed to clear screen");
        main();
    }
    // clearscreen::clear().expect("failed to clear screen");
    main();

}

fn bool_to_int (x: Vec<bool>) -> Vec<u8> {
    let mut y: Vec<u8> = Vec::new();
    for i in 0..x.len() {
        if x[i] == true {
            y.push(1);
        }
        else {
            y.push(0);
        }
    }
    y
}

fn rgb_to_xy(r: u8, g: u8, b:u8) -> Double {
    let x: f32 = 0.4124*r as f32 + 0.3576*g as f32 + 0.1805*b as f32;
    let y: f32 = 0.2126*r as f32 + 0.7152*g as f32 + 0.0722*b as f32;
    let z: f32 = 0.0193*r as f32 + 0.1192*g as f32 + 0.9505*b as f32;
    let x1 = x / (x+y+z);
    let y1 = y / (x+y+z);
    Double(x1,y1)
}
