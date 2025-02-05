use openssl::symm::{encrypt, Cipher};

pub fn encrypt_data(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_256_cbc();
    encrypt(cipher, key, None, data).unwrap()
}
