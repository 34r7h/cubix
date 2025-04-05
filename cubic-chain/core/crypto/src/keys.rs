use sha3::{Digest, Sha3_256};

pub struct CubicKey {
    master_seed: [u8; 32],     // Securely stored in WASM memory
    current_vector: [u8; 4],   // (face, depth, x, y)
}

impl CubicKey {
    pub fn new(master_seed: [u8; 32], initial_vector: [u8; 4]) -> Self {
        Self { master_seed, current_vector: initial_vector }
    }

    // Deterministic key rotation using vector
    pub fn next_key(&mut self) -> [u8; 32] {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.master_seed);
        hasher.update(&self.current_vector);
        let key = hasher.finalize().into();
        
        // Advance vector (e.g., increment depth mod 3)
        self.current_vector[1] = (self.current_vector[1] + 1) % 3;
        key
    }
} 