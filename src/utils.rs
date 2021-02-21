use std::convert::TryInto;


pub fn str_to_bytes_sized<const L: usize>(s: &str) -> [u8; L] {
    let mut v = s.as_bytes().to_vec();
    v.resize(L, 0u8);
    v.try_into().unwrap()
}


pub fn str_from_bytes(bytes: &[u8]) -> String {
    let mut bytes_truncated: Vec<u8> =
        bytes.to_vec().into_iter().rev().skip_while(|&x| x == 0u8).collect();
    bytes_truncated.reverse();
    String::from_utf8(bytes_truncated).unwrap()
}


pub fn hex_to_bytes_vec(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2).rev().map(
        |i| u8::from_str_radix(&hex[i..(i + 2)], 16).unwrap()
    ).collect()
}


pub fn hex_to_bytes<const L: usize>(hex: &str) -> [u8; L] {
    hex_to_bytes_vec(hex).try_into().unwrap()
}


pub fn hex_from_bytes(bytes: &[u8]) -> String {
    bytes.iter().rev().map(|b| format!("{:02X?}", b)).collect()
}
