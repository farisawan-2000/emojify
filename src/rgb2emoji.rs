use std::collections::HashMap;
use maplit::hashmap;

// pub fn search(tbl: &HashMap<u32, char>, entry: u32)
pub fn search(tbl: &HashMap<u32, char>, entry: u32) -> char {
    if entry & 0xFF < 128 {
        // return (payload.0, payload.1, (data.0, data.1, ' '));
        return ' ';
    }
    let mut min_dist = 9999999;
    let mut min_colour = ' ';
    for (key, _val) in tbl {
        let r_c = key >> 16;
        let g_c = (key >> 8) & 0xFF;
        let b_c = (key >> 0) & 0xFF;

        let rd = (r_c as i32 - ((entry >> 24) & 0xFF) as i32).pow(2) as u32;
        let gd = (g_c as i32 - ((entry >> 16) & 0xFF) as i32).pow(2) as u32;
        let bd = (b_c as i32 - ((entry >> 8)  & 0xFF) as i32).pow(2) as u32;

        if (rd + gd + bd) < min_dist {
            min_dist = rd + gd + bd;
            min_colour = tbl[&key];
        }
    }
    // return (payload.0, payload.1, min_colour);
    // return min_colour;
    return min_colour;
}



pub fn generate() -> HashMap<u32, char> {
    return hashmap!(
        0xBDB5B0 => '🀄',
        0xB2AEAA => '🃏',
        0xE15264 => '🅰',
        0xE3596A => '🅱',
        0xE35B6C => '🅾',
        0x4880AA => '🅿',
        0xE66E7D => '🆎',
        0xE46373 => '🆑',
        0x5698CA => '🆒',
        0x65A1CF => '🆓',
        0xB091D9 => '🆔',
        0x619FCD => '🆕',
        0x6CA5D1 => '🆖',
        0x68A3D0 => '🆗',
        0xE35D6E => '🆘',
        0x609ECD => '🆙',
        0xF6B053 => '🆚',
        0x5C9CCC => '🇦',
        0x63A0CE => '🇧',
        0x5899CB => '🇨',
        0x629FCE => '🇩',
        0x599ACB => '🇪',
        0x5496C9 => '🇫',
        0x63A0CE => '🇬',
        0x629FCE => '🇭',
        0x4A91C7 => '🇮',
        0x4E93C8 => '🇯',
        0x5E9DCD => '🇰',
        0x4F94C8 => '🇱',
        0x6FA7D2 => '🇲',
        0x64A0CE => '🇳',
        0x629FCE => '🇴',
        0x5A9BCC => '🇵',
        0x67A3CF => '🇶',
        0x609ECD => '🇷',
        0x5B9BCC => '🇸',
        0x5195C9 => '🇹',
        0x5F9DCD => '🇺',
        0x5899CB => '🇻',
        0x70A8D2 => '🇼',
        0x5C9CCC => '🇽',
        0x5597CA => '🇾',
        0x5A9ACB => '🇿',
        0x5396C9 => '🈁',
        0x3A76A3 => '🈂',
        0xF5A53C => '🈚',
        0x90C074 => '🈯',
        0xE25667 => '🈲',
        0x457EA8 => '🈳',
        0xE04C5E => '🈴',
        0xE25869 => '🈵',
        0xF4A235 => '🈶',
        0xF49F2F => '🈷',
        0xF4A030 => '🈸',
        0xE25768 => '🈹',
        0xF4A439 => '🈺',
        0x885248 => '🉐',
        0xD49939 => '🉑',
        0x4E90A3 => '🌀',
        0xCCC6CA => '🌁',
        0x5A696F => '🌂',
        0x38515F => '🌃',
        0xBCAE4B => '🌄',
        0xA9A478 => '🌅',
        0x7B5E31 => '🌆',
        0xA98D4F => '🌇',
        0x87734E => '🌈',
        0x53515C => '🌉',
        0x4A8EA7 => '🌊',
        0x70715B => '🌋',
        0x5F4889 => '🌌',
        0x6DA69C => '🌍',
        0x70A9A6 => '🌎',
        0x6DA59A => '🌏',
        0x417B86 => '🌐',
        0x5E7274 => '🌑',
        0x798374 => '🌒',
        0x9D9A73 => '🌓',
        0xC0B171 => '🌔',
        0xDBC371 => '🌕',
        0xC0B171 => '🌖',
        0x9D9A73 => '🌗',
        0x798374 => '🌘',
        0x7D8E5A => '🌙',
        0x5A6D6E => '🌚',
        0x828F5A => '🌛',
        0x828F5A => '🌜',
        0xCAB56C => '🌝',
        0xBE9A37 => '🌞',
        0xA69347 => '🌟',
        0x758880 => '🌠',
        0x737B68 => '🌡',
        0xA8A26E => '🌤',
        0x98A588 => '🌥',
        0x99AA93 => '🌦',
        0x8AA9A0 => '🌧',
        0x8AAAA3 => '🌨',
        0x96A58A => '🌩',
        0x6B867B => '🌪',
        0xDBE3E7 => '🌫',
        0x7A9888 => '🌬',
        0x9A7F60 => '🌭',
        0xA6924A => '🌮',
        0x969A70 => '🌯',
        0x965E42 => '🌰',
        0x53804E => '🌱',
        0x4C763E => '🌲',
        0x527D3D => '🌳',
        0x597847 => '🌴',
        0x5F924E => '🌵',
        0x77604B => '🌶',
        0x818769 => '🌷',
        0xB39288 => '🌸',
        0x66543E => '🌹',
        0xB48F81 => '🌺',
        0x867B32 => '🌻',
        0x879E7D => '🌼',
        0x81903F => '🌽',
        0x60854D => '🌾',
        0x5F8453 => '🌿',
        0x649851 => '🍀',
        0x8F4F48 => '🍁',
        0x967E5F => '🍂',
        0x6D9D69 => '🍃',
        0xA67273 => '🍄',
        0xAD4847 => '🍅',
        0x5C6374 => '🍆',
        0x716795 => '🍇',
        0x79A767 => '🍈',
        0x796A4F => '🍉',
        0xBA851F => '🍊',
        0xB7AD4D => '🍋',
        0x969E67 => '🍌',
        0x768948 => '🍍',
        0xA04B46 => '🍎',
        0x63944E => '🍏',
        0x729B65 => '🍐',
        0xC2835F => '🍑',
        0x735B42 => '🍒',
        0x8B4E44 => '🍓',
        0xB08764 => '🍔',
        0x907742 => '🍕',
        0x857651 => '🍖',
        0x7D7450 => '🍗',
        0x83614B => '🍘',
        0x95A399 => '🍙',
        0xAA8D88 => '🍚',
        0x9AA17E => '🍛',
        0xAD9C76 => '🍜',
        0xA49B7B => '🍝',
        0xC0A984 => '🍞',
        0xAF5E3E => '🍟',
        0x946E51 => '🍠',
        0x7C8E69 => '🍡',
        0x70846B => '🍢',
        0x946660 => '🍣',
        0xBB8F46 => '🍤',
        0xB5B7BA => '🍥',
        0x929A61 => '🍦',
        0x9197A2 => '🍧',
        0x728C87 => '🍨',
        0x887045 => '🍩',
        0xAB8B6D => '🍪',
        0x8F6C5E => '🍫',
        0x8E6459 => '🍬',
        0xA17842 => '🍭',
        0x908F6B => '🍮',
        0xA78F40 => '🍯',
        0xB6B395 => '🍰',
        0x835A50 => '🍱',
        0x6A7754 => '🍲',
        0x687869 => '🍳',
        0x668673 => '🍴',
        0x84A47F => '🍵',
        0x64969B => '🍶',
        0x6D7A68 => '🍷',
        0x67856E => '🍸',
        0x7D8C5F => '🍹',
        0xD2B462 => '🍺',
        0xBEA55F => '🍻',
        0x859E90 => '🍼',
        0xBBC8CB => '🍽',
        0x657E59 => '🍾',
        0xAC967D => '🍿',
        0xA74346 => '🎀',
        0xCA9166 => '🎁',
        0xBA837D => '🎂',
        0xB1751B => '🎃',
        0x577643 => '🎄',
        0xC99E7F => '🎅',
        0x678E97 => '🎆',
        0x658D9A => '🎇',
        0x695D45 => '🎈',
        0x886455 => '🎉',
        0x80704D => '🎊',
        0x5F7044 => '🎋',
        0x7C8D7B => '🎌',
        0x729558 => '🎍',
        0x777C6A => '🎎',
        0x90908D => '🎏',
        0x608663 => '🎐',
        0x678D89 => '🎑',
        0x9A3638 => '🎒',
        0x4A5C48 => '🎓',
        0x817F5F => '🎖',
        0x8B8F4A => '🎗',
        0x577365 => '🎙',
        0xA1B4B7 => '🎚',
        0xA0A8AE => '🎛',
        0x708A85 => '🎞',
        0x90544E => '🎟',
        0x798870 => '🎠',
        0x71847E => '🎡',
        0x8A938E => '🎢',
        0x4F736B => '🎣',
        0x496E61 => '🎤',
        0x475F52 => '🎥',
        0x8AB8DA => '🎦',
        0x4A756E => '🎧',
        0x9E8A6A => '🎨',
        0x434A55 => '🎩',
        0x847B71 => '🎪',
        0x8CA498 => '🎫',
        0x587855 => '🎬',
        0x869C8A => '🎭',
        0x384C3E => '🎮',
        0xB97580 => '🎯',
        0x787C77 => '🎰',
        0x4D5A58 => '🎱',
        0x9A5E5A => '🎲',
        0x5E6B63 => '🎳',
        0x8B4F50 => '🎴',
        0x518D98 => '🎵',
        0x4E8584 => '🎶',
        0x888A50 => '🎷',
        0x6E6A59 => '🎸',
        0x787E84 => '🎹',
        0x86874E => '🎺',
        0x787846 => '🎻',
        0x577265 => '🎼',
        0x679DAD => '🎽',
        0x85B46F => '🎾',
        0x7E9797 => '🎿',
        0xA9731C => '🏀',
        0x6C8376 => '🏁',
        0x8B9797 => '🏂',
        0x747D4B => '🏃',
        0x63877B => '🏄',
        0x7F916E => '🏅',
        0xB19948 => '🏆',
        0x706F4D => '🏇',
        0x6E5140 => '🏈',
        0x8EAFB4 => '🏉',
        0x839B86 => '🏊',
        0x767A4A => '🏋',
        0x6E805A => '🏌',
        0x557856 => '🏍',
        0x6D7953 => '🏎',
        0x7A7457 => '🏏',
        0xB0BDBA => '🏐',
        0x796C49 => '🏑',
        0x586C44 => '🏒',
        0x995C50 => '🏓',
        0x506956 => '🏔',
        0x828F4D => '🏕',
        0x817B63 => '🏖',
        0x705847 => '🏗',
        0x8D9370 => '🏘',
        0x607A89 => '🏙',
        0x81785E => '🏚',
        0x8A9D98 => '🏛',
        0x8F9F5C => '🏜',
        0x708255 => '🏝',
        0x639290 => '🏞',
        0x81908D => '🏟',
        0x8F9D78 => '🏠',
        0x94A577 => '🏡',
        0xA1B0A7 => '🏢',
        0x90A6AC => '🏣',
        0xA5685D => '🏤',
        0x8EB0BF => '🏥',
        0x91A3A2 => '🏦',
        0x609DCB => '🏧',
        0xB1B397 => '🏨',
        0xBB98AF => '🏩',
        0x8B8189 => '🏪',
        0xACB295 => '🏫',
        0x99B1B6 => '🏬',
        0x905554 => '🏭',
        0x9E333C => '🏮',
        0x838B7D => '🏯',
        0x7B9999 => '🏰',
        0x93AA9D => '🏳',
        0x445B4E => '🏴',
        0xCB9334 => '🏵',
        0xA2A166 => '🏷',
        0x5A8174 => '🏸',
        0x707659 => '🏹',
        0x645E45 => '🏺',
        0xC0BCA5 => '🏻',
        0xBEB387 => '🏼',
        0xA99875 => '🏽',
        0x8E7953 => '🏾',
        0x6B5B42 => '🏿',
        0x5F7063 => '🐀',
        0x7A8D7A => '🐁',
        0x856E50 => '🐂',
        0x41594B => '🐃',
        0x5B7464 => '🐄',
        0x858141 => '🐅',
        0x8B8E48 => '🐆',
        0x799086 => '🐇',
        0x9A954C => '🐈',
        0x5B8D4A => '🐉',
        0x4F7C46 => '🐊',
        0x437F8A => '🐋',
        0x9D8D68 => '🐌',
        0x639250 => '🐍',
        0x6C684A => '🐎',
        0x95A68F => '🐏',
        0x87A08D => '🐐',
        0xA9B69E => '🐑',
        0x8F7053 => '🐒',
        0x9AA290 => '🐓',
        0x8A947A => '🐔',
        0x898061 => '🐕',
        0x9C8C82 => '🐖',
        0x975F48 => '🐗',
        0x839999 => '🐘',
        0x75669D => '🐙',
        0x8DA49B => '🐚',
        0x606A77 => '🐛',
        0x3C5545 => '🐜',
        0x6C8066 => '🐝',
        0x6F4A45 => '🐞',
        0x488695 => '🐟',
        0xAF9E46 => '🐠',
        0x847758 => '🐡',
        0x4D7B45 => '🐢',
        0xB5AE6E => '🐣',
        0x999549 => '🐤',
        0xA89D49 => '🐥',
        0x8F5545 => '🐦',
        0x858F7A => '🐧',
        0x6B867C => '🐨',
        0x698578 => '🐩',
        0x7E6C4D => '🐪',
        0x836C4D => '🐫',
        0x458197 => '🐬',
        0x848F8B => '🐭',
        0xBDACA6 => '🐮',
        0xB3A268 => '🐯',
        0x7D9085 => '🐰',
        0xC0A74E => '🐱',
        0x5C8E4B => '🐲',
        0x438297 => '🐳',
        0x91674D => '🐴',
        0x97775D => '🐵',
        0x8D725A => '🐶',
        0xCD8A91 => '🐷',
        0x80AD6E => '🐸',
        0xD0A364 => '🐹',
        0x70877E => '🐺',
        0xA66F56 => '🐻',
        0xA4ACA6 => '🐼',
        0x897D6A => '🐽',
        0x4F593B => '🐾',
        0x857349 => '🐿',
        0x9EB1A7 => '👀',
        0x73866F => '👁',
        0xAAA352 => '👂',
        0x979C51 => '👃',
        0x836859 => '👄',
        0x825E50 => '👅',
        0x949D53 => '👆',
        0x949D53 => '👇',
        0x949D53 => '👈',
        0x949D53 => '👉',
        0xAFA653 => '👊',
        0xB0B166 => '👋',
        0x939A52 => '👌',
        0xB5A954 => '👍',
        0xB5A954 => '👎',
        0xCAAD53 => '👏',
        0x959B52 => '👐',
        0xBD9E49 => '👑',
        0x889367 => '👒',
        0x456F61 => '👓',
        0x8894BE => '👔',
        0x4083A2 => '👕',
        0x3F7A85 => '👖',
        0x4F8FA1 => '👗',
        0x508BA3 => '👘',
        0x506561 => '👙',
        0x787A94 => '👚',
        0xC9636D => '👛',
        0x8A74A0 => '👜',
        0x79A069 => '👝',
        0x616345 => '👞',
        0x80675D => '👟',
        0x695746 => '👠',
        0x6E6A4D => '👡',
        0x846649 => '👢',
        0x554C32 => '👣',
        0x316A78 => '👤',
        0x3E7D91 => '👥',
        0xC5AA4B => '👦',
        0xC8A849 => '👧',
        0xBFA74A => '👨',
        0xCCA545 => '👩',
        0xDDAE53 => '👪',
        0x9A8F5D => '👫',
        0x84915F => '👬',
        0xA68D59 => '👭',
        0x6F8668 => '👮',
        0x818348 => '👯',
        0xDFD3AB => '👰',
        0xBDB341 => '👱',
        0x948C50 => '👲',
        0xA69966 => '👳',
        0xBCB35E => '👴',
        0x999762 => '👵',
        0xB9AB52 => '👶',
        0xA3A252 => '👷',
        0xC9A64C => '👸',
        0x7E4D43 => '👹',
        0x9B5E5A => '👺',
        0x98ABA2 => '👻',
        0xAFAD75 => '👼',
        0x8A9C97 => '👽',
        0x4C5864 => '👾',
        0x937FBC => '👿',
        0x8FA19C => '💀',
        0xBA9347 => '💁',
        0x5D6147 => '💂',
        0x7E6B48 => '💃',
        0x977956 => '💄',
        0x8B7955 => '💅',
        0xB89A4B => '💆',
        0xA49051 => '💇',
        0x758B7B => '💈',
        0x668973 => '💉',
        0x9E7048 => '💊',
        0x924E47 => '💋',
        0xA89C99 => '💌',
        0x61887C => '💍',
        0x5D949C => '💎',
        0xC49449 => '💏',
        0x757D46 => '💐',
        0xCA9251 => '💑',
        0x926C62 => '💒',
        0xA35348 => '💓',
        0xA24747 => '💔',
        0x8F4F48 => '💕',
        0xB25148 => '💖',
        0xB57272 => '💗',
        0xAB5E46 => '💘',
        0x5599B7 => '💙',
        0x679B54 => '💚',
        0xC1AD54 => '💛',
        0x8984A8 => '💜',
        0xC07B58 => '💝',
        0x9E5551 => '💞',
        0xE57D8B => '💟',
        0x538F9C => '💠',
        0x909B68 => '💡',
        0x695744 => '💢',
        0x435443 => '💣',
        0x457566 => '💤',
        0x836148 => '💥',
        0x5393A8 => '💦',
        0x4F888D => '💧',
        0x90B3B4 => '💨',
        0x96705A => '💩',
        0xB2AD55 => '💪',
        0x868C52 => '💫',
        0x91B5B9 => '💬',
        0x8BAEAD => '💭',
        0xBA8E8A => '💮',
        0x7E4640 => '💯',
        0xABA76F => '💰',
        0x44624C => '💱',
        0x3F5F45 => '💲',
        0xA68E4F => '💳',
        0xD8B873 => '💴',
        0x91B16C => '💵',
        0x6998AB => '💶',
        0xAE9BB8 => '💷',
        0x89A97C => '💸',
        0x97C47D => '💹',
        0x59878E => '💺',
        0x7BA6B4 => '💻',
        0x8A5427 => '💼',
        0xAE9B72 => '💽',
        0x7F8B93 => '💾',
        0x90A3A4 => '💿',
        0xD8BF74 => '📀',
        0x468CAD => '📁',
        0x4388AA => '📂',
        0xB6C5C5 => '📃',
        0xA7B9B7 => '📄',
        0xD1A3AD => '📅',
        0x9B8789 => '📆',
        0x80979A => '📇',
        0xC7B5B8 => '📈',
        0xACC6D1 => '📉',
        0xA9B9BA => '📊',
        0xA9A699 => '📋',
        0x80584D => '📌',
        0x646750 => '📍',
        0x5E806A => '📎',
        0x8C8F4A => '📏',
        0xA69F4C => '📐',
        0xA3B0B6 => '📑',
        0xDAAF54 => '📒',
        0x768285 => '📓',
        0xC59580 => '📔',
        0xBE3D4E => '📕',
        0x8BA6A5 => '📖',
        0x6E9F56 => '📗',
        0x5599C7 => '📘',
        0xE6A43E => '📙',
        0x9A9D9F => '📚',
        0xBF6D70 => '📛',
        0xCBB071 => '📜',
        0xB3B5A6 => '📝',
        0x3E5A46 => '📞',
        0x748E71 => '📟',
        0x798B8B => '📠',
        0x738773 => '📡',
        0x86746C => '📢',
        0x45818D => '📣',
        0x936D56 => '📤',
        0x848058 => '📥',
        0x867960 => '📦',
        0xA1B4AF => '📧',
        0x7F9C8D => '📨',
        0xA0BAC1 => '📩',
        0x798074 => '📪',
        0x798074 => '📫',
        0x84887D => '📬',
        0x62675A => '📭',
        0x926360 => '📮',
        0x877E42 => '📯',
        0xA6BBC6 => '📰',
        0x467782 => '📱',
        0x487D92 => '📲',
        0xF5AB49 => '📳',
        0xF6AE4F => '📴',
        0x6A4443 => '📵',
        0x5F9DCD => '📶',
        0x4A5E5C => '📷',
        0x656E5E => '📸',
        0x455C52 => '📹',
        0x447182 => '📺',
        0x6F8482 => '📻',
        0x3E544D => '📼',
        0x455A51 => '📽',
        0x4C5C5A => '📿',
        0x5F9ECD => '🔀',
        0x65A1CF => '🔁',
        0x67A2CF => '🔂',
        0x65A1CF => '🔃',
        0x6DA6D1 => '🔄',
        0x7A8044 => '🔅',
        0x898543 => '🔆',
        0x778372 => '🔇',
        0x6F8E7A => '🔈',
        0x718F7D => '🔉',
        0x789487 => '🔊',
        0x83A87B => '🔋',
        0x45644D => '🔌',
        0x73928C => '🔍',
        0x73928C => '🔎',
        0x838259 => '🔏',
        0xB0884C => '🔐',
        0x7B6D4D => '🔑',
        0x9A8F4F => '🔒',
        0x998F4E => '🔓',
        0xA18D3F => '🔔',
        0xA38340 => '🔕',
        0xC5B1B7 => '🔖',
        0x60806F => '🔗',
        0x417A98 => '🔘',
        0x3C5444 => '🔙',
        0x3A4F43 => '🔚',
        0x3A4F43 => '🔛',
        0x3C5444 => '🔜',
        0x3E5A46 => '🔝',
        0x775151 => '🔞',
        0x74AAD3 => '🔟',
        0x71A8D2 => '🔠',
        0x68A3D0 => '🔡',
        0x64A1CF => '🔢',
        0x72A9D3 => '🔣',
        0x5A9BCC => '🔤',
        0xAE8F33 => '🔥',
        0x5E7C63 => '🔦',
        0x5C7D69 => '🔧',
        0x627449 => '🔨',
        0x668571 => '🔩',
        0x62806A => '🔪',
        0x659052 => '🔫',
        0x6A8679 => '🔬',
        0x666555 => '🔭',
        0x897A9A => '🔮',
        0xB79DDC => '🔯',
        0x6D9E78 => '🔰',
        0x9B9A4C => '🔱',
        0x818588 => '🔲',
        0x94989B => '🔳',
        0xC03A45 => '🔴',
        0x52A0CF => '🔵',
        0xA2802A => '🔶',
        0x4E8FA1 => '🔷',
        0x6A763E => '🔸',
        0x497C6D => '🔹',
        0x6A6B53 => '🔺',
        0x6A6B53 => '🔻',
        0x609ECD => '🔼',
        0x609ECD => '🔽',
        0xA988D6 => '🕉',
        0x77957F => '🕊',
        0xB293DA => '🕎',
        0xA7B8B8 => '🕐',
        0xA7B8B8 => '🕑',
        0xA7B8B8 => '🕒',
        0xA7B8B7 => '🕓',
        0xA7B8B7 => '🕔',
        0xA7B8B8 => '🕕',
        0xA7B8B7 => '🕖',
        0xA7B8B7 => '🕗',
        0xA7B8B8 => '🕘',
        0xA7B8B8 => '🕙',
        0xA7B8B8 => '🕚',
        0xA9BAB9 => '🕛',
        0xA7B8B7 => '🕜',
        0xA7B8B7 => '🕝',
        0xA7B8B7 => '🕞',
        0xA7B8B7 => '🕟',
        0xA7B8B8 => '🕠',
        0xA9BAB9 => '🕡',
        0xA7B8B8 => '🕢',
        0xA7B8B8 => '🕣',
        0xA7B8B8 => '🕤',
        0xA7B8B7 => '🕥',
        0xA7B8B7 => '🕦',
        0xA7B8B8 => '🕧',
        0x81926D => '🕯',
        0x776A52 => '🕰',
        0x4A6852 => '🕳',
        0x4B6B4E => '🕴',
        0x5C6F5F => '🕵',
        0x426348 => '🕶',
        0x3E5946 => '🕷',
        0x51715D => '🕸',
        0x4F5D4C => '🕹',
        0x4F6546 => '🕺',
        0x698777 => '🖇',
        0x526847 => '🖊',
        0x4B6854 => '🖋',
        0x507365 => '🖌',
        0x7A5A4D => '🖍',
        0xA4A554 => '🖐',
        0x919B52 => '🖕',
        0xACA854 => '🖖',
        0x384941 => '🖤',
        0x98BFD8 => '🖥',
        0x759CA8 => '🖨',
        0x6F8980 => '🖱',
        0x697381 => '🖲',
        0x8A8882 => '🖼',
        0xCAC090 => '🗂',
        0x97A4A3 => '🗃',
        0x556A63 => '🗄',
        0x748C87 => '🗑',
        0x99AFB1 => '🗒',
        0xBAA6A8 => '🗓',
        0x805E54 => '🗜',
        0x46654E => '🗝',
        0x7A988B => '🗞',
        0x718260 => '🗡',
        0x3B6A73 => '🗣',
        0x75ACBE => '🗨',
        0x7A9386 => '🗯',
        0x7FA5A8 => '🗳',
        0x72A6A6 => '🗺',
        0x5A7265 => '🗻',
        0x6E6A55 => '🗼',
        0x3D9DA0 => '🗽',
        0x7FBEDF => '🗾',
        0x849A93 => '🗿',
        0xC3A746 => '😀',
        0xC6A949 => '😁',
        0xADA461 => '😂',
        0xC0A344 => '😃',
        0xC5A847 => '😄',
        0xBAA754 => '😅',
        0xC0A445 => '😆',
        0xC2B05D => '😇',
        0x937FBC => '😈',
        0xCCAC44 => '😉',
        0xD0A64E => '😊',
        0xD1A948 => '😋',
        0xCCAC45 => '😌',
        0xCB8142 => '😍',
        0xA99444 => '😎',
        0xCEAE45 => '😏',
        0xD1B147 => '😐',
        0xD3B348 => '😑',
        0xCCAD45 => '😒',
        0xC8B156 => '😓',
        0xCEAF46 => '😔',
        0xD2B248 => '😕',
        0xC6A741 => '😖',
        0xD1B047 => '😗',
        0xCBA244 => '😘',
        0xD2B248 => '😙',
        0xCEA54E => '😚',
        0xCBA14B => '😛',
        0xCBA65B => '😜',
        0xC89F4A => '😝',
        0xD1B147 => '😞',
        0xCBAB44 => '😟',
        0xCCAC44 => '😠',
        0xAB3B45 => '😡',
        0xC0AA52 => '😢',
        0xC6A742 => '😣',
        0xD3B965 => '😤',
        0xC3AC51 => '😥',
        0xD0B047 => '😦',
        0xCBAC44 => '😧',
        0xBAB06F => '😨',
        0xBFA343 => '😩',
        0xC9AF50 => '😪',
        0xBFA140 => '😫',
        0xC3A74A => '😬',
        0xB6A963 => '😭',
        0xCDAD45 => '😮',
        0xCBAC44 => '😯',
        0xB2B07C => '😰',
        0xC3B47F => '😱',
        0xC6A844 => '😲',
        0xCCAB63 => '😳',
        0xC3AA4E => '😴',
        0xC3A440 => '😵',
        0xD4B449 => '😶',
        0xD4C177 => '😷',
        0xC5A651 => '😸',
        0xB8A562 => '😹',
        0xCAAA52 => '😺',
        0xCB8451 => '😻',
        0xD0AD4D => '😼',
        0xD0AD4D => '😽',
        0xCEAB4D => '😾',
        0xC0A95C => '😿',
        0xD7B260 => '🙀',
        0xD0B046 => '🙁',
        0xD0B046 => '🙂',
        0xCEAE46 => '🙃',
        0xD1B85E => '🙄',
        0xBA8B44 => '🙅',
        0xD39347 => '🙆',
        0xBD984F => '🙇',
        0xA2765A => '🙈',
        0xA4775C => '🙉',
        0xA17A5F => '🙊',
        0xC29448 => '🙋',
        0xA6A85A => '🙌',
        0xA98F48 => '🙍',
        0xA99048 => '🙎',
        0x8A9F6C => '🙏',
        0x66736E => '🚀',
        0x82936A => '🚁',
        0x6F5952 => '🚂',
        0x80976D => '🚃',
        0x61868C => '🚄',
        0x789AA0 => '🚅',
        0x899B91 => '🚆',
        0x565656 => '🚇',
        0x838A75 => '🚈',
        0x6C8375 => '🚉',
        0x698473 => '🚊',
        0x9C7D88 => '🚋',
        0x607C77 => '🚌',
        0x8D9B89 => '🚍',
        0x70937C => '🚎',
        0x5F6F54 => '🚏',
        0x7C9C9F => '🚐',
        0x909390 => '🚑',
        0x926B69 => '🚒',
        0x5F7E70 => '🚓',
        0x698489 => '🚔',
        0x7E8953 => '🚕',
        0xA0A26B => '🚖',
        0x7A6154 => '🚗',
        0x598C98 => '🚘',
        0x4B7D82 => '🚙',
        0x938E7B => '🚚',
        0x6F8F5F => '🚛',
        0x637D51 => '🚜',
        0x839E92 => '🚝',
        0xA1AF94 => '🚞',
        0x837F61 => '🚟',
        0x648A89 => '🚠',
        0x73706C => '🚡',
        0x769191 => '🚢',
        0x668681 => '🚣',
        0x668583 => '🚤',
        0x4E5E46 => '🚥',
        0x4E5E46 => '🚦',
        0x747E52 => '🚧',
        0xBA636D => '🚨',
        0x68624E => '🚩',
        0x89674B => '🚪',
        0x974C47 => '🚫',
        0x6D876D => '🚬',
        0x633D3B => '🚭',
        0x67A3D0 => '🚮',
        0x704A49 => '🚯',
        0x5589B0 => '🚰',
        0x694342 => '🚱',
        0x5B5C48 => '🚲',
        0x754F4E => '🚳',
        0x6D714F => '🚴',
        0x859356 => '🚵',
        0x5A7552 => '🚶',
        0x613C3A => '🚷',
        0x77753A => '🚸',
        0x4780A9 => '🚹',
        0xEC7889 => '🚺',
        0x7FB1D7 => '🚻',
        0xF5A841 => '🚼',
        0x8CA498 => '🚽',
        0x5186AE => '🚾',
        0x558079 => '🚿',
        0x859C89 => '🛀',
        0x7E9A89 => '🛁',
        0x6292B6 => '🛂',
        0x6292B5 => '🛃',
        0x77A0BF => '🛄',
        0x729DBD => '🛅',
        0x6A8C4D => '🛋',
        0x688170 => '🛌',
        0x8D5967 => '🛍',
        0x978243 => '🛎',
        0x5D7863 => '🛏',
        0xAC8BD7 => '🛐',
        0xB35C61 => '🛑',
        0x587563 => '🛒',
        0x796A42 => '🛕',
        0xA3925F => '🛖',
        0x6D9ABB => '🛗',
        0x747F5D => '🛠',
        0x5890A9 => '🛡',
        0x3C798C => '🛢',
        0x86ADA6 => '🛣',
        0x8DB5B2 => '🛤',
        0x618C85 => '🛥',
        0x7B675A => '🛩',
        0x779B84 => '🛫',
        0x779B85 => '🛬',
        0x6E9285 => '🛰',
        0x6295A6 => '🛳',
        0x507162 => '🛴',
        0x675C49 => '🛵',
        0x647D7E => '🛶',
        0x979283 => '🛷',
        0x98A88F => '🛸',
        0x746158 => '🛹',
        0x637942 => '🛺',
        0x776556 => '🛻',
        0x7B8174 => '🛼',
        0xD28918 => '🟠',
        0xDAB955 => '🟡',
        0x6EA456 => '🟢',
        0x9788BB => '🟣',
        0xA96A4E => '🟤',
        0xDC2E44 => '🟥',
        0x54ABED => '🟦',
        0xF28F0C => '🟧',
        0xFBCA57 => '🟨',
        0x77B058 => '🟩',
        0xA98DD5 => '🟪',
        0xC0694E => '🟫',
        0xB5AA53 => '🤌',
        0xB2C0B4 => '🤍',
        0x996B4E => '🤎',
        0xAAA352 => '🤏',
        0xCDB152 => '🤐',
        0xBFA948 => '🤑',
        0xC8A94C => '🤒',
        0xB69E49 => '🤓',
        0xC4A342 => '🤔',
        0xCBB668 => '🤕',
        0x769CAC => '🤖',
        0xCB9E35 => '🤗',
        0x8F9751 => '🤘',
        0xA1A253 => '🤙',
        0xA8A754 => '🤚',
        0x9EA153 => '🤛',
        0x9FA153 => '🤜',
        0x989B50 => '🤝',
        0x8B9551 => '🤞',
        0xAAA854 => '🤟',
        0x958333 => '🤠',
        0xCBAF96 => '🤡',
        0x679B4E => '🤢',
        0xA59B63 => '🤣',
        0xCAAE4B => '🤤',
        0xB7A043 => '🤥',
        0xAC8B46 => '🤦',
        0xCBB461 => '🤧',
        0xCBAB44 => '🤨',
        0xCA9541 => '🤩',
        0xC9AB67 => '🤪',
        0xC0A243 => '🤫',
        0x98474E => '🤬',
        0xC8A43D => '🤭',
        0xAAA348 => '🤮',
        0xBBA664 => '🤯',
        0x747867 => '🤰',
        0x84885A => '🤱',
        0xBAAF55 => '🤲',
        0x737751 => '🤳',
        0xA6A15C => '🤴',
        0x8A8A53 => '🤵',
        0xAE996F => '🤶',
        0xBB9247 => '🤷',
        0x6F7A4B => '🤸',
        0xA48D52 => '🤹',
        0x5F8168 => '🤺',
        0x8B8C54 => '🤼',
        0x7A9282 => '🤽',
        0x6F7C4B => '🤾',
        0x6C835D => '🤿',
        0x717C58 => '🥀',
        0xBE7363 => '🥁',
        0x9DA374 => '🥂',
        0x8B9575 => '🥃',
        0x597C63 => '🥄',
        0x90867F => '🥅',
        0x7C8963 => '🥇',
        0x6A9494 => '🥈',
        0x787E66 => '🥉',
        0x945353 => '🥊',
        0xA8B7B5 => '🥋',
        0x687067 => '🥌',
        0x49705F => '🥍',
        0xC1BA4E => '🥎',
        0xAB9854 => '🥏',
        0x958337 => '🥐',
        0x698A52 => '🥑',
        0x598A4D => '🥒',
        0x86755F => '🥓',
        0x9E8A6B => '🥔',
        0x857F36 => '🥕',
        0x81895C => '🥖',
        0x7E9975 => '🥗',
        0x88713B => '🥘',
        0xA39461 => '🥙',
        0x9DA388 => '🥚',
        0x8DA497 => '🥛',
        0x918262 => '🥜',
        0x838959 => '🥝',
        0xAC9257 => '🥞',
        0xB6AC66 => '🥟',
        0xBB964A => '🥠',
        0x9C9B98 => '🥡',
        0x586A48 => '🥢',
        0x715A4D => '🥣',
        0x7A989A => '🥤',
        0x807F68 => '🥥',
        0x4A7A39 => '🥦',
        0x9F7737 => '🥧',
        0x988032 => '🥨',
        0x8A7659 => '🥩',
        0xA8994D => '🥪',
        0x819961 => '🥫',
        0x60914D => '🥬',
        0xB5674C => '🥭',
        0x87632A => '🥮',
        0xA0795E => '🥯',
        0xD69844 => '🥰',
        0xBF9B37 => '🥱',
        0xC4AE54 => '🥲',
        0xC29A57 => '🥳',
        0xC7A844 => '🥴',
        0xAB697A => '🥵',
        0x5A9BC1 => '🥶',
        0x445944 => '🥷',
        0x9F8A42 => '🥸',
        0xCCB055 => '🥺',
        0x6A8753 => '🥻',
        0xAEBFB9 => '🥼',
        0x799584 => '🥽',
        0x625B42 => '🥾',
        0x476E44 => '🥿',
        0x833938 => '🦀',
        0x926D3E => '🦁',
        0x5F6547 => '🦂',
        0x535A49 => '🦃',
        0x879491 => '🦄',
        0x7C8D7A => '🦅',
        0x728562 => '🦆',
        0x607C6F => '🦇',
        0x587268 => '🦈',
        0x856A45 => '🦉',
        0xA57D3A => '🦊',
        0x406F7C => '🦋',
        0x706045 => '🦌',
        0x455752 => '🦍',
        0x4C7A42 => '🦎',
        0x879B99 => '🦏',
        0x7F5145 => '🦐',
        0xA37F7A => '🦑',
        0x818342 => '🦒',
        0x6B7B74 => '🦓',
        0x7C6249 => '🦔',
        0x59894D => '🦕',
        0x59884C => '🦖',
        0x6C7B60 => '🦗',
        0x777E5D => '🦘',
        0x797F5E => '🦙',
        0x679B7C => '🦚',
        0x6C897B => '🦛',
        0x5A804F => '🦜',
        0x6F817D => '🦝',
        0x7D4E43 => '🦞',
        0x567360 => '🦟',
        0x588946 => '🦠',
        0x587662 => '🦡',
        0x95AB9A => '🦢',
        0x886A4C => '🦣',
        0x6B856B => '🦤',
        0x797662 => '🦥',
        0x717E65 => '🦦',
        0xB28140 => '🦧',
        0x4F6657 => '🦨',
        0x7F6E5E => '🦩',
        0xA29C73 => '🦪',
        0x755F43 => '🦫',
        0x71583E => '🦬',
        0x5D7D6A => '🦭',
        0x7B7C56 => '🦮',
        0x4F7252 => '🦯',
        0xCF7637 => '🦰',
        0xD7A23B => '🦱',
        0xC8BC57 => '🦲',
        0xCAD0B8 => '🦳',
        0x758A64 => '🦴',
        0x899651 => '🦵',
        0x939C52 => '🦶',
        0x95AD9E => '🦷',
        0x88845E => '🦸',
        0x8F7647 => '🦹',
        0xC89037 => '🦺',
        0xAAA76A => '🦻',
        0x53644E => '🦼',
        0x4F654E => '🦽',
        0x7E998C => '🦾',
        0x6D8472 => '🦿',
        0xBF9C44 => '🧀',
        0xA0A481 => '🧁',
        0x89A192 => '🧂',
        0x989655 => '🧃',
        0x8BA495 => '🧄',
        0xA4A272 => '🧅',
        0x936B3A => '🧆',
        0x998D50 => '🧇',
        0x8B9A6F => '🧈',
        0x747055 => '🧉',
        0x9BBDBB => '🧊',
        0x758B7B => '🧋',
        0x627853 => '🧍',
        0x5C7757 => '🧎',
        0xB2954B => '🧏',
        0xBEA95A => '🧐',
        0xB5A049 => '🧑',
        0xA0974A => '🧒',
        0xA8AC79 => '🧓',
        0xBFA044 => '🧔',
        0x737474 => '🧕',
        0xB8A470 => '🧖',
        0x929586 => '🧗',
        0x89894F => '🧘',
        0xA28450 => '🧙',
        0xB4A770 => '🧚',
        0x7A6846 => '🧛',
        0x73834E => '🧜',
        0x86703F => '🧝',
        0x6F888E => '🧞',
        0x6D765C => '🧟',
        0xA9857F => '🧠',
        0xBB8520 => '🧡',
        0x3D7A8C => '🧢',
        0x7E5145 => '🧣',
        0x68827A => '🧤',
        0x4B91B2 => '🧥',
        0x487A79 => '🧦',
        0xA05B3D => '🧧',
        0x796D4E => '🧨',
        0x619252 => '🧩',
        0x619375 => '🧪',
        0x8D867D => '🧫',
        0x5A796B => '🧬',
        0xCCB27E => '🧭',
        0x8D8567 => '🧮',
        0x6E6453 => '🧯',
        0xB24550 => '🧰',
        0x896A51 => '🧱',
        0x7E5E4D => '🧲',
        0x4C8289 => '🧳',
        0x9C9151 => '🧴',
        0x6C5A75 => '🧵',
        0x3D7894 => '🧶',
        0x5F806B => '🧷',
        0x83684C => '🧸',
        0x6D7E52 => '🧹',
        0x816C4F => '🧺',
        0xB7C7C0 => '🧻',
        0xC1A5A2 => '🧼',
        0xB4993E => '🧽',
        0xA3B6B3 => '🧾',
        0x5766B1 => '🧿',
        0xA68780 => '🩰',
        0x5E6E71 => '🩱',
        0x397585 => '🩲',
        0x689C52 => '🩳',
        0x578881 => '🩴',
        0x825549 => '🩸',
        0x968C6F => '🩹',
        0x4B6D53 => '🩺',
        0x9C5755 => '🪀',
        0x4F8197 => '🪁',
        0x7B6153 => '🪂',
        0x777F61 => '🪃',
        0x5A744E => '🪄',
        0x737C69 => '🪅',
        0x6F6851 => '🪆',
        0x8D7D37 => '🪐',
        0x826E4F => '🪑',
        0x5C8756 => '🪒',
        0x65784F => '🪓',
        0x896B3D => '🪔',
        0x6D805E => '🪕',
        0x606C3E => '🪖',
        0xA67B6B => '🪗',
        0x7A6544 => '🪘',
        0xD5AB48 => '🪙',
        0x687E63 => '🪚',
        0x646650 => '🪛',
        0x646648 => '🪜',
        0x71814B => '🪝',
        0x95997A => '🪞',
        0xB2BBCB => '🪟',
        0x677156 => '🪠',
        0x597566 => '🪡',
        0x558984 => '🪢',
        0x5991A7 => '🪣',
        0x77532E => '🪤',
        0x4B7C60 => '🪥',
        0x92A59C => '🪦',
        0x7F9681 => '🪧',
        0x8AA199 => '🪨',
        0x607467 => '🪰',
        0x84635D => '🪱',
        0x568845 => '🪲',
        0x595237 => '🪳',
        0x6E7E4F => '🪴',
        0x675B3F => '🪵',
        0x767253 => '🪶',
        0x885C54 => '🫀',
        0x9E635E => '🫁',
        0x3E83A5 => '🫂',
        0x4D6878 => '🫐',
        0x598B48 => '🫑',
        0x597743 => '🫒',
        0xB49566 => '🫓',
        0x999651 => '🫔',
        0x725F52 => '🫕',
        0x3F7677 => '🫖',
        0x685744 => '‼',
        0x754D41 => '⁉',
        0x405E47 => '™',
        0x4990C6 => 'ℹ',
        0x63A0CE => '↔',
        0x63A0CE => '↕',
        0x5E9DCD => '↖',
        0x5E9DCD => '↗',
        0x5E9DCD => '↘',
        0x5E9DCD => '↙',
        0x5A9BCC => '↩',
        0x5A9BCC => '↪',
        0x778F80 => '⌚',
        0x849470 => '⌛',
        0x5F796E => '⌨',
        0x67A2CF => '⏏',
        0x63A0CE => '⏩',
        0x63A0CE => '⏪',
        0x63A0CE => '⏫',
        0x63A0CE => '⏬',
        0x70A8D2 => '⏭',
        0x70A8D2 => '⏮',
        0x6BA5D1 => '⏯',
        0xBE8E7C => '⏰',
        0x7C9083 => '⏱',
        0xACB5B5 => '⏲',
        0x849570 => '⏳',
        0x5C9CCC => '⏸',
        0x84B4D8 => '⏹',
        0x75ABD4 => '⏺',
        0x71A3BC => 'Ⓜ',
        0x436749 => '▪',
        0x5F8163 => '▫',
        0x609ECD => '▶',
        0x609ECD => '◀',
        0xB5C2B7 => '◻',
        0x374841 => '◼',
        0x829C86 => '◽',
        0x3E5A46 => '◾',
        0x928841 => '☀',
        0xA4B8AE => '☁',
        0x5B6871 => '☂',
        0x879382 => '☃',
        0x629391 => '☄',
        0xAE4D51 => '☎',
        0x3877A5 => '☑',
        0x5A717E => '☔',
        0xACA69D => '☕',
        0x5D8F50 => '☘',
        0x879351 => '☝',
        0x80988E => '☠',
        0xC69B49 => '☢',
        0xC59741 => '☣',
        0xA886D5 => '☦',
        0xB193DA => '☪',
        0xB091D9 => '☮',
        0xB395DB => '☯',
        0xAF8FD9 => '☸',
        0xCEAE46 => '☹',
        0xCEA44E => '☺',
        0xEC7183 => '♀',
        0x4880AA => '♂',
        0xA27DD3 => '♈',
        0xA480D3 => '♉',
        0xA784D5 => '♊',
        0xA480D4 => '♋',
        0xA784D5 => '♌',
        0xAA88D6 => '♍',
        0xA785D5 => '♎',
        0xA581D4 => '♏',
        0xA27ED3 => '♐',
        0xA480D4 => '♑',
        0xAC8BD7 => '♒',
        0xA480D3 => '♓',
        0x3F5A48 => '♟',
        0x3A4F43 => '♠',
        0x3B5143 => '♣',
        0x85423D => '♥',
        0x754D41 => '♦',
        0x7E5749 => '♨',
        0x588949 => '♻',
        0x5A9DC3 => '♾',
        0x4E84AC => '♿',
        0x7B7746 => '⚒',
        0x3B6C64 => '⚓',
        0x818E73 => '⚔',
        0xA480D3 => '⚕',
        0x7E8149 => '⚖',
        0x7E9F9C => '⚗',
        0x557264 => '⚙',
        0xA987D6 => '⚛',
        0x676C83 => '⚜',
        0x9A9549 => '⚠',
        0x7E8244 => '⚡',
        0xA682D4 => '⚧',
        0xC7D0CA => '⚪',
        0x35413F => '⚫',
        0x8F7757 => '⚰',
        0x887253 => '⚱',
        0x86928E => '⚽',
        0xC0B8B7 => '⚾',
        0x818C73 => '⛄',
        0xB1B08A => '⛅',
        0x98AA99 => '⛈',
        0xA784D5 => '⛎',
        0x697647 => '⛏',
        0x8D5B50 => '⛑',
        0x8EA898 => '⛓',
        0xB25059 => '⛔',
        0x704F45 => '⛩',
        0x857E56 => '⛪',
        0x425B49 => '⛰',
        0x907B5E => '⛱',
        0x689299 => '⛲',
        0x667D4B => '⛳',
        0x70929F => '⛴',
        0x889475 => '⛵',
        0x8EA5AB => '⛷',
        0x92A496 => '⛸',
        0x7B7E4B => '⛹',
        0x95873F => '⛺',
        0x999472 => '⛽',
        0x73715F => '✂',
        0x8ABC6D => '✅',
        0x689391 => '✈',
        0xA5B8B3 => '✉',
        0xCFBC57 => '✊',
        0xA7A553 => '✋',
        0x8F9851 => '✌',
        0x88945D => '✍',
        0x858A59 => '✏',
        0x4B6855 => '✒',
        0x406047 => '✔',
        0x3E5A46 => '✖',
        0xA07AD2 => '✝',
        0xA885D5 => '✡',
        0x838646 => '✨',
        0xA4CB8E => '✳',
        0x406047 => '✴',
        0x5D8F88 => '❄',
        0x98C47E => '❇',
        0x7F5748 => '❌',
        0x97C47D => '❎',
        0x655A45 => '❓',
        0x688970 => '❔',
        0x597E60 => '❕',
        0x576348 => '❗',
        0x815648 => '❣',
        0xAB4346 => '❤',
        0x416048 => '➕',
        0x436749 => '➖',
        0x416248 => '➗',
        0x5C9BCC => '➡',
        0x416148 => '➰',
        0x4E8198 => '➿',
        0x5094C8 => '⤴',
        0x5094C8 => '⤵',
        0x5C9BCC => '⬅',
        0x5C9BCC => '⬆',
        0x5C9BCC => '⬇',
        0x31373D => '⬛',
        0xE5E6E7 => '⬜',
        0x958941 => '⭐',
        0x8F5048 => '⭕',
        0x436849 => '〰',
        0x7B8A4C => '〽',
        0xC66269 => '㊗',
        0xC7656B => '㊙',
        0x3E5A46 => '©',
        0x3D5745 => '®',
        0xAAAEAE => '',
    );

}
