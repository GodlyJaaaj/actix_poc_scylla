use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use base64::{engine::general_purpose, Engine as _};
use std::error::Error;

// Key derivation with Argon2id (strong against both side-channel and GPU attacks)
fn derive_key(master_key: &str, salt: &[u8]) -> Result<Key<Aes256Gcm>, Box<dyn Error>> {
    let argon2 = Argon2::default();
    let salt = SaltString::encode_b64(salt).map_err(|e| format!("Salt error: {}", e))?;

    let password_hash = argon2
        .hash_password(master_key.as_bytes(), &salt)
        .map_err(|e| format!("Key derivation failed: {}", e))?;

    let hash = password_hash.hash.ok_or("No hash produced")?;
    let hash_bytes = hash.as_bytes();

    // Ensure we have enough bytes for the AES-256 key (32 bytes)
    if hash_bytes.len() < 32 {
        return Err("Insufficient bytes for encryption key".into());
    }

    let key = Key::<Aes256Gcm>::from_slice(&hash_bytes[0..32]).clone();
    Ok(key)
}

// Encrypt a value using AES-256-GCM with a derived key
pub fn encrypt_value(value: &str, master_key: &str) -> Result<(String, Vec<u8>), Box<dyn Error>> {
    // Create a random nonce
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // Derive the encryption key using the master key and nonce as salt
    let key = derive_key(master_key, &nonce)?;

    // Create the cipher
    let cipher = Aes256Gcm::new(&key);

    // Encrypt
    let ciphertext = cipher
        .encrypt(&nonce, value.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    // Encode the ciphertext as base64 for storage
    let encoded = general_purpose::STANDARD.encode(ciphertext);

    Ok((encoded, nonce.to_vec()))
}

// Decrypt a value using AES-256-GCM with a derived key
pub fn decrypt_value(
    encrypted_value: &str,
    nonce: &[u8],
    master_key: &str,
) -> Result<String, Box<dyn Error>> {
    // Decode the base64 ciphertext
    let ciphertext = general_purpose::STANDARD
        .decode(encrypted_value)
        .map_err(|e| format!("Base64 decoding failed: {}", e))?;

    // Derive the decryption key using the master key and stored nonce
    let key = derive_key(master_key, nonce)?;

    // Create the cipher
    let cipher = Aes256Gcm::new(&key);

    // Create the nonce object from the stored bytes
    let nonce = Nonce::from_slice(nonce);

    // Decrypt
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("Decryption failed: {}", e))?;

    // Convert bytes to string
    let result =
        String::from_utf8(plaintext).map_err(|e| format!("UTF-8 conversion failed: {}", e))?;

    Ok(result)
}
