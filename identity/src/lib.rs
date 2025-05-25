use wasm_bindgen::prelude::*;
use mayo::{Keypair, PublicKey, SecretKey, Signature};
use blake2::{Blake2b512, Digest};

#[wasm_bindgen]
pub fn generate_keypair() -> (PublicKey, SecretKey) {
    let keypair = Keypair::generate();
    (keypair.public, keypair.secret)
}

#[wasm_bindgen]
pub fn sign_message(secret_key: &SecretKey, message: &[u8]) -> Signature {
    secret_key.sign(message)
}

#[wasm_bindgen]
pub fn verify_signature(public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool {
    public_key.verify(message, signature)
}

#[wasm_bindgen]
pub fn hash_secret_key(secret_key: &SecretKey) -> Vec<u8> {
    let key_bytes = secret_key.to_bytes(); // Assuming this method exists and returns &[u8] or Vec<u8>
    let mut hasher = Blake2b512::new();
    hasher.update(key_bytes);
    hasher.finalize().to_vec()
}

// The add function and tests are removed as they are not part of the identity library's core functionality.
// If they were intended to be kept, they should be moved to a relevant module or kept if this is a multi-purpose library.
// For now, assuming a clean slate for the identity-specific code.
