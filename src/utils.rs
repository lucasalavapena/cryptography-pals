// pub fn file_to_bytes(file_path: String, decoder: F) -> Vec<Vec<u8>>
// where
//     F: Fn(&str) -> Result<Vec<u8>, std::num::ParseIntError>,
// {
//     let input: Vec<&str> = include_str!(file_path).split('\n').collect();
//     input
//         .iter()
//         .map(|line| hex::decode(line).unwrap())
//         .collect()
// }
