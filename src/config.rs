

pub const EMOJI_WD: u32 = 16;
pub const EMOJI_HT: u32 = 16;

pub fn get_emoji_path(home: &String, c : u32) -> String {
    return format!("{}/emojifont/{:x}.png", home, c);
}

