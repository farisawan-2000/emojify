#![allow(non_snake_case)]

use std::process::exit;
use std::collections::HashMap;

use image::RgbaImage;
use image::Delay;
use image::Frame;
use image::imageops::Lanczos3;
use show_image::{ImageView, ImageInfo, create_window};

mod emojify;
mod rgb2emoji;
mod config;

use nokhwa::{
    Camera,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    pixel_format::RgbFormat,
};

fn usage() {
    println!("Usage: {} image_width", std::env::args().nth(0).unwrap());
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // get width
    let im_width : u32 = str::parse::<u32>(&args[1]).unwrap();

    let emojiTable = rgb2emoji::generate(); // color -> emoji char

    // populate the emoji -> image map using stored font
    // TODO: serialize?
    println!("Populating emoji map...");
    let mut emojimap : HashMap<char, RgbaImage> = HashMap::new(); // char -> image
    let home = std::env::var("HOME").unwrap();
    for (_i, c) in &emojiTable {
        let emojipath = config::get_emoji_path(&home, *c as u32);

        let em = image::open(emojipath.clone()).unwrap().into_rgba8();
        emojimap.insert(*c, em);
    }

    // first camera in system
    let index = CameraIndex::Index(0); 
    // request the absolute highest resolution CameraFormat that can be decoded to RGB.
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::HighestFrameRate(30));
    // make the camera
    let mut camera = Camera::new(index, requested).unwrap();


    // Here we create a loop and just capture images as long as the device produces them. Normally,
    // this loop will run forever unless we unplug the camera or exit the program.

    // Create a window and display the image.
    let window = create_window("image", Default::default()).expect("create_window fail");

    // for event in window.event_channel()? {
    //     // Print keyboard events until Escape is pressed, then exit.
    //     // If the user closes the window, the channel is closed and the loop also exits.
    //     if let event::WindowEvent::KeyboardInput(event) = event {
    //         println!("{:#?}", event);
    //         if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
    //             break;
    //         }
    //     }
    // }

    camera.open_stream().unwrap();

    loop {
        let frame = camera.frame().unwrap();
        let buffer = frame.buffer(); // &[u8]

        println!("new frame!");

        let resized = image::load_from_memory(buffer)?
                            .resize(im_width, 999999, Lanczos3)
                            .into_rgba8();

        let result = emojify::emojify(
            &mut emojimap,
            &emojiTable,
            Frame::from_parts(
                resized,
                0, 0,
                Delay::from_numer_denom_ms(10, 1)
            )
        ).into_buffer();

        let (wd, ht) = result.dimensions();

        let im = ImageView::new(ImageInfo::rgba8(wd, ht), &result);
        
        window.set_image("image-001", &im).expect("set_image fail");
    }

}
