pub fn digital_root(hash: &[u8]) -> u8 {
    let sum: u32 = hash.iter().map(|&b| b as u32).sum();
    let root = (sum % 9) as u8;
    if root == 0 { 9 } else { root }
}

pub fn cube_position(tx_hash: &[u8]) -> (u8, u8, u8, u8) {
    let face = digital_root(tx_hash);
    let depth = tx_hash[0] % 3;
    let x = tx_hash[1] % 3;
    let y = tx_hash[2] % 3;
    let z = tx_hash[3] % 3;
    (face, depth, x, y, z)
}