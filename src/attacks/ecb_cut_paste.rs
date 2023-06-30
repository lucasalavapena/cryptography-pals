use crate::aes_128;
use crate::utils::generate_random_vec;

fn display_map(map: &Vec<(String, String)>) -> String {
    let mut result = String::new();
    result.push_str("{\n");
    for (key, value) in map {
        result.push_str(&format!("  {}: {},\n", key, value));
    }
    result.push_str("}");
    result
}

pub fn parse(input: &str) -> String {
    let mut map: Vec<(String, String)> = vec![];
    for pair in input.split('&') {
        let key_value: Vec<&str> = pair.split('=').collect();
        if key_value.len() == 2 {
            let key = key_value[0].to_owned();
            let value = key_value[1].to_owned();
            map.push((key, value));
        }
    }
    display_map(&map)
}

fn profile_for(email_address: &str) -> String {
    let filtered_email: String = email_address
        .chars()
        .filter(|c| !['&', '='].contains(c))
        .collect();
    format!("email={filtered_email}@bar.com&uid=10&role=user")
}

pub fn ecb_cute_paste_oracle() {
    let random_key = generate_random_vec(16);
    // email={'A' * (block_size-6)}
    // b"AAAAAAAAAA" + b"admin" + &[11;11];
    let input: Vec<u8> = b"AAAAAAAAAA"
        .iter()
        .chain(b"admin".iter())
        .chain([11u8; 11].iter())
        .cloned()
        .collect();

    let input_string = std::str::from_utf8(&input).unwrap();

    let admin_profile = profile_for(input_string);
    let encrypted_admin_block: Vec<u8> =
        aes_128::ecb::encrypt(random_key.as_slice(), admin_profile.as_bytes(), None, true)
            .unwrap()
            .iter()
            .skip(16)
            .take(16)
            .cloned()
            .collect();

    // char 5 is used as that allows the role assignment to be in its own block
    let my_profile = profile_for("lucas");
    let relevant_ciphertext: Vec<u8> =
        aes_128::ecb::encrypt(&random_key, my_profile.as_bytes(), None, true)
            .unwrap()
            .iter()
            .take(32)
            .cloned()
            .collect();

    let start_ciphertext = relevant_ciphertext.as_slice();
    let end_ciphertext = encrypted_admin_block.as_slice();

    let merged_ciphertext: Vec<u8> = [start_ciphertext, end_ciphertext].concat();

    let plaintext = aes_128::ecb::decrypt(&random_key, &merged_ciphertext, None, true).unwrap();
    let res = std::str::from_utf8(&plaintext).unwrap();
    println!("plaintext: {res}");
}

// Encrypt the encoded user profile under the key; "provide" that to the "attacker".
// Decrypt the encoded user profile and parse it.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cookie() {
        let cookie_str = "foo=bar&baz=qux&zap=zazzle";
        let cookie = parse(cookie_str);

        let expected = "{\n  foo: bar,\n  baz: qux,\n  zap: zazzle,\n}";
        assert_eq!(&cookie, expected);
    }

    #[test]
    fn cut_paste_oracle() {
        ecb_cute_paste_oracle()
    }
}
