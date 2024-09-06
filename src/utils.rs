use std::fs::File;
use std::io::Result;
use std::time::{SystemTime, UNIX_EPOCH};

use anstream::println;
use owo_colors::OwoColorize as _;
use aes_gcm::aead::{Aead, KeyInit, OsRng, generic_array::GenericArray};
use aes_gcm::{Aes256Gcm, Nonce}; // Or `Aes128Gcm`
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;

pub fn get_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn touch(filename: &str) -> Result<()> {
    File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;

    Ok(())
}

pub fn encrypt(plaintext: &str) -> String {
    let key = super::cfg::get_key();

    // 将密钥转换为字节数组
    let key = GenericArray::from_slice(key.as_bytes());

    // 初始化加密器
    let cipher = Aes256Gcm::new(&key);

    // 生成 12 字节的随机 nonce
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    let nonce = Nonce::from_slice(&nonce); // 96-bits; unique per message

    // 加密
    let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())
        .expect("encryption failure!");

    // 编码为 Base64 以便于存储和传输
    let encoded_ciphertext = general_purpose::STANDARD.encode(&ciphertext);
    let encoded_nonce = general_purpose::STANDARD.encode(nonce);
    
    // 返回 nonce 和加密后的字符串
    format!("{}:{}", encoded_nonce, encoded_ciphertext)
}

pub fn decrypt(enc_str: &str) -> String {
    let key = super::cfg::get_key();

    // 将密钥转换为字节数组
    let key = GenericArray::from_slice(key.as_bytes());

    // 初始化解密器
    let cipher = Aes256Gcm::new(&key);

    // 分割 nonce 和加密后的字符串
    let parts: Vec<&str> = enc_str.split(':').collect();
    let encoded_nonce = parts.get(0).unwrap();
    let encoded_ciphertext = match parts.get(1) {
        Some(t) => t,
        None => {
            println!("{}", format!("Err: Failed to decrypt: {enc_str}").red());
            return "".to_owned();
        }
    };

    // 解码 Base64
    let nonce = match general_purpose::STANDARD.decode(encoded_nonce) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", format!("Err: Failed to decrypt: {enc_str}, err: {e}").red());
            return "".to_owned();
        }        
    };
    let ciphertext = general_purpose::STANDARD.decode(encoded_ciphertext).expect("Decoding ciphertext failed");

    // 解密
    let decrypted_plaintext = match cipher.decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref()) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", format!("Err: Failed to decrypt: {enc_str}, err: {e}").red());
            return "".to_owned();
        }
    };
    String::from_utf8(decrypted_plaintext).expect("Decryption failed")
}