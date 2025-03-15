use bcrypt::{hash, verify, DEFAULT_COST};
use std::error::Error;

// Hash a password using bcrypt
pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    let cost = DEFAULT_COST;

    match hash(password, cost) {
        Ok(hash) => Ok(hash),
        Err(e) => Err(format!("Password hashing error : {}", e).into()),
    }
}

// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
    match verify(password, hash) {
        Ok(true) => Ok(true),
        Ok(false) => Ok(false),
        Err(e) => Err(format!("Password verification error : {}", e).into()),
    }
}
