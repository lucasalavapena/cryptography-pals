
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
const RANDOM_KEY: [u8; 16] = [199, 92, 104, 210, 54, 115, 162, 4, 165, 41, 162, 40, 240, 70, 205, 78];

pub fn ecb_encryption_oracle(input: &Vec<u8>) -> Vec<u8> {
    let pad = general_purpose::STANDARD.decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();

    let mut padded_bytes = input.clone();
    padded_bytes.extend(pad);

    aes_128::ecb::encrypt(
        &RANDOM_KEY,
        padded_bytes.as_slice(),
        None,
        true,
        ).unwrap()
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
    // 6

    while current_size == og_size {
        input.push(65);
        current_size = ecb_encryption_oracle(&input).len();
    }
    let block_size = current_size - og_size;
    input = vec![65; block_size-1];

    // let mut : Option<Vec<u8>> = None;


    for i in 0..og_size{
        let mut ref_input = vec![];

        if i >= block_size {
            ref_input.extend(input.clone());
        }
        if i > 0 && i % block_size == 0 {
            input.extend(vec![65; block_size-1])
        }
        let num_blocks = (i / block_size);
        let current_idx = (block_size - i % block_size) - 1;
        let actual_idx = block_size * num_blocks  + current_idx;

        // let mut hashmap: HashMap<u8, Vec<u8>> = HashMap::new();
        ref_input.extend(vec![65; current_idx]);

        let reference_block: Vec<u8> = ecb_encryption_oracle(&ref_input).iter().skip(num_blocks * block_size).take(block_size).cloned().collect();

        // println!("{i} - ref_input={ref_input:?} len={}", ref_input.len());
        println!("{i} - reference={reference_block:?} len={}", reference_block.len());
        println!("{i} input = {:?} len={}", input, input.len());

        for j in 0..=255 {
            input.push(j);
            let res = ecb_encryption_oracle(&input);
            let small: Vec<u8> = res.iter().skip(num_blocks * block_size).take(block_size).cloned().collect();
            // println!("{:?} {:?} {:?}", input, small, res.len());
            if reference_block == small {
                println!("{i} - found reference={j} {reference_block:?} {small:?}");
                if current_idx > 0 {
                    input.remove(actual_idx - 1);
                    println!("{input:?} {} {}", actual_idx - 1, input.len())
                }
                break;
            }
            // hashmap.insert(j, small);
            input.pop();
        }
        // println!("input = {:?}", input);
        if (i == 20) {
            break;

        }
    }
    println!("input = {:?}", input);

    let decoded_message = std::str::from_utf8(&input).unwrap();

    println!("decoded_message={:?}", decoded_message);

}