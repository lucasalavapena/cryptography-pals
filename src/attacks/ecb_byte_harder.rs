use crate::block::pkcs7_padding_remove;
use crate::metrics::find_repeated_chunk_indexes;
use crate::{aes_128, block};

use base64::{engine::general_purpose, Engine as _};
use itertools::Itertools;

const RANDOM_KEY: [u8; 16] = [
    199, 92, 104, 210, 54, 115, 162, 4, 165, 41, 162, 40, 240, 70, 205, 78,
];
use crate::utils::{generate_rand_vec_rand_size, generate_random_vec};

// AES-128-ECB(random-prefix || attacker-controlled || target-bytes, random-key)

pub fn ecb_input(attacker_input: &[u8], random_prefix: &[u8]) -> Vec<u8> {
    // let random_prefix = generate_rand_vec_rand_size(5, 50);
    let target_bytes = general_purpose::STANDARD.decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();

    let mut entire_input: Vec<u8> = random_prefix.to_vec();
    entire_input.extend(attacker_input);
    entire_input.extend(target_bytes);

    aes_128::ecb::encrypt(&RANDOM_KEY, entire_input.as_slice(), None, true).unwrap()
}

pub fn ecb_oracle(ecb_encrypter: fn(&[u8], &[u8]) -> Vec<u8>) -> Vec<u8> {
    let random_prefix = generate_rand_vec_rand_size(5, 50);

    let mut input = vec![];

    let og_size = ecb_encrypter(&input, &random_prefix).len();
    let mut current_size = og_size;

    while current_size == og_size {
        input.push(b'A');
        current_size = ecb_encrypter(&input, &random_prefix).len();
    }

    let block_size = current_size - og_size;
    // let prefix_size = og_size + (block_size - input.len());

    // get prefix_size
    // 2 * might be enough but this should also work
    input.extend(vec![b'A'; 4 * block_size]);

    let prefix_size_experiment_og = ecb_encrypter(&input, &random_prefix);
    let no_repeated_block_og = find_repeated_chunk_indexes(&prefix_size_experiment_og, block_size);
    let mut repeated_blocks = no_repeated_block_og.len();

    while repeated_blocks == no_repeated_block_og.len() {
        let prefix_size_experiment = ecb_encrypter(&input, &random_prefix);
        repeated_blocks = find_repeated_chunk_indexes(&prefix_size_experiment, block_size).len();
        input.pop();
    }
    // println!("no_repeated_block_og={no_repeated_block_og:?}");

    let prefix_size = block_size * (no_repeated_block_og.last().unwrap() + 1) - input.len() - 2;
    // println!("prediced prfix_size={prefix_size}");

    // ceiling
    let num_blocks_offset = (prefix_size + (block_size - 1)) / block_size;
    // get repeated blocks and idx
    let prefix_remaining = block_size * num_blocks_offset - prefix_size;

    // println!("block_size: {block_size}");
    // let mut : Option<Vec<u8>> = None;
    let mut res: Vec<u8> = vec![];
    let num_blocks = current_size / block_size;

    for k in 0..num_blocks {
        for i in 0..block_size {
            // unkown in block fill in with b'A' i.e. 65
            let no_unkown = block_size - i - 1;

            let reference_input: Vec<u8> = vec![b'A'; no_unkown + prefix_remaining];
            // note that the element we are looking for is always at the end of the block
            // whether it has the aritfical 65s in front of it other additional padding.
            let reference_block: Vec<u8> = ecb_encrypter(&reference_input, &random_prefix)
                .iter()
                .skip((k + num_blocks_offset) * block_size)
                .take(block_size)
                .cloned()
                .collect();

            let mut relevant_block_input: Vec<u8> = vec![];
            // let mut skip_amount = 0;
            // probably is a nicer way to do this
            if k > 0 {
                // deal with prefix padding
                relevant_block_input.extend(vec![b'A'; prefix_remaining]);
                relevant_block_input.extend(res[res.len() - block_size + 1..].to_vec());
            } else {
                relevant_block_input = reference_input
                    .iter()
                    .cloned()
                    .chain(res.iter().cloned())
                    .collect();
            }

            for j in 0..=255 {
                relevant_block_input.push(j);

                let cipher_text = ecb_encrypter(&relevant_block_input, &random_prefix);

                let candidate_block: Vec<u8> = cipher_text
                    .iter()
                    .skip(num_blocks_offset * block_size)
                    .take(block_size)
                    .cloned()
                    .collect();
                if candidate_block == reference_block {
                    res.push(j);
                    break;
                }
                relevant_block_input.pop();
            }
        }
    }
    pkcs7_padding_remove(&res, block_size).unwrap()
}

#[test]
fn challenge_14() {
    let res = ecb_oracle(ecb_input);
    // println!("res={res:?}");
    // let decoded_message = std::str::from_utf8(&res).unwrap();
    // println!("decoded_message: {}", decoded_message);

    let expected = general_purpose::STANDARD.decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();

    assert_eq!(res, expected)
}
