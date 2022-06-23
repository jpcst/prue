extern crate serde_json;
extern crate ureq;

mod bridge;
use bridge::{read_file, url, data};
use ureq::json;

const IP_PATH: &'static str = "C:/Rust/Rue/src/info/ip.txt";
const KEY_PATH: &'static str = "C:/Rust/Rue/src/info/key.txt";

#[derive(Debug)]
pub struct Double(pub f32, pub f32);

pub fn lights_list() -> Vec<u8> {
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

pub fn is_on(list: Vec<u8>) -> Vec<bool> { // returns a vector with light states (on->true/off->false)
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

pub fn brightness(list: Vec<u8>) -> Vec<u8> { // returns a vector with light states (on->true/off->false)
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

pub fn names(list: Vec<u8>) -> Vec<String> {
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

pub fn do_light(bri: u8, tt: u8, list: Vec<u8>) {
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

pub fn change_color(list: Vec<u8>, xy: &Double) {
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

pub fn change_bri(bri: u8, tt: u8, list: Vec<u8>) {
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