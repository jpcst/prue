#![allow(unused_variables)]

extern crate ureq;
extern crate serde_json;
extern crate clearscreen;
extern crate terminal;
mod states;
use states::{lights_list, is_on, brightness, names,
                          do_light, change_color, change_bri};
use std::io;

// [ 0. C1
//   1. BED
//   2. C2
//   3. DESK
//   4. C3
//   5. C4  ]

fn main() {
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

    println!("|----- NAME -----|-- STATE --|-- BRI --|");
    for i in 0..lights.len() {
        println!("|   {:<12} |   {:<5}   |   {:<3}   |", names[i].replace("\"", ""), is_on[i], bri[i]);
    }

    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Error reading input");
    let ipt_vec: Vec<&str> = ipt.split_whitespace().collect();
    // println!("{:?}", ipt_vec);

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
                change_color(change, &states::Double(0.3, 0.3));
            }
        }
        if is_on.clone()[1] == true {
            change_color(bed, &states::Double(0.5019, 0.4152));
        }
    }

    else if ipt_vec[0] == "am" {
        for i in 0..is_on.len() {
            if is_on.clone()[i] == true {
                let change: Vec<u8> = bool_to_int(is_on.clone());
                change_color(change, &states::Double(0.5019, 0.4152)); // .5203, .4141
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

    clearscreen::clear().expect("failed to clear screen");
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

fn rgb_to_xy(r: u8, g: u8, b:u8) -> states::Double {
    let x: f32 = 0.4124*r as f32 + 0.3576*g as f32 + 0.1805*b as f32;
    let y: f32 = 0.2126*r as f32 + 0.7152*g as f32 + 0.0722*b as f32;
    let z: f32 = 0.0193*r as f32 + 0.1192*g as f32 + 0.9505*b as f32;
    let x1 = x / (x+y+z);
    let y1 = y / (x+y+z);
    states::Double(x1,y1)
}
