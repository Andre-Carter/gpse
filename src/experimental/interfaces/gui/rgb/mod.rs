pub const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }

    /// "#RRGGBB"
    pub fn to_hex(self) -> String {
        let mut s = String::with_capacity(7);
        s.push('#');
        for byte in [self.r, self.g, self.b] {
            s.push(HEX_DIGITS[(byte >> 4) as usize] as char);
            s.push(HEX_DIGITS[(byte & 0x0F) as usize] as char);
        }
        s
    }

    /// Parses "#RRGGBB" or "RRGGBB"
    pub fn from_hex(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 || !s.is_ascii() {
            return None;
        }
        let byte = |i: usize| u8::from_str_radix(&s[i..i + 2], 16).ok();
        Some(Rgb::new(byte(0)?, byte(2)?, byte(4)?))
    }
}