#![allow(dead_code)]

use magic_crypt::{ MagicCryptError, MagicCryptTrait };

pub fn encrypt(key: &str, content: &str) -> String {
  let mcrypt = new_magic_crypt!(&key, 256);
  mcrypt.encrypt_str_to_base64(content)
}

pub fn decrypt(key: &str, encrypted: &str) -> Result<String, MagicCryptError> {
  let mcrypt = new_magic_crypt!(&key, 256);
  match mcrypt.decrypt_base64_to_string(encrypted) {
    Ok(resolved) => Ok(resolved),
    Err(err) => Err(err)
  }
}

pub fn generate_random_id(length: usize) -> String {
  let mut id: String = String::new();
  while id.len() < length { id.push_str(&uuid::Uuid::new_v4().to_string().replace("-", "")) };
  id[0..length].to_string()
}

const RANDOM_CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789/*-+.&#=_";

pub fn generate_random_str(length: usize) -> String {
  let mut text: String = String::new();
  let chars = RANDOM_CHARACTERS.split("").collect::<Vec<&str>>();
  for _ in 0..length {
    let r: f32 = (rand::random::<f32>()) * RANDOM_CHARACTERS.len() as f32;
    match chars.get(r.floor() as usize) {
      Some(c) => text.push_str(c),
      _ => text.push_str(",")
    }
  }
  text
}

/// Hash a String into a sha256 result, the result have a length of 64 chars
pub fn hash(content: String) -> String {
  sha256::digest(content)
}