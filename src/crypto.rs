use sha3::{Digest, Sha3_256};

pub fn derive_key(seed: &[u8], vector: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(seed);
    hasher.update(vector);
    hasher.finalize().to_vec()
}

pub fn generate_address(pub_key: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(pub_key);
    hex::encode(hasher.finalize())
}