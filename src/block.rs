// Tranpose blocks
// Now transpose the blocks: make a block that is the first byte of every block,
// and a block that is the second byte of every block, and so on.
pub fn tranpose_blocks(bytes: Vec<u8>, chunk_size: usize) -> Vec<Vec<u8>> {
    (0..chunk_size)
        .map(|i| {
            bytes
                .iter()
                .skip(i)
                .step_by(chunk_size)
                .cloned()
                .collect::<Vec<u8>>()
        })
        .collect()
}


use std::{error::Error, fmt};

#[derive(Debug)]
pub struct InvalidPKCS7Padding;

impl Error for InvalidPKCS7Padding {}

impl fmt::Display for InvalidPKCS7Padding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InvalidPKCS7Padding !!!")
    }
}

pub fn pkcs7_padding(block: &[u8], block_size: usize) -> Vec<u8> {
    let mut res = block.to_owned();

    let required_padding = 16 - block.len() % block_size;
    res.extend(vec![required_padding as u8; required_padding]);

    res
}

pub fn pkcs7_padding_remove(block: &[u8], block_size: usize) -> Result<Vec<u8>, InvalidPKCS7Padding> {
    let mut res = block.to_owned();
    let last_val = *block.last().unwrap();

    if last_val as usize > block_size {
        return Ok(res);
    }

    let mut cnt = 0;

    for i in 0..block_size {
        if *res.last().unwrap() == last_val {
            res.pop();
            cnt += 1;
        } else {
            if cnt == last_val {
                return Ok(res);
            }
            else {
                return Err(InvalidPKCS7Padding);
            }
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_PKCS7_padding() {
        let bytes: Vec<u8> = (0..12).collect();
        let res = pkcs7_padding(&bytes, 20);

        let mut expected: Vec<u8> = (0..12).collect();
        expected.extend(vec![4, 4, 4, 4]);
        assert_eq!(res.clone(), expected)
    }

    #[test]
    fn test_PKCS7_padding_remove() {
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 2, 2];
        let res = pkcs7_padding_remove(&bytes, 3);

        assert!(res.is_ok());

        let expected: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(res.unwrap(), expected)
    }

    #[test]
    fn test_PKCS7_padding_remove2() {
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 4, 4];
        let res = pkcs7_padding_remove(&bytes, 3);

        assert!(res.is_err());

    }

    #[test]
    fn test_PKCS7_padding_remove3() {
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 19, 23];
        let res = pkcs7_padding_remove(&bytes, 3);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), bytes)

    }

    #[test]
    fn test_tranpose_blocks() {
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let res = tranpose_blocks(bytes, 3);

        let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(res, expected)
    }
}
