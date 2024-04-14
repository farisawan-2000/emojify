// use image::{
//     RgbaImage,
//     DynamicImage,
//     Rgba,
//     Frame,
//     // Delay,
//     imageops,
//     // GenericImageView,
// };
use std::process::exit;
// mod rgb2emoji;
// use serde::Serialize;

fn usage() {
    println!("Usage: {} path/to/emojifont/", std::env::args().nth(0).unwrap());
    println!("    (Emoji images must be named based off their unicode path, see Twemoji assets/16x16)");
}

fn main() {
    let args : Vec<String> = std::env::args().collect();

    match args.len() {
        2 => {
            // good
        },
        _ => {
            usage();
            exit(1);
        },
    }

    // let 

    
}