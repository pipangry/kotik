use cfb8::cipher::{AsyncStreamCipher, KeyIvInit};
use rand::Rng;

#[derive(Debug)]
pub enum CipherError {
    InvalidKeyLength,
    InvalidLength,
}

type Aes256Cfb8Encryptor = cfb8::Encryptor<aes::Aes256>;
type Aes256Cfb8Decryptor = cfb8::Decryptor<aes::Aes256>;

fn get_key_as_bytes(key: &str) -> Result<&[u8], CipherError> {
    let key = key.as_bytes();
    if key.len() != 32 {
        return Err(CipherError::InvalidKeyLength);
    }
    Ok(key)
}

pub fn aes256_cbf8_encrypt(key: &str, mut bytes: Vec<u8>) -> Result<Vec<u8>, CipherError> {
    let key = get_key_as_bytes(key)?;
    let iv = key.split_at(16).0;

    let encryptor = Aes256Cfb8Encryptor::new_from_slices(key, iv)
        .map_err(|_| CipherError::InvalidLength)?;

    encryptor.encrypt(&mut bytes);

    Ok(bytes)
}

pub fn aes256_cfb8_decrypt(key: &str, mut bytes: Vec<u8>) -> Result<Vec<u8>, CipherError> {
    let key = get_key_as_bytes(key)?;
    let iv = key.split_at(16).0;

    let decryptor = Aes256Cfb8Decryptor::new_from_slices(key, iv)
        .map_err(|_| CipherError::InvalidLength)?;

    decryptor.decrypt(&mut bytes);
    
    Ok(bytes)
}

pub fn generate_random_key() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";

    let mut rng = rand::rng();
    (0..32)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}