// use crate::utils::generate_random_vec;
// use crate::aes_128;
// use crate::xor;

// const EMPTY_IV: [u8; 16] = [0; 16];
// const RANDOM_STRINGS: [&str; 10] = [
//     "MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
//     "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
//     "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
//     "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
//     "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
//     "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
//     "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
//     "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
//     "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
//     "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93",
// ];

// pub struct CBCPaddingOracle {
//     aes_key: Vec<u8>
// }

// impl CBCBitflipper {
//     fn new() -> Self {
//         Self{
//             aes_key: generate_random_vec(16)

//         }
//     }

//     fn encrypt(&self, input: &str) -> Vec<u8> {
//         //  ... generate a random AES key (which it should save for all future encryptions),
//         // pad the string out to the 16-byte AES block size and CBC-encrypt it under that key, providing the caller the ciphertext and IV.
//         let random_idx = 1;
//         let bytes = RANDOM_STRINGS[random_idx].as_bytes();

//         aes_128::cbc::encrypt(&bytes, &self.aes_key, &EMPTY_IV, 16)
//     }

//     fn decrypt_and_check_padding(&self, ciphertext: &[u8]) -> Vec<u8> {
//         let res = aes_128::cbc::decrypt(ciphertext, &self.aes_key, &EMPTY_IV, 16);

//     }
// }
