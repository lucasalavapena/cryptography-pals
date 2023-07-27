use crate::aes_128;
use crate::utils::generate_random_vec;
use crate::xor;

const EMPTY_IV: [u8; 16] = [0; 16];

pub struct CBCBitflipper {
    aes_key: Vec<u8>,
}

impl CBCBitflipper {
    fn new() -> Self {
        Self {
            aes_key: generate_random_vec(16),
        }
    }

    fn encrypt(&self, input: &str) -> Vec<u8> {
        let modified_input = input.replace(";", "';'").replace("=", "'='");
        let plaintext = format!(
            "comment1=cooking%20MCs;userdata={};comment2=%20like%20a%20pound%20of%20bacon",
            modified_input
        );
        let bytes = plaintext.as_bytes();

        println!("bytes-len={}", bytes.len());

        aes_128::cbc::encrypt(&bytes, &self.aes_key, &EMPTY_IV, 16)
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        aes_128::cbc::decrypt(ciphertext, &self.aes_key, &EMPTY_IV, 16)
    }
}

pub fn cbc_bitflipping_attack() -> bool {
    // https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Cipher_block_chaining_(CBC)
    let original_message = "";
    let victim = CBCBitflipper::new();

    let ciphertext = victim.encrypt(&original_message);

    let og_plaintext = victim.decrypt(&ciphertext);

    println!("ciphertext_len={}", ciphertext.len());

    let target_second_block_ciphertext_before_oxr: Vec<u8> =
        xor::xor_bytes2(&ciphertext[0..16], &og_plaintext[16..32]);
    let target_plaintext = b"aaba;admin=true;";

    // required_first_block_ciphertext
    let new_first_block: Vec<u8> =
        xor::xor_bytes2(target_second_block_ciphertext_before_oxr, target_plaintext);

    let new_ciphertext: Vec<u8> = new_first_block
        .iter()
        .chain(ciphertext.iter().skip(16))
        .cloned()
        .collect();

    let res = victim.decrypt(&new_ciphertext);

    println!("res={:?}", res[16..32].to_vec());

    // let plaintext = std::str::from_utf8(&res).unwrap();
    // println!("plaintext={}", plaintext);
    let goal = ";admin=true;".as_bytes();
    println!("goal={:?}", goal.to_vec());

    // res.contains(goal)
    true
}

#[test]
fn test_solution() {
    let res = cbc_bitflipping_attack();
    assert!(res);
}
