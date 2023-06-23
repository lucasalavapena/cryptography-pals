use std::collections::HashMap;

pub fn base64_hashmap() -> HashMap<u8, char> {
    let mut u8_to_base64 = HashMap::new();

    for (k, v) in ('A'..='Z').enumerate() {
        u8_to_base64.insert(k as u8, v);
    }

    for (k, v) in ('a'..='z').enumerate() {
        u8_to_base64.insert(k as u8 + 26, v);
    }

    for (k, v) in ('0'..='9').enumerate() {
        u8_to_base64.insert(k as u8 + 52, v);
    }
    u8_to_base64.insert(62, '+');
    u8_to_base64.insert(63, '/');

    u8_to_base64
}

pub fn hex_to_binary(c: char) -> u8 {
    if c <= '9' {
        c as u8 - 48
    } else {
        c as u8 - 97 + 10
    }
}

pub fn u24_to_base64(input: [u8; 3]) -> [char; 4] {
    // input : aaaa_aabb bbbb_cccc ccdd_dddd
    // output : 00aaaaaa 00bbbbbb 00cccccc 00dddddd

    let u8_to_base64 = base64_hashmap();
    let a = (input[0] & 0b111_111_00) >> 2;
    let b = (input[0] & 0b000_000_11) << 4 | (input[1] & 0b1111_0000) >> 4;
    let c = (input[1] & 0b0000_1111) << 2 | (input[2] & 0b1100_0000) >> 6;
    let d = input[2] & 0b00_11_1111;

    let bytes = [a, b, c, d];
    bytes.map(|byte| u8_to_base64[&byte])
}
