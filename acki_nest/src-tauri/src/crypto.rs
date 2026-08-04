use argon2::{Algorithm, Argon2, Params, Version};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use rand::RngCore;

pub const KEY_LEN: usize = 32; // 256‑bit key for AES‑256‑GCM
pub const NONCE_LEN: usize = 12; // Standard nonce size for GCM

/// Параметры Argon2id (рекомендации OWASP): 19 МБ памяти, 2 итерации, 1 поток.
fn argon_params() -> Result<Params, argon2::Error> {
    Params::new(19 * 1024, 2, 1, Some(KEY_LEN))
}

/// Derive a fixed‑size key from a password and a salt using Argon2id.
pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; KEY_LEN], Box<dyn std::error::Error>> {
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon_params().map_err(|e| e.to_string())?);
    let mut key = [0u8; KEY_LEN];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| e.to_string())?;
    Ok(key)
}

/// Generate a random 16‑byte salt.
pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

/// Encrypt plaintext with the derived key using AES‑256‑GCM.
/// Returns (nonce, ciphertext).
pub fn encrypt(plain: &[u8], key: &[u8; KEY_LEN]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| e.to_string())?;
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, plain).map_err(|e| e.to_string())?;
    Ok((nonce_bytes.to_vec(), ciphertext))
}

/// Decrypt ciphertext with the given key and nonce.
pub fn decrypt(ciphertext: &[u8], nonce: &[u8], key: &[u8; KEY_LEN]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(nonce);
    let plain = cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())?;
    Ok(plain)
}
