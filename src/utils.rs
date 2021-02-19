use bigi::Bigi;
use bigi_ecc::{point, Point};

use crate::HASH_STORAGE_BITS;

const BIGI_HEX_LENGTH: usize = HASH_STORAGE_BITS / 4;


pub fn string_to_bytes_fixed(s: &str, len: usize) -> Vec<u8> {
    let mut res = s.as_bytes().to_vec();
    res.resize(len, 0u8);
    res
}


pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2).map(
        |i| u8::from_str_radix(&hex[i..(i + 2)], 16).unwrap()
    ).collect()
}


pub fn hex_to_bigi(hex: &str) -> Bigi {
    Bigi::from_bytes(&hex_to_bytes(&hex[..BIGI_HEX_LENGTH]))
}


pub fn hex_to_point(hex: &str) -> Point {
    point!(
        hex_to_bigi(&hex[..BIGI_HEX_LENGTH]),
        hex_to_bigi(&hex[BIGI_HEX_LENGTH..])
    )
}


pub fn hex_to_bigi_pair(hex: &str) -> (Bigi, Bigi) {
    (
        hex_to_bigi(&hex[..BIGI_HEX_LENGTH]),
        hex_to_bigi(&hex[BIGI_HEX_LENGTH..])
    )
}
