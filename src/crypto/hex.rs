use std::fmt::{write, Debug, Display, Formatter};
use std::str::FromStr;

static CHARS: &[u8] = b"0123456789abcdef";

pub trait HexDisplay {
    fn hex_str(&self) -> String;
}

impl HexDisplay for dyn AsRef<[u8]> {
    fn hex_str(&self) -> String {
        let data = self.as_ref();
        format(data)
    }
}

impl HexDisplay for [u8] {
    fn hex_str(&self) -> String {
        format(&self)
    }
}

impl HexDisplay for Vec<u8> {
    fn hex_str(&self) -> String {
        format(&self)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hex(Vec<u8>);

impl HexDisplay for Hex {
    fn hex_str(&self) -> String {
        self.0.hex_str()
    }
}

impl Display for Hex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.hex_str())
    }
}

impl FromStr for Hex {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = parse(s);
        data.map(|v| Self(v))
    }
}

fn format(data: &[u8]) -> String {
    let mut hex = String::new();
    let mut h: u8 = 0;
    let mut l: u8 = 0;
    for n in data {
        l = n & 0x0f;
        h = (n >> 4) & 0x0f;
        hex.push(CHARS[h as usize] as char);
        hex.push(CHARS[l as usize] as char);
    }
    return hex;
}

fn parse_number_from_ascii(ch: u8) -> Result<u8, &'static str> {
    if ch >= 0x30 && ch <= 0x39 {
        return Ok(ch - 0x30);
    }
    if ch >= 0x41 && ch <= 0x46 {
        return Ok(ch - 0x41 + 10);
    }
    if ch >= 0x61 && ch <= 0x66 {
        return Ok(ch - 0x61 + 10);
    }
    return Err("illegal hex character");
}

fn parse(hex_str: impl AsRef<str>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let arr = hex_str.as_ref().as_bytes();
    if arr.len() % 2 != 0 {
        return Err("illegal length of characters".into());
    }
    let n = arr.len() / 2;
    let mut vec: Vec<u8> = Vec::with_capacity(n);
    let mut ch: u8 = 0;
    let mut cl: u8 = 0;
    for i in 0..n {
        ch = parse_number_from_ascii(arr[i * 2])?;
        cl = parse_number_from_ascii(arr[i * 2 + 1])?;
        vec.push((ch << 4) + cl);
    }
    Ok(vec)
}

#[cfg(test)]
mod test {
    use crate::crypto::hex::{Hex, HexDisplay};

    #[test]
    fn test() {
        let data: [u8; 10] = [78, 84, 65, 72, 32, 14, 5, 84, 65, 12];
        let str: String = data.hex_str();
        let data2: Hex = str.parse().unwrap();
        let str2 = data2.hex_str();
        assert_eq!(str, str2);
    }
}
