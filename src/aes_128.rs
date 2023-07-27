use std::vec;

// Mode::Decrypt

pub mod ecb {
    use openssl::error::ErrorStack;
    use openssl::symm::{Cipher, Crypter, Mode};

    fn ecb_helper(
        key: &[u8],
        ciphertext: &[u8],
        mode: Mode,
        iv: Option<&[u8]>,
        pad: bool,
    ) -> Result<Vec<u8>, ErrorStack> {
        let cipher = Cipher::aes_128_ecb();
        let mut crypter = Crypter::new(cipher, mode, key, iv)?;
        crypter.pad(pad);
        let mut res = vec![0; ciphertext.len() + cipher.block_size()];
        let count = crypter.update(ciphertext, &mut res)?;
        let final_count = crypter.finalize(&mut res[count..])?;
        res.truncate(count + final_count);

        Ok(res)
    }
    pub fn encrypt(
        key: &[u8],
        plaintext: &[u8],
        iv: Option<&[u8]>,
        pad: bool,
    ) -> Result<Vec<u8>, ErrorStack> {
        ecb_helper(key, plaintext, Mode::Encrypt, iv, pad)
    }
    pub fn decrypt(
        key: &[u8],
        ciphertext: &[u8],
        iv: Option<&[u8]>,
        pad: bool,
    ) -> Result<Vec<u8>, ErrorStack> {
        ecb_helper(key, ciphertext, Mode::Decrypt, iv, pad)
    }
}

pub mod cbc {
    use super::*;
    use crate::block::{pkcs7_padding, pkcs7_padding_remove};
    use crate::xor;

    pub fn encrypt(bytes: &[u8], key: &[u8], iv: &[u8], block_size: usize) -> Vec<u8> {
        let mut encrypted_bytes = vec![];
        let padded_bytes = pkcs7_padding(bytes, block_size);

        let mut prev_block = iv.to_vec();

        for curr_block in padded_bytes.chunks(block_size) {
            let xored_block: Vec<u8> = xor::xor_bytes2(curr_block, &prev_block);

            let encrypted_block =
                super::ecb::encrypt(key, xored_block.as_slice(), None, false).unwrap();
            encrypted_bytes.extend(encrypted_block.clone());

            prev_block = encrypted_block;
        }

        encrypted_bytes
    }

    pub fn decrypt(bytes: &[u8], key: &[u8], iv: &[u8], block_size: usize) -> Vec<u8> {
        let mut decrypted_bytes = vec![];
        let mut prev_block = iv.to_vec();

        for curr_block in bytes.chunks(block_size) {
            let decrypted_block = super::ecb::decrypt(key, curr_block, None, false).unwrap();
            let xored_block: Vec<u8> = xor::xor_bytes2(decrypted_block, &prev_block);
            decrypted_bytes.extend(xored_block.clone());
            prev_block = curr_block.to_vec();
        }

        println!("before removing trim {:?} ", decrypted_bytes);
        pkcs7_padding_remove(&decrypted_bytes.to_vec(), block_size).unwrap()
    }
}

pub mod ctr {
    use openssl::error::ErrorStack;

    use super::*;
    use crate::xor;

    fn helper(plaintext: &[u8], key: &[u8], nonce: u64) -> Result<Vec<u8>, ErrorStack>  {
        let mut res: Vec<u8> = vec![];
        let nonce_array: [u8; 8] = nonce.to_le_bytes();

        let mut counter = 0u64;
        let mut counter_array: [u8; 8] = counter.to_le_bytes();

        let mut keystream: [u8; 16] = [0; 16];

        for (i, e) in nonce_array.iter().enumerate() {
            keystream[i] = *e;
        }

        for plaintext_section in plaintext.chunks(16) {

            // if plaintext_section.len() != 16 {break;}
            let keystream_salt = super::ecb::encrypt(key, keystream.as_slice(), None, false)?;
            let chipertext_block: Vec<u8> = xor::xor_bytes2(plaintext_section, keystream_salt);
            res.extend(chipertext_block);

            // update count
            counter += 1;
            counter_array = counter.to_le_bytes();

            for (i, e) in counter_array.iter().enumerate() {
                keystream[i + 8] = *e;
            }
            println!("keystream={keystream:?}");

        }

        Ok(res)
    }

