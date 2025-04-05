pub struct CubePosition {
    pub face: u8,    // 1-9 from digital root
    pub depth: u8,   // 0-2 (front to back)
    pub x: u8,       // 0-2 (local cube)
    pub y: u8,       // 0-2 (local cube)
}

pub fn compute_position(tx_hash: &[u8; 32]) -> CubePosition {
    let digital_root = (tx_hash.iter().map(|b| *b as u32).sum::<u32>() % 9;
    let face = if digital_root == 0 { 9 } else { digital_root as u8 };
    
    CubePosition {
        face,
        depth: tx_hash[0] % 3,
        x: tx_hash[1] % 3,
        y: tx_hash[2] % 3,
    }
} 