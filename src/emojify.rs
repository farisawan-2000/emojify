use image::{
    // GenericImageView,
    DynamicImage,
    // ImageBuffer,
    Rgba,
    imageops,
    imageops::FilterType::Lanczos3,
};
// use image::io::Reader as ImageReader;
use std::collections::HashMap;
use super::rgb2emoji;
// use itertools::izip;

fn pack_color(payload: (u32, u32, &Rgba<u8>)) -> (u32, u32, u32) {
    let px = payload.2;
    let color = ((px[0] as u32) << 24)
         | ((px[1] as u32) << 16)
         | ((px[2] as u32) << 8)
         | (px[3] as u32);

    return (payload.0, payload.1, color);
}

pub fn emojify(tbl: &HashMap<u32, char>, width: u32, path: &String) {

    let binding = image::open(path).unwrap()
        .resize(width, 999999, Lanczos3);
    let img = binding.as_rgba8().unwrap();

    let (mut wd, ht) = img.dimensions();

    let mut resultImg = DynamicImage::new_rgba8(wd * 16, ht * 16);
    // let colors = pixels;

    let pixels = img.enumerate_pixels().map(pack_color);

    let mut emojis : Vec<char> = Vec::new();
    for i in pixels {
        emojis.push(rgb2emoji::search(tbl, i.2));
    }






    // emojis.iter().map(commit_pixel);

    let mut x = 0;
    let mut y = 0;

    let mut cursor = 0;

    wd -= 1;

    for i in emojis {
        let home = std::env::var("HOME").unwrap();

        let filepath = format!("{}/Devel/twemoji/assets/16x16/{:x}.png", home, i as u32);
        let em = image::open(filepath).unwrap();

        imageops::overlay(&mut resultImg, &em, x, y);

        cursor += 1;
        x += 16;
        if cursor > wd {
            x = 0;
            cursor = 0;
            y += 16;
        }
    }

    let _ = resultImg.save("output.png");
}

