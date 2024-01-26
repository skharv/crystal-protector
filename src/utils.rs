pub const COLOUR_PLAYER: [u8; 4] = [255, 194, 122, 255];
pub const COLOUR_DARK_FLOOR: [u8; 4] = [53, 93, 104, 255];
pub const COLOUR_LIGHT_FLOOR: [u8; 4] = [66, 116, 120, 255];
pub const COLOUR_OUTER_CRYSTAL: [u8; 4] = [194, 159, 214, 255];
pub const COLOUR_INNER_CRYSTAL: [u8; 4] = [212, 80, 172, 255];
pub const COLOUR_BEAM: [u8; 4] = [167, 49, 105, 255];
pub const COLOUR_DEEP: [u8; 4] = [32, 17, 39, 255];
pub const COLOUR_MEDIUM: [u8; 4] = [32, 20, 51, 255];
pub const COLOUR_SHALLOW: [u8; 4] = [27, 30, 52, 255];
pub const COLOUR_RESOURCE: [u8; 4] = [236, 154, 109, 255];
pub const COLOUR_RICH_RESOURCE: [u8; 4] = [217, 98, 107, 255];
pub const COLOUR_SPREAD: [u8; 4] = [148, 197, 172, 255];
pub const COLOUR_UI: [u8; 4] = [255, 235, 153, 255];

pub const SYMBOL_RESOURCE: &str = "08181C3C3466663C";
pub const SYMBOL_HOUSE: &str = "182442FF425A5A7E";
pub const SYMBOL_BOMB: &str = "01091670F8F8F870";
pub const SYMBOL_FACE: &str = "7E81A5A58199817E";
pub const SYMBOL_BUBBLE: &str = "3C4289858581423C";
pub const SYMBOL_BASE_SHAPE: &str = "C3810000000081C3";
pub const SYMBOL_OUTER_CRYSTAL: &str = "005A3C7E7E3C5A00";
pub const SYMBOL_INNER_CRYSTAL: &str = "0000183C3C180000";
pub const SYMBOL_SAD: &str = "00000066003C2400";
pub const SYMBOL_HAPPY: &str = "0000242400243C00";
pub const SYMBOL_W: &str = "7E81A5A5B5BD817E";
pub const SYMBOL_A: &str = "7E8199A5BDA5817E";
pub const SYMBOL_S: &str = "7E81BDB18DBD817E";
pub const SYMBOL_D: &str = "7E81B9A5A5B9817E";
pub const SYMBOL_1: &str = "7E818999899D817E";
pub const SYMBOL_4: &str = "7E81A9A9BD89817E";
pub const SYMBOL_SQUIGGLE: &str = "0008100810081000";
pub const SYMBOL_L_BUTTON: &str = "3C72727E4242423C";
pub const SYMBOL_R_BUTTON: &str = "3C4E4E7E4242423C";
pub const SYMBOL_LASER: &str = "5088209048040201";
pub const SYMBOL_HEART: &str = "0066F9FDFF7E3C18";
pub const SYMBOL_ARROW: &str = "001818187E3C1800";
pub const SYMBOL_YOU: &str = "0000000008000000";

pub const AUTOMATON_SEEK_RANGE: f32 = 20.0;
pub const CRYSTAL_COUNT: i32 = 4;
pub const CRYSTAL_SECTIONS: i32 = 28;

#[derive(Clone, Copy)]
pub enum Action {
    House = 100,
    Bomb = 110,
    Face = 80,
    Bubble = 50
}


pub fn convert_string_to_symbol(
    input: &String
    ) -> [bool; crate::SYMBOL_SIZE * crate::SYMBOL_SIZE] {
    let mut output = [false; crate::SYMBOL_SIZE * crate::SYMBOL_SIZE]; 
    for i in 0..input.len() {
        if let Some(input_char) = input.chars().nth(i) {
            let bools = binary_lookup(input_char);
            for j in 0..bools.len() {
                output[i*4 + j] = bools[j];
            }
        }
    }
    return output;
}

pub fn is_position_part_of_symbol(
    x: i32,
    y: i32,
    symbol: [bool; crate::SYMBOL_SIZE * crate::SYMBOL_SIZE]
    ) -> bool {
    for i in 0..symbol.len() {
        if symbol[i] {
            let symbol_x = (i % crate::SYMBOL_SIZE) as i32;
            let symbol_y = (i / crate::SYMBOL_SIZE) as i32;
            
            if x == symbol_x && y == symbol_y {
                return true;
            }
        }
    }
    return false;
}

pub fn binary_lookup(
    input: char
    ) -> [bool; 4] {
    match input {
        '0' => [false, false, false, false],
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'A' => [true, false, true, false],
        'B' => [true, false, true, true],
        'C' => [true, true, false, false],
        'D' => [true, true, false, true],
        'E' => [true, true, true, false],
        'F' => [true, true, true, true],
        _ => [false, false, false, false],
    }
}


