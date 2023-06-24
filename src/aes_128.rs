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
        ciphertext: &[u8],
        iv: Option<&[u8]>,
        pad: bool,
    ) -> Result<Vec<u8>, ErrorStack> {
        ecb_helper(key, ciphertext, Mode::Encrypt, iv, pad)
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
    use crate::block::{PKCS7_padding, PKCS7_padding_remove};
    use crate::xor;
    use openssl::symm::Mode;

    // pub fn symmetry(bytes: &[u8], key: &[u8], iv: &[u8], block_size: usize, mode: Mode) -> Vec<u8> {
    //     let mut encrypted_bytes = vec![];
    //     let mut prev_block = iv.to_vec();

    //     for curr_block in bytes.chunks(block_size) {
    //         let xored_block: Vec<u8> = xor::xor_bytes2(curr_block, &prev_block);

    //         let encrypted_block = aes_128_ecb(key, xored_block.as_slice(), mode).unwrap();
    //         encrypted_bytes.extend(encrypted_block.clone());

    //         let mut prev_block = encrypted_block;
    //     }

    //     encrypted_bytes
    // }

    pub fn encrypt(bytes: &[u8], key: &[u8], iv: &[u8], block_size: usize) -> Vec<u8> {
        let mut encrypted_bytes = vec![];
        let padded_bytes = PKCS7_padding(&bytes.to_vec(), block_size);

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
        PKCS7_padding_remove(&decrypted_bytes.to_vec(), block_size)
    }
}

#[cfg(test)]
mod tests {
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
}
