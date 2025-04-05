pub fn is_leader(node_id: &[u8], current_state_root: &[u8; 32]) -> bool {
    let node_root = compute_digital_root(node_id);
    let state_root = compute_digital_root(state_root);
    node_root == state_root
} 