// AES-128-ECB(your-string || unknown-string, random-key)

// Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
// aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
// dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
// YnkK

// Spoiler alert.

// Do not decode this string now. Don't do it.

// Base64 decode the string before appending it. Do not base64 decode the string by hand; make your code do it. The point is that you don't know its contents.

// What you have now is a function that produces:

// AES-128-ECB(your-string || unknown-string, random-key)

// It turns out: you can decrypt "unknown-string" with repeated calls to the oracle function!

// Here's roughly how:

//     Feed identical bytes of your-string to the function 1 at a time --- start with 1 byte ("A"), then "AA", then "AAA" and so on. Discover the block size of the cipher. You know it, but do this step anyway.
//     Detect that the function is using ECB. You already know, but do this step anyways.
//     Knowing the block size, craft an input block that is exactly 1 byte short (for instance, if the block size is 8 bytes, make "AAAAAAA"). Think about what the oracle function is going to put in that last byte position.
//     Make a dictionary of every possible last byte by feeding different strings to the oracle; for instance, "AAAAAAAA", "AAAAAAAB", "AAAAAAAC", remembering the first block of each invocation.
//     Match the output of the one-byte-short input to one of the entries in your dictionary. You've now discovered the first byte of unknown-string.
//     Repeat for the next byte.
use std::collections::HashMap;

use crate::aes_128;

use base64::{
    engine::{self, general_purpose},
    Engine as _,
};
use itertools::Itertools;
const RANDOM_KEY: [u8; 16] = [
    199, 92, 104, 210, 54, 115, 162, 4, 165, 41, 162, 40, 240, 70, 205, 78,
];

pub fn ecb_encryption_oracle(input: &Vec<u8>) -> Vec<u8> {
    let pad = general_purpose::STANDARD.decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();

    let mut padded_bytes = input.clone();
    padded_bytes.extend(pad);

    aes_128::ecb::encrypt(&RANDOM_KEY, padded_bytes.as_slice(), None, true).unwrap()
}

// 1. Feed identical bytes of your-string to the function 1 at a time --- start with 1 byte ("A"), then "AA", then "AAA" and so on. Discover the block size of the cipher. You know it, but do this step anyway.
// 2. Detect that the function is using ECB. You already know, but do this step anyways.
// 3. Knowing the block size, craft an input block that is exactly 1 byte short (for instance, if the block size is 8 bytes, make "AAAAAAA"). Think about what the oracle function is going to put in that last byte position.
// 4. Make a dictionary of every possible last byte by feeding different strings to the oracle; for instance, "AAAAAAAA", "AAAAAAAB", "AAAAAAAC", remembering the first block of each invocation.
// 5. Match the output of the one-byte-short input to one of the entries in your dictionary. You've now discovered the first byte of unknown-string.
// Repeat for the next byte.

#[test]
pub fn decrypt_ecb() {
    let mut input = vec![];

    let og_size = ecb_encryption_oracle(&input).len();
    let mut current_size = og_size;

    while current_size == og_size {
        input.push(b'A');
        current_size = ecb_encryption_oracle(&input).len();
    }
    let block_size = current_size - og_size;

    // let mut : Option<Vec<u8>> = None;
    let mut res: Vec<u8> = vec![];
    let num_blocks = current_size / block_size;

    for k in 0..num_blocks {
        let mut unkown_block: Vec<u8> = vec![];
        for i in 0..block_size {
            // unkown in block fill in with b'A' i.e. 65
            let no_unkown = block_size - i - 1;

            let mut input: Vec<u8> = res
                .iter()
                .cloned()
                .chain(vec![b'A'; no_unkown].iter().cloned())
                .collect();


            let reference_block: Vec<u8> = ecb_encryption_oracle(&input)
                .iter()
                .skip(k * block_size)
                .take(block_size)
                .cloned()
                .collect();

            println!(
                "{i} - reference={reference_block:?} len={}",
                reference_block.len()
            );
            println!("{i} input = {:?} len={}", input, input.len());

            // use known plaintext to check with reference
            input.extend(&unkown_block);

            for j in 0..=255 {
                input.push(j);

                let cipher_text = ecb_encryption_oracle(&input);

                let candidate_block: Vec<u8> = cipher_text
                    .iter()
                    .skip(k * block_size)
                    .take(block_size)
                    .cloned()
                    .collect();
                if candidate_block == reference_block {
                    // println!("{:?} {:?} {:?}", input, small, res.len());
                    println!(
                        "{k} {i} - found reference={j} {reference_block:?} {candidate_block:?}"
                    );
                    unkown_block.push(j);
                    break;
                }
                input.pop();
            }
        }
        res.extend(unkown_block);
        if (k == 2) {
            break;
        }
    }

    let decoded_message = std::str::from_utf8(&res).unwrap();
    println!("decoded_message={:?}", decoded_message);
}
