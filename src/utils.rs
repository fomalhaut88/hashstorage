use std::convert::TryInto;


pub fn str_to_bytes_sized<const L: usize>(s: &str) -> [u8; L] {
    let mut v = s.as_bytes().to_vec();
    v.resize(L, 0u8);
    v.try_into().unwrap()
}


pub fn hex_to_bytes_vec(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2).map(
        |i| u8::from_str_radix(&hex[i..(i + 2)], 16).unwrap()
    ).collect()
}


pub fn hex_to_bytes<const L: usize>(hex: &str) -> [u8; L] {
    hex_to_bytes_vec(hex).try_into().unwrap()
}


pub fn hex_from_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X?}", b)).collect()
}
