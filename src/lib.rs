mod aes_128;
mod bases;
mod block;
mod metrics;
mod utils;
mod xor;
mod attacks;

use itertools::Itertools;
use std::iter::zip;

#[cfg(test)]
mod tests {

    use crate::aes_128;
    use crate::bases;
    use crate::block;
    use crate::metrics;
    use crate::utils;
    use crate::xor;
    use crate::attacks;

    use base64::{
        engine::{self, general_purpose},
        Engine as _,
    };
    use itertools::Itertools;
    use openssl::symm::Mode;
    #[test]
    fn challenge_01() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let pairs = input.chars().chunks(2);
        let bytes = pairs.into_iter().map(|pair| {
            let (x, y) = pair.collect_tuple().unwrap();
            (bases::hex_to_binary(x) << 4) + bases::hex_to_binary(y)
        });

        let base64 = bytes
            .chunks(3)
            .into_iter()
            .flat_map(|triplet| {
                bases::u24_to_base64(triplet.collect::<Vec<_>>().try_into().unwrap())
            })
            .collect::<String>();

        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(base64, expected);
    }

    #[test]
    fn challenge_02() {
        let input = "1c0111001f010100061a024b53535009181c";
        let XORer = "686974207468652062756c6c277320657965";
        let decoded_input = hex::decode(input).unwrap();
        let decoded_XORer = hex::decode(XORer).unwrap();
        let xored_vector = xor::xor_bytes(decoded_input, decoded_XORer);

        let result = hex::encode(xored_vector);

        let expected = "746865206b696420646f6e277420706c6179";
        assert_eq!(result, expected);
    }

    #[test]
    fn challenge_03() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let decoded_input = hex::decode(input).unwrap();
        // Note to inverse an xor, you xor it again :))

        let (score, key) = metrics::get_best_key(&decoded_input);

        let binding = xor::xor_bytes_single(decoded_input.clone(), key);
        let decoded_message = std::str::from_utf8(&binding).unwrap();
        let expected = "Cooking MC's like a pound of bacon";
        assert_eq!(decoded_message, expected);
    }

    #[test]
    fn challenge_04() {
        let input: Vec<&str> = include_str!("data/4.txt").split('\n').collect();

        let decoded_input: Vec<Vec<u8>> = input
            .iter()
            .map(|line| hex::decode(line).unwrap())
            .collect();

        let ((score, key), line) = decoded_input
            .iter()
            .map(|line| (metrics::get_best_key(&line), line))
            .max_by_key(|(score, _)| score.clone())
            .unwrap();

        let message = xor::xor_bytes_single(line.clone(), key);
        let decoded_message = std::str::from_utf8(&message).unwrap();

        assert_eq!(
            decoded_message.replace("\n", ""),
            "Now that the party is jumping"
        );
    }

    #[test]
    fn challenge_05() {
        let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let input_vec = input.to_vec();
        let key = "ICE".as_bytes().to_vec();

        let result_vec = xor::repeating_key_xor(input_vec, key);
        let result = hex::encode(result_vec);

        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(result, expected);
    }

    #[test]
    fn challenge_06() {
        let input = include_str!("data/6.txt").replace("\n", "");

        let bytes = general_purpose::STANDARD.decode(input).unwrap();
        let (key_size, normalised_score) = (2..=40)
            .map(|key_size| {
                (
                    key_size,
                    metrics::hamming_distance(
                        bytes[0..(32 * key_size)].to_vec(),
                        bytes[(32 * key_size)..(2 * 32 * key_size)].to_vec(),
                    ) / (key_size),
                )
            })
            .min_by_key(|(key_size, normalised_score)| normalised_score.clone())
            .unwrap();

        let mut key: Vec<u8> = vec![];
        let tranposed_blocks = block::tranpose_blocks(bytes, key_size);

        for block in tranposed_blocks {
            let (score, block_key) = metrics::get_best_key(&block);
            key.push(block_key);
        }

        let decoded_key = std::str::from_utf8(&key).unwrap();
        assert_eq!(decoded_key, "Terminator X: Bring the noise")
    }

    #[test]
    fn challenge_07() {
        let input = include_str!("data/7.txt").replace("\n", "");
        let bytes = general_purpose::STANDARD.decode(input).unwrap();
        let key = b"YELLOW SUBMARINE";

        let res = aes_128::ecb::decrypt(key, &bytes, None, true).unwrap();

        let message = std::str::from_utf8(&res).unwrap();
        println!("{}", message);
    }

    #[test]
    fn challenge_08() {
        const BLOCK_SIZE: usize = 16;
        let input: Vec<&str> = include_str!("data/8.txt").split('\n').collect();

        let decoded_input: Vec<Vec<u8>> = input
            .iter()
            .map(|line| hex::decode(line).unwrap())
            .collect();

        let (idx, score) = decoded_input
            .iter()
            .enumerate()
            .map(|(i, line)| (i, metrics::count_repeated_blocks(line, BLOCK_SIZE)))
            .max_by_key(|(_, score)| score.clone())
            .unwrap();

        println!("{} {}", idx, score);

    }

    #[test]
    fn challenge_09() {
        let mut bytes: Vec<u8> = (0..16).collect();
        let res = block::PKCS7_padding(&mut bytes, 20);

        let mut expected: Vec<u8> = (0..16).collect();
        expected.extend(vec![4, 4, 4, 4]);
        assert_eq!(res.clone(), expected)
    }
    fn challenge_10() {
        todo!()
    }

    #[test]
    fn challenge_11() {
        const BLOCK_SIZE: usize = 16;
        let bytes = include_str!("data/11.txt").replace("\n", "").as_bytes().to_vec();

        let (output, mode) = attacks::aes_mode::encryption_oracle(bytes, BLOCK_SIZE);

        let count = metrics::count_repeated_blocks(&output, BLOCK_SIZE);

        if count > 2 {
            assert_eq!(mode, attacks::aes_mode::AESModes::ECB)
        } else {
            assert_eq!(mode, attacks::aes_mode::AESModes::CBC)
        }

    }
}
