#[cfg(test)]
mod tests {
    #[test]
    fn test_key_rotation() {
        let seed = [0u8; 32];
        let mut key = CubicKey::new(seed, [1, 0, 0, 0]);
        let k1 = key.next_key();
        let k2 = key.next_key();
        assert_ne!(k1, k2); // Ensure deterministic rotation
    }
} 