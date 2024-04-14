#![feature(const_trait_impl)]
#[macro_use] extern crate maplit;

use std::process::exit;
use std::path::Path;
use std::fs::File;
use attohttpc;
use image::{
    DynamicImage,
    RgbaImage,
    Frame,
    Delay,
    imageops::FilterType::Lanczos3,
    AnimationDecoder,
};
use std::collections::HashMap;
use image::codecs::gif::{Repeat, GifDecoder, GifEncoder};
use std::io::{BufReader, BufWriter};
use arboard::Clipboard;

mod emojify;
mod rgb2emoji;

fn open_img(width: u32, path: &String) -> Frame {
    let img = image::open(path).unwrap()
        .resize(width, 999999, Lanczos3).into_rgba8();

    return Frame::from_parts(img, 0, 0, Delay::from_numer_denom_ms(10, 1));
}

fn usage() {
    println!("Usage: {} image_width [path/to/image or leave blank for clipboard]", std::env::args().nth(0).unwrap());
}

fn open_gif(width: u32, path: &String) -> Vec<Frame> {
    let file_in = BufReader::new(File::open(path).unwrap());
    let decoder = GifDecoder::new(file_in).unwrap();
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding gif");

    let mut ret : Vec<Frame> = Vec::new();

    for f in frames {
        let frm = DynamicImage::ImageRgba8(
                      f.clone().into_buffer()
                  ).resize(
                      width, 999999, Lanczos3
                  );
        ret.push(Frame::from_parts(frm.into_rgba8(), 0, 0, f.delay()));
    }

    return ret;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            usage();
            exit(1);
        },
        2 | 3 => {
            // good
        },
        _ => {
            usage();
            exit(1);
        },
    }

    // get width
    let im_width : u32 = str::parse::<u32>(&args[1]).unwrap();

    let emojiTable = rgb2emoji::generate();

    // global emoji map
    let mut emojimap : HashMap<String, RgbaImage> = HashMap::new();

    let filepath : &String = match args.len() {
        2 => {
            // clipboard; guess format
            let mut clipboard = Clipboard::new().unwrap();
            let g = clipboard.get();
            let gt = g.text().unwrap();

            let resp = attohttpc::get(&gt).send().unwrap();

            // Check if the status is a 2XX code.
            if resp.is_success() {
                // Consume the response body as text and print it.
                // let tmpImg = DynamicImage::from_bytes(resp.bytes());
                match resp.headers()["content-type"].to_str().unwrap() {
                    "image/gif" => {
                        let fil = File::create("tmp.gif").unwrap();
                        let _ = resp.write_to(fil);
                        &String::from("tmp.gif")
                    },
                    "image/jpeg" => {
                        let fil = File::create("tmp.jpg").unwrap();
                        let _ = resp.write_to(fil);
                        &String::from("tmp.jpg")
                    },
                    "image/png" => {
                        let fil = File::create("tmp.png").unwrap();
                        let _ = resp.write_to(fil);
                        &String::from("tmp.png")
                    },
                    "image/webp" => {
                        let fil = File::create("tmp.webp").unwrap();
                        let _ = resp.write_to(fil);
                        &String::from("tmp.webp")
                    },
                    _ => {
                        println!("{:?} is not a supported file!", resp.headers()["content-type"]);
                        exit(1);
                    }
                }
            } else {
                println!("'{}' is not a URL!", gt);
                exit(1);
            }
        },
        3 => {
            &args[2]
        },
        _ => {
            usage();
            exit(1);
        },
    };

    println!("Converting {} to emojis...", &filepath);

    match Path::new(&filepath).extension().unwrap().to_str() {
        Some("gif") => {
            let frames = open_gif(im_width, filepath);

            let mut bufFrames : Vec<Frame> = Vec::new();

            let file_out = File::create("output.gif").unwrap();

            let bufWriter = BufWriter::with_capacity(163840, file_out);
            let mut encoder = GifEncoder::new_with_speed(bufWriter, 30);
            let _ = encoder.set_repeat(Repeat::Infinite);

            for (i, f) in frames.iter().enumerate() {
                println!("Processing Frame {} of {}", i + 1, frames.len());

                let result = emojify::emojify(&mut emojimap, &emojiTable, f.clone());

                bufFrames.push(result);
            }
            let _ = encoder.encode_frames(bufFrames);
        },
        Some("png") | Some("jpg") | Some("webp") => {
            let img = open_img(im_width, filepath);
            let result = emojify::emojify(&mut emojimap, &emojiTable, img).into_buffer();
            let _ = result.save("output.png");
        },
        Some(&_) => {
            println!("Filepath not supported: {}", filepath);
            exit(1);
        }
        None => {
            println!("Filepath not valid: {}", filepath);
            exit(1);
        }
    }
    
}
