#![allow(non_snake_case)]

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
    let file_in = BufReader::with_capacity(73728, File::open(path).unwrap());
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

fn main() -> std::io::Result<()> {
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

    let emojiTable = rgb2emoji::generate(); // color -> emoji char

    // populate the emoji -> image map using stored font
    // TODO: serialize?
    println!("Populating emoji map...");
    let mut emojimap : HashMap<char, RgbaImage> = HashMap::new(); // char -> image
    let home = std::env::var("HOME").unwrap();
    for (_i, c) in &emojiTable {
        let emojipath = format!("{}/Devel/twemoji/assets/16x16/{:x}.png", home, *c as u32);

        let em = image::open(emojipath.clone()).unwrap().into_rgba8();
        emojimap.insert(*c, em);
    }
    

    let filepath : &String = match args.len() {
        2 => {
            // clipboard; guess format
            let mut clipboard = Clipboard::new().unwrap();
            let g = clipboard.get();
            let gt = g.text().unwrap();

            let basename_pt = Path::new(&gt).file_name().unwrap();
            let basename = basename_pt.to_str().unwrap();

            if !Path::new(basename_pt).exists() {
                println!("Downloading clipboard link: {}", &gt);

                let _ = std::fs::create_dir("cacheDownload/");

                let resp = attohttpc::get(&gt).send().unwrap();

                // Check if the status is a 2XX code.
                if resp.is_success() {
                    let fil = File::create(format!("cacheDownload/{}", basename)).unwrap();
                    // Consume the response body as text and print it.
                    // let tmpImg = DynamicImage::from_bytes(resp.bytes());
                    match resp.headers()["content-type"].to_str().unwrap() {
                        "image/gif" => {
                            let _ = resp.write_to(fil);
                        },
                        "image/jpeg" => {
                            let _ = resp.write_to(fil);
                        },
                        "image/png" => {
                            let _ = resp.write_to(fil);
                        },
                        "image/webp" => {
                            let _ = resp.write_to(fil);
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

            }

            &String::from(format!("cacheDownload/{}", basename))
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
 

    Ok(())   
}
