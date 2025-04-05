use wasm_bindgen::prelude::*;
use sha3::{Digest, Sha3_256};
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct CubicKey {
    master_seed: [u8; 32],
    current_vector: [u8; 4],
}

#[wasm_bindgen]
impl CubicKey {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: &[u8]) -> Self {
        let mut master_seed = [0u8; 32];
        master_seed.copy_from_slice(&seed[..32]);
        Self {
            master_seed,
            current_vector: [0, 0, 0, 0],
        }
    }

    pub fn next_key(&mut self) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.master_seed);
        hasher.update(&self.current_vector);
        let key = hasher.finalize().to_vec();
        
        self.current_vector[1] = (self.current_vector[1] + 1) % 3;
        key
    }
}

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct NetworkState {
    peers: Vec<String>,
    #[wasm_bindgen(js_name = chainHeight)]
    chain_height: u64,
}

#[wasm_bindgen]
impl NetworkState {
    pub fn current() -> Self {
        Self {
            peers: vec![],
            chain_height: 0,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn peers(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.peers).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn chain_height(&self) -> u64 {
        self.chain_height
    }
}

#[wasm_bindgen]
pub async fn generate_proof() -> Vec<u8> {
    // Placeholder for actual proof generation
    vec![0u8; 32]
} 