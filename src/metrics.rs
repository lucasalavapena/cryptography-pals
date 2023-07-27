use itertools::Itertools;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::iter::zip;

use crate::xor;

//todo wasserstein distance or kl divergence?

// from wikipedia
const FREQUENCY_CHAR: [u32; 26] = [
    8200,  // a
    3500,  // b
    2800,  // c
    4300,  // d
    13000, // e
    2200,  // f
    2000,  // g
    6100,  // h
    7000,  // i
    150,   // j
    770,   //k
    4000,  // l
    2400,  // m
    6700,  // n
    7500,  // o
    1900,  // p
    95,    //q
    6000,  //r
    6300,  //s
    9100,  //t
    2800,  //u
    980,   // v
    2400,  // w
    150,   // x
    2000,  // y
    740,   // z
];

fn score_based_char_freq(bytes: Vec<u8>) -> u32 {
    let mut score: u32 = 0;
    for byte in bytes {
        let c: char = byte.try_into().unwrap();
        let c_lower = c.to_lowercase().to_string().chars().next().unwrap();

        if !c_lower.is_ascii_lowercase() {
            // 32 is the space character

            if c_lower == ' ' {
                score += 26000;
            }
            continue;
        }

        let b: u32 = c_lower.into();
        score += FREQUENCY_CHAR[b as usize - 97];
    }
    score
}

pub fn get_best_key(bytes: &[u8]) -> (u32, u8) {
    (0..=255)
        .map(|c| {
            (
                score_based_char_freq(xor::xor_bytes_single(bytes.to_owned(), c)),
                c,
            )
        })
        .max_by_key(|(score, _)| *score)
        .unwrap()
}

pub fn count_repeated_blocks(bytes: &[u8], block_size: usize) -> usize {
    let mut unique_items: HashSet<Vec<u8>> = HashSet::new();

    let blocks = bytes.chunks(block_size);
    let total_blocks = blocks.len();

    for b in blocks {
        unique_items.insert(b.to_vec());
    }
    // println!("og: {:?} uq: {:?}", bytes.chunks(block_size), unique_items);
    total_blocks - unique_items.len()
}

// pub fn repeated_blocks_idxs(bytes: &[u8], block_size: usize) -> Vec<usize> {
//     // bytes.windows(block_size)
//     // .enumerate()
//     // .group_by(|(_, chunk)| *chunk)
//     // .into_iter()
//     // .filter_map(|(thing, group)| {
//     //     let cnt = group.clone().count();
//     //     println!("thing={thing:?}");
//     //     let res = group.into_iter().map(|(i, _)| i).collect::<Vec<usize>>();

//     //     if res.len() > 1 {
//     //         Some(res)
//     //     } else {
//     //         None
//     //     }
//     //     // None
//     // })
//     // .flatten()
//     // .collect()
//     let mut unique_items: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();

//     let blocks = bytes.chunks(block_size);
//     let total_blocks = blocks.len();

//     for (i, b) in blocks.enumerate() {
//         let value = b.to_vec();
//         if !unique_items.contains(value) {
//             unique_items.insert((value, i));
//         }
//         else {

//             repeated_idx
//         }
//     }
// }

pub fn find_repeated_chunk_indexes(data: &[u8], chunk_len: usize) -> Vec<usize> {
    let mut chunk_map: HashMap<&[u8], HashSet<usize>> = HashMap::new();

    for (i, chunk) in data.chunks(chunk_len).enumerate() {
        let entry = chunk_map.entry(chunk).or_insert_with(HashSet::new);
        entry.insert(i);
    }

    let repeated_chunks: Vec<&HashSet<usize>> = chunk_map
        .values()
        .filter(|indexes| indexes.len() > 1)
        .collect();

    let mut indexes: Vec<usize> = repeated_chunks
        .iter()
        .flat_map(|indexes| indexes.iter())
        .cloned()
        .collect();

    indexes.sort();

    indexes
}

// pub fn count_repeated_bytes(bytes: &[u8]) -> usize {
//     let mut unique_items: HashSet<u8> = HashSet::new();

//     for b in bytes {
//         unique_items.insert(*b);
//     }
//     bytes.len() - unique_items.len()
// }

pub fn hamming_distance<I, J>(a: I, b: J) -> usize
where
    I: IntoIterator<Item = u8>,
    J: IntoIterator<Item = u8>,
{
    a.into_iter()
        .zip(b.into_iter())
        .map(|(x, y)| (x ^ y).count_ones() as usize)
        .sum()
}

#[test]
fn test_hamming_distance() {
    let a: Vec<u8> = "this is a test".as_bytes().to_vec();
    let b: Vec<u8> = "wokka wokka!!!".as_bytes().to_vec();

    let result = hamming_distance(a, b);

    let expected = 37;
    assert_eq!(result, expected);
}

#[test]
fn test_hamming_distance2() {
    let a: Vec<u8> = "this is a testpssadsa".as_bytes().to_vec();
    let b: Vec<u8> = "wokka wokka!!!".as_bytes().to_vec();

    let result = hamming_distance(a, b);

    let expected = 37;
    assert_eq!(result, expected);
}

#[test]
fn test_score_based_char_freq() {
    let bytes: Vec<u8> = vec![32, 32];
    let result = score_based_char_freq(bytes);
    let expected = 52000;
    assert_eq!(result, expected);
}

#[test]
fn test_repeated_blocks_idxs() {
    let bytes: Vec<u8> = vec![32, 32, 15, 15, 19, 15, 32, 32, 15, 32, 32, 15];
    let result = find_repeated_chunk_indexes(&bytes, 3);
    println!("result={result:?}");
    // let expected = vec![];
    // assert_eq!(result, expected);
}

// repeated_blocks_idxs(bytes: &[u8], block_size: usize)
