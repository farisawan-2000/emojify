use image::{
    RgbaImage,
    DynamicImage,
    Rgba,
    Frame,
    // Delay,
    imageops,
    // GenericImageView,
};
use std::collections::HashMap;
use super::rgb2emoji;

fn pack_color(payload: (u32, u32, &Rgba<u8>)) -> (u32, u32, u32) {
    let px = payload.2;
    let color = ((px[0] as u32) << 24)
         | ((px[1] as u32) << 16)
         | ((px[2] as u32) << 8)
         | (px[3] as u32);

    return (payload.0, payload.1, color);
}

// TODO: Convert Frame to RgbaImage and back in here
pub fn emojify(
    map: &mut HashMap<String, RgbaImage>,
    tbl: &HashMap<u32, char>,
    img: Frame
) -> Frame {
    let image = img.buffer();
    let delay = img.delay();

    let (mut wd, ht) = image.dimensions();

    let mut resultImg = DynamicImage::new_rgba8(wd * 16, ht * 16);
    // let colors = pixels;

    let pixels = image.enumerate_pixels().map(pack_color);

    let mut emojis : Vec<char> = Vec::new();
    for i in pixels {
        emojis.push(rgb2emoji::search(tbl, i.2));
    }

    // emojis.iter().map(commit_pixel);

    let mut x = 0;
    let mut y = 0;

    let mut cursor = 0;

    wd -= 1;

    // TODO: make this global so we can save it every frame
    

    for i in emojis {
        let home = std::env::var("HOME").unwrap();

        let emojipath = format!("{}/Devel/twemoji/assets/16x16/{:x}.png", home, i as u32);
        match map.get(&emojipath) {
            Some(emoji) => {
                imageops::overlay(&mut resultImg, emoji, x, y);
            },
            None => {
                let em = image::open(emojipath.clone()).unwrap().into_rgba8();
                map.insert(emojipath.clone(), em);
                imageops::overlay(&mut resultImg, &map[&emojipath.clone()], x, y);
            }
        }


        cursor += 1;
        x += 16;
        if cursor > wd {
            x = 0;
            cursor = 0;
            y += 16;
        }
    }

    return Frame::from_parts(resultImg.into_rgba8(), 0, 0, delay);
}

