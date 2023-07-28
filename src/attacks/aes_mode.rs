use crate::aes_128;
use rand::{distributions::Uniform, Rng};

use crate::utils::generate_random_vec;

#[derive(Debug, PartialEq)]
pub enum AESModes {
    ECB,
    CBC,
}

pub fn encryption_oracle(bytes: Vec<u8>, key_size: usize) -> (Vec<u8>, AESModes) {
    let aes_key = generate_random_vec(key_size);
    let iv = generate_random_vec(key_size);

    let mut padded_bytes = vec![];
    let mut rng = rand::thread_rng();
    let padding_dist: Uniform<usize> = Uniform::new(5, 11);

    let before_padding = rng.sample(padding_dist);
    let after_padding = rng.sample(padding_dist);

    padded_bytes.extend(generate_random_vec(before_padding));
    padded_bytes.extend(bytes);
    padded_bytes.extend(generate_random_vec(after_padding));

    let prob: f64 = rng.gen();
    // mode decision
    if prob > 0.5 {
        (
            aes_128::ecb::encrypt(
                aes_key.as_slice(),
                padded_bytes.as_slice(),
                Some(iv.as_slice()),
                true,
            )
            .unwrap(),
            AESModes::ECB,
        )
    } else {
        (
            aes_128::cbc::encrypt(
                padded_bytes.as_slice(),
                aes_key.as_slice(),
                iv.as_slice(),
                key_size,
            ),
            AESModes::CBC,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_vec() {
        let key_size = 43;
        let res = generate_random_vec(key_size);

        assert_eq!(res.len(), key_size);

        let massive = 1000;
        let res_massive = generate_random_vec(massive);
        let total = res_massive.iter().map(|x| *x as i32).sum();
        assert!(100_000 < total && total < 200_000);
    }
}
