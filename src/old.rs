use std::iter::zip;

fn XOR(x: Vec<u8>, y: Vec<u8>) -> Vec<u8> {
    let decoded_input = hex::decode(x).unwrap();
    let decoded_XORer = hex::decode(y).unwrap();

    let zipped = zip(decoded_input, decoded_XORer);
    zipped.map(|(a, b)| a ^ b).collect::<Vec<u8>>()
}



// fn challenge_03() {
//     let u8_to_base64 = base64_hashmap();

//     let mut frequency: HashMap<char, u32> = HashMap::new();
//     for c in input.chars() { // word is a &str
//         *frequency.entry(c).or_insert(0) += 1; // word does not live long enough
//     }
//     println!("{:?}", frequency);

//     let decoded_input = hex::decode(input).unwrap();
//     println!("decoded_input: {:?}", decoded_input);

//     let no_bytes = decoded_input.len();
//     let mut max_score: f32 = f32::MIN;
//     let mut res: Vec<u8> = decoded_input.clone();

//     use std::collections::BinaryHeap;

//     // let mut heap: BinaryHeap<(f32, &str)> = BinaryHeap::<(f32, &str)>::new();

//     for c in 0..=255 {
//         let xored_vector = xor::xor_bytes_single(decoded_input.clone(), c);
//         let score = score_based_char_freq(xored_vector.clone());

//         if score > max_score {
//             max_score = score;
//             res = xored_vector.clone();
//         }

//         let s = match std::str::from_utf8(&xored_vector) {
//             Ok(v) => v,
//             Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//         };

//         // heap.push((score, s))
//         // println!("result: {}", s);
//         // println!("score: {}", score);
// let debuging: Vec<(u32, String)> = (0..=255)
// .map(|c| xor::xor_bytes_single(decoded_input.clone(), c))
// .map(|xor_vec| {
//     let xor_vec_clone = xor_vec.clone();
//     (metrics::score_based_char_freq(xor_vec_clone.clone()), String::from_utf8(xor_vec_clone).unwrap())
// })
// .collect();
// println!("{:?}", debuging);        
//     }

    // #[test]
    // fn challenge_03() {
    //     let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    //     let decoded_input = hex::decode(input).unwrap();
    //     // Note to inverse an xor, you xor it again :))
    //     let (result, score)(0..=255).map(|c| metrics::score_based_char_freq(xor::xor_bytes_single(decoded_input.clone(), c)))
    //     let expected = "746865206b696420646f6e277420706c6179";
    //     assert_eq!(result, expected);
    // }