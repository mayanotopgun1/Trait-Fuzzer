#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait CipherKey {
    type KeyType;
    fn new(key: &Self::KeyType) -> Self;
}

trait Cipher {
    const KEY_LEN: usize;
}

#[derive(Debug, Clone)]
pub struct Aes128CipherKey([u8; Aes128Cipher::KEY_LEN]);

impl CipherKey for Aes128CipherKey {
    type KeyType = [u8; Aes128Cipher::KEY_LEN];
    fn new(key: &Self::KeyType) -> Self {
        Self(key.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Aes128Cipher;

impl Cipher for Aes128Cipher {
    const KEY_LEN: usize = 16;
}

fn main() {}