use crate::aes_128;
use crate::block::pkcs7_padding_remove;
use base64::{
    engine::{self, general_purpose},
    Engine as _,
};
use itertools::Itertools;
const RANDOM_KEY: [u8; 16] = [
    199, 92, 104, 210, 54, 115, 162, 4, 165, 41, 162, 40, 240, 70, 205, 78,
];

pub fn padded_ecb(input: &[u8]) -> Vec<u8> {
    let pad = general_purpose::STANDARD.decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();

    let mut padded_bytes = input.to_owned();
    padded_bytes.extend(pad);

    aes_128::ecb::encrypt(&RANDOM_KEY, padded_bytes.as_slice(), None, true).unwrap()
}

pub fn ecb_oracle(ecb_encrypter: fn(&[u8]) -> Vec<u8>) -> Vec<u8> {
    let mut input = vec![];

    let og_size = ecb_encrypter(&input).len();
    let mut current_size = og_size;

    while current_size == og_size {
        input.push(b'A');
        current_size = ecb_encrypter(&input).len();
    }
    let block_size = current_size - og_size;

    // let mut : Option<Vec<u8>> = None;
    let mut res: Vec<u8> = vec![];
    let num_blocks = current_size / block_size;

    for k in 0..num_blocks {
        for i in 0..block_size {
            // unkown in block fill in with b'A' i.e. 65
            let no_unkown = block_size - i - 1;

            let reference_input: Vec<u8> = vec![b'A'; no_unkown];
            // note that the element we are looking for is always at the end of the block
            // whether it has the aritfical 65s in front of it other additional padding.
            let reference_block: Vec<u8> = ecb_encrypter(&reference_input)
                .iter()
                .skip(k * block_size)
                .take(block_size)
                .cloned()
                .collect();

            let mut relevant_block_input: Vec<u8> = vec![];

            // probably is a nicer way to do this
            if k > 0 {
                relevant_block_input = res[res.len() - block_size + 1..].to_vec();
            } else {
                relevant_block_input = reference_input
                    .iter()
                    .cloned()
                    .chain(res.iter().cloned())
                    .collect();
            }

            for j in 0..=255 {
                relevant_block_input.push(j);

                let cipher_text = ecb_encrypter(&relevant_block_input);

                let candidate_block: Vec<u8> =
                    cipher_text.iter().take(block_size).cloned().collect();
                if candidate_block == reference_block {
                    res.push(j);
                    break;
                }
                relevant_block_input.pop();
            }
        }
    }
    pkcs7_padding_remove(&res, block_size)
}

#[test]
fn challenge_12() {
    let res = ecb_oracle(padded_ecb);
    let expected = general_purpose::STANDARD.decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();

    assert_eq!(res, expected)
}