    pub fn encrypt(plaintext: &[u8], key: &[u8], nonce: Option<u64>) -> Result<Vec<u8>, ErrorStack> {
        helper(plaintext, key, nonce.unwrap_or(0u64))
    }
    pub fn decrypt(
        ciphertext: &[u8],
        key: &[u8],
        nonce: Option<u64>,
    ) -> Result<Vec<u8>, ErrorStack> {
        helper(ciphertext, key, nonce.unwrap_or(0u64))
    }
}

#[cfg(test)]
mod tests {

    use base64::{
        engine::{self, general_purpose},
        Engine as _,
    };
    use super::*;

    // fn aes_128_ecb_test() {
    //     let aes_key = b"YELLOW SUBMARINE";
    //     let padded_bytes = ;

    //     let res = aes_128_ecb(
    //         aes_key.as_slice(),
    //         padded_bytes.as_slice(),
    //         Mode::Encrypt,
    //         Some(iv.as_slice()),
    //     )
    //     .unwrap()
    // }

    #[test]
    fn aes_128_cbc_14_bytes() {
        let bytes = b"19 is the best";
        let key = b"YELLOW SUBMARINE";
        let iv = b"0000000000000000";

        let encrypted_bytes = cbc::encrypt(bytes, key, iv, 16);
        let decrypted_bytes = cbc::decrypt(encrypted_bytes.as_slice(), key, iv, 16);

        assert_eq!(decrypted_bytes, bytes.as_slice());
    }

    #[test]
    fn aes_128_cbc_16_bytes() {
        let bytes = b"2219 is the best";
        let key = b"YELLOW SUBMARINE";
        let iv = b"0000000000000000";

        let encrypted_bytes = cbc::encrypt(bytes, key, iv, 16);
        let decrypted_bytes = cbc::decrypt(encrypted_bytes.as_slice(), key, iv, 16);

        assert_eq!(decrypted_bytes, bytes.as_slice());
    }

    #[test]
    fn aes_128_cbc_37_bytes() {
        let bytes = b"1<<255-19 is the biggest prime I know";
        let key = b"YELLOW SUBMARINE";
        let iv = b"0000000000000000";

        let encrypted_bytes = cbc::encrypt(bytes, key, iv, 16);
        let decrypted_bytes = cbc::decrypt(encrypted_bytes.as_slice(), key, iv, 16);

        assert_eq!(decrypted_bytes, bytes.as_slice());
    }

    #[test]
    fn aes_128_ctr_example() {
        let encrypted_bytes =
        general_purpose::STANDARD.decode("L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==").unwrap();
            
        let key = b"YELLOW SUBMARINE";
        let nonce = 0;
        // let format = 64;

        let decrypted_bytes = ctr::decrypt(&encrypted_bytes, key, Some(nonce));
        let res: Vec<u8> = decrypted_bytes.unwrap();
        // let res = decrypted_bytes.unwrap();

        println!("res={:?} {}", res, res.len());

        let message = std::str::from_utf8(&res).unwrap();
        println!("{}", message);

        // assert_eq!(decrypted_bytes, bytes.as_slice());
    }


    #[test]
    fn aes_128_ctr_32_bytes() {
        let bytes =
            b"There once was a boy named Harry";
        let key = b"YELLOW SUBMARINE";
        let nonce: Option<u64> = Some(4343u64);

        let encrypted_bytes = ctr::encrypt(bytes, key, nonce).unwrap();
        let decrypted_bytes = ctr::decrypt(&encrypted_bytes, key, nonce).unwrap();
        assert_eq!(decrypted_bytes, bytes)
    }

    #[test]
    fn aes_128_ctr_56_bytes() {
        let bytes =
            b"There once was a boy named Harry. Destined to be a star.";
        let key = b"YELLOW SUBMARINE";
        let nonce: Option<u64> = Some(4343u64);

        let encrypted_bytes = ctr::encrypt(bytes, key, nonce).unwrap();
        let decrypted_bytes = ctr::decrypt(&encrypted_bytes, key, nonce).unwrap();
        assert_eq!(decrypted_bytes, bytes)
    }
}
