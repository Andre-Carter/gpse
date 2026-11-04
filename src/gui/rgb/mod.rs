use crate::core::input::text::characters;
use std::str::Chars;

pub const RGB_HEX_MAP: &str = "#RRGGBB";

pub struct Decimal {
    r: u8,
    g: u8,
    b: u8,
}

impl Decimal {
    pub fn new() -> Decimal { 
        Decimal {
            r: 0,
            g: 0,
            b: 0,
        } 
    }

    pub fn push(code: u8) {
        
    }
}

pub fn decimal_code<T>(decimal: Decimal) -> Vec<T> {
    let code_array = Vec::new();

    code_array

}

pub struct Red<'a> {
    red_one: &'a str,
    red_two: &'a str,
}

pub struct Green<'a> {
    green_one: &'a str,
    green_two: &'a str,
}

pub struct Blue<'a> {
    blue_one: &'a str,
    blue_two: &'a str,
}

pub struct RGB<'a> {
    r: Red<'a>,
    g: Green<'a>,
    b: Blue<'a>,
}

pub struct HexCode<'a> {
    r1: &'a str,
    r2: &'a str,
    g1: &'a str,
    g2: &'a str,
    b1: &'a str, 
    b2: &'a str,
}

pub struct HexCodeChars<'a> {
    r1: &'a char,
    r2: &'a char,
    g1: &'a char,
    g2: &'a char,
    b1: &'a char, 
    b2: &'a char,
}

pub fn rgb_code() {
    let black: HexCode = HexCode {
        r1: "0",
        r2: "0",
        g1: "0",
        g2: "0",
        b1: "0",
        b2: "0",
    };
}

pub const HEX_MAP: &str = "0123456789ABCDEF";

pub const HEX_MAPP: [&str; 16] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];

pub fn hex_pair_map() -> Chars<'static> {
    let hex_pair_map = characters(&HEX_MAP);

    hex_pair_map
}

pub fn hex_map() {
    let hex_map = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];

    let get_hex_map = hex_map[0];

    println!("{get_hex_map}");
}