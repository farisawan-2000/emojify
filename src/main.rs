#![feature(const_trait_impl)]
#[macro_use] extern crate maplit;

use std::process::exit;
mod emojify;
mod rgb2emoji;


fn main() {
    let args : Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} image_width path/to/image", std::env::args().nth(0).unwrap());
        exit(1);
    }
            let im_width : u32 = str::parse::<u32>(&args[1]).unwrap();
            let filepath : &String = &args[2];
            let emojiTable = rgb2emoji::generate();
            emojify::emojify(&emojiTable, im_width, filepath);
}
