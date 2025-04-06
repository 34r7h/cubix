use std::collections::HashMap;
use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};
use heed::{Database, Env, EnvOpenOptions};
use heed::types::*;
use heed::byteorder::BigEndian;
use sha2::{Digest, Sha256};
use thiserror::Error;
use serde_json;
use std::str::FromStr;

const FACE_SIZE: usize = 9;
const CUBE_SIZE: usize = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Vec<String>,
    pub to: Vec<String>,
    pub meta: TransactionMeta,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMeta {
    pub tx_type: String,
    pub sig: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    pub slots: Vec<Option<String>>,
    pub edges: [usize; 4],
    pub position: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cube {
    pub slots: Vec<Option<String>>,
    pub faces: [usize; 6],
    pub position: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stack {
    pub level: u32,
    pub blocks: Vec<Transaction>,
    pub faces: Vec<Face>,
    pub cubes: Vec<Cube>,
}

#[derive(Debug)]
pub enum StackError {
    InvalidFace,
    InvalidCube,
    InvalidStack,
    DatabaseError(heed::Error),
    IoError(std::io::Error),
}

impl std::error::Error for StackError {}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::InvalidFace => write!(f, "Invalid face"),
            StackError::InvalidCube => write!(f, "Invalid cube"),
            StackError::InvalidStack => write!(f, "Invalid stack"),
            StackError::DatabaseError(e) => write!(f, "Database error: {}", e),
            StackError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<heed::Error> for StackError {
    fn from(e: heed::Error) -> Self {
        StackError::DatabaseError(e)
    }
}

impl From<std::io::Error> for StackError {
    fn from(e: std::io::Error) -> Self {
        StackError::IoError(e)
    }
}

pub struct StackManager {
    pub stacks: HashMap<u32, Stack>,
    env: Env,
    db: Database<Str, SerdeJson<HashMap<u32, Stack>>>,
}

impl StackManager {
    pub fn new(path: &Path) -> Result<Self, StackError> {
        fs::create_dir_all(path)?;
        
        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(10 * 1024 * 1024) // 10MB
                .max_dbs(3000)
                .open(path)?
        };

        let mut txn = env.write_txn()?;
        let db: Database<Str, SerdeJson<HashMap<u32, Stack>>> = env.create_database(&mut txn, Some("stacks"))?;

        let stacks = match db.get(&txn, "0")? {
            Some(stacks) => stacks,
            None => {
                let mut stacks = HashMap::new();
                stacks.insert(0, Stack::new(0));
                db.put(&mut txn, "0", &stacks)?;
                stacks
            }
        };
        txn.commit()?;

        Ok(Self { stacks, env, db })
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), StackError> {
        let level = 0;
        let hash = self.hash_transaction(&tx);
        let digital_root = self.get_digital_root(&hash);

        // Add transaction to blocks
        let stack = self.stacks.entry(level).or_insert_with(|| Stack::new(level));
        stack.blocks.push(tx);

        // Add hash to faces
        self.add_to_faces(level, digital_root, hash)?;

        // Process faces into cubes
        self.process_faces_into_cubes(level)?;

        // Process cubes into next level
        self.process_cubes_into_next_level(level)?;

        // Save state to database
        let mut txn = self.env.write_txn()?;
        self.db.put(&mut txn, &level.to_string(), &self.stacks)?;
        txn.commit()?;

        Ok(())
    }

    fn hash_transaction(&self, tx: &Transaction) -> String {
        let mut hasher = Sha256::new();
        for from in &tx.from {
            hasher.update(from.as_bytes());
        }
        for to in &tx.to {
            hasher.update(to.as_bytes());
        }
        hasher.update(tx.meta.tx_type.as_bytes());
        hasher.update(tx.meta.sig.as_bytes());
        hasher.update(tx.timestamp.to_be_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn get_digital_root(&self, hash: &str) -> usize {
        let mut sum = 0;
        for c in hash.chars() {
            if let Some(digit) = c.to_digit(16) {
                sum += digit as usize;
            }
        }
        while sum > 9 {
            let mut new_sum = 0;
            while sum > 0 {
                new_sum += sum % 10;
                sum /= 10;
            }
            sum = new_sum;
        }
        sum
    }

    fn combine_hashes(&self, hashes: &[String]) -> String {
        let mut hasher = Sha256::new();
        for hash in hashes {
            hasher.update(hash.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    fn add_to_faces(&mut self, level: u32, digital_root: usize, hash: String) -> Result<(), StackError> {
        let mut should_process_next_level = false;
        let index = digital_root % FACE_SIZE;

        if let Some(stack) = self.stacks.get_mut(&level) {
            // First try to fill faces that are closest to completion
            let mut faces_with_slots: Vec<_> = stack.faces.iter()
                .enumerate()
                .filter(|(_, face)| !face.is_complete() && face.slots[index].is_none())
                .map(|(i, face)| (i, face.count_filled_slots()))
                .collect();
            
            // Sort faces by number of filled slots in descending order
            faces_with_slots.sort_by(|a, b| b.1.cmp(&a.1));

            if let Some((face_index, _)) = faces_with_slots.first() {
                stack.faces[*face_index].slots[index] = Some(hash.clone());
                if stack.faces[*face_index].is_complete() {
                    should_process_next_level = true;
                }
            } else {
                // If no existing face has an empty slot at index, create new face
                let mut new_face = Face::new(FACE_SIZE);
                new_face.slots[index] = Some(hash);
                let position = if !stack.faces.is_empty() {
                    let last_face = &stack.faces[stack.faces.len() - 1];
                    [
                        last_face.position[0] + 1.0,
                        last_face.position[1],
                        last_face.position[2],
                    ]
                } else {
                    [0.0, 0.0, 0.0]
                };
                new_face.position = position;
                stack.faces.push(new_face);
            }
        } else {
            // Create new stack and face if none exists
            let mut new_stack = Stack::new(level);
            let mut new_face = Face::new(FACE_SIZE);
            new_face.slots[index] = Some(hash);
            new_face.position = [0.0, 0.0, 0.0];
            new_stack.faces.push(new_face);
            self.stacks.insert(level, new_stack);
        }

        if should_process_next_level {
            self.process_faces_into_cubes(level)?;
        }

        Ok(())
    }

    fn add_to_cubes(&mut self, level: u32, digital_root: usize, hash: String) -> Result<(), StackError> {
        let stack = self.stacks.entry(level).or_insert_with(|| Stack::new(level));
        let index = digital_root % CUBE_SIZE;
        
        // First try to fill existing cubes
        for cube in &mut stack.cubes {
            if !cube.is_complete() && cube.slots[index].is_none() {
                cube.slots[index] = Some(hash.clone());
                return Ok(());
            }
        }
        
        // If no existing cube has an empty slot at index, create new cube
        let mut new_cube = Cube::new(CUBE_SIZE);
        new_cube.slots[index] = Some(hash);
        let position = if !stack.cubes.is_empty() {
            let last_cube = &stack.cubes[stack.cubes.len() - 1];
            [
                last_cube.position[0] + 1.0,
                last_cube.position[1],
                last_cube.position[2],
            ]
        } else {
            [0.0, 0.0, 0.0]
        };
        new_cube.position = position;
        stack.cubes.push(new_cube);

        Ok(())
    }

    fn process_faces_into_cubes(&mut self, level: u32) -> Result<(), StackError> {
        let mut completed_faces = Vec::new();
        let mut should_process_next_level = false;
        
        // First pass: collect completed faces and calculate their hashes and digital roots
        if let Some(stack) = self.stacks.get(&level) {
            for (face_index, face) in stack.faces.iter().enumerate() {
                if face.is_complete() {
                    let hash = face.calculate_hash();
                    let digital_root = self.get_digital_root(&hash);
                    let index = digital_root % CUBE_SIZE;
                    completed_faces.push((face_index, hash, index));
                }
            }
        }

        // Second pass: process completed faces
        if let Some(stack) = self.stacks.get_mut(&level) {
            // Sort completed faces by index to fill cubes in sequence
            completed_faces.sort_by_key(|(_face_index, _hash, index)| *index);

            for (face_index, hash, index) in completed_faces {
                // Try to fill cubes in sequence, starting from the first incomplete cube
                let mut found_slot = false;
                for cube in &mut stack.cubes {
                    if !cube.is_complete() && cube.slots[index].is_none() {
                        cube.slots[index] = Some(hash.clone());
                        if cube.is_complete() {
                            should_process_next_level = true;
                        }
                        found_slot = true;
                        break;
                    }
                }

                // If no existing cube has an empty slot at index, create new cube
                if !found_slot {
                    let mut new_cube = Cube::new(CUBE_SIZE);
                    new_cube.slots[index] = Some(hash);
                    let position = if !stack.cubes.is_empty() {
                        let last_cube = &stack.cubes[stack.cubes.len() - 1];
                        [
                            last_cube.position[0] + 1.0,
                            last_cube.position[1],
                            last_cube.position[2],
                        ]
                    } else {
                        [0.0, 0.0, 0.0]
                    };
                    new_cube.position = position;
                    stack.cubes.push(new_cube);
                }

                // Clear the completed face
                stack.faces[face_index] = Face::new(FACE_SIZE);
            }
        }

        if should_process_next_level {
            self.process_cubes_into_next_level(level)?;
        }

        Ok(())
    }

    fn process_cubes_into_next_level(&mut self, level: u32) -> Result<(), StackError> {
        let mut completed_cubes = Vec::new();
        
        // First pass: collect completed cubes and calculate their hashes
        if let Some(stack) = self.stacks.get(&level) {
            for (cube_index, cube) in stack.cubes.iter().enumerate() {
                if cube.is_complete() {
                    let hash = cube.calculate_hash();
                    let digital_root = self.get_digital_root(&hash);
                    let index = digital_root % FACE_SIZE;
                    completed_cubes.push((cube_index, hash, index));
                }
            }
        }

        // Sort completed cubes by index to fill faces in sequence
        completed_cubes.sort_by_key(|(_cube_index, _hash, index)| *index);

        // Store cube indices for later cleanup
        let cube_indices: Vec<_> = completed_cubes.iter().map(|(index, _, _)| *index).collect();

        // Process completed cubes
        for (_cube_index, hash, index) in completed_cubes {
            // Add to faces at the next level
            self.add_to_faces(level + 1, index, hash)?;
        }

        // Clear completed cubes
        if let Some(stack) = self.stacks.get_mut(&level) {
            for cube_index in cube_indices {
                stack.cubes[cube_index] = Cube::new(CUBE_SIZE);
            }
        }

        Ok(())
    }
}

impl Face {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
            edges: [0; 4],
            position: [0.0; 3],
        }
    }

    fn is_complete(&self) -> bool {
        self.slots.iter().all(|x| x.is_some())
    }

    fn can_connect_to(&self, other: &Face) -> bool {
        // Check if faces are adjacent in exactly one dimension
        let mut diff_count = 0;
        for i in 0..3 {
            if (self.position[i] - other.position[i]).abs() == 1.0 {
                diff_count += 1;
            }
        }
        diff_count == 1
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        for slot in &self.slots {
            if let Some(slot) = slot {
                hasher.update(slot.as_bytes());
            }
        }
        format!("{:x}", hasher.finalize())
    }

    fn count_filled_slots(&self) -> usize {
        self.slots.iter().filter(|slot| slot.is_some()).count()
    }
}

impl Cube {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
            faces: [0; 6],
            position: [0.0; 3],
        }
    }

    fn is_complete(&self) -> bool {
        self.slots.iter().all(|x| x.is_some())
    }

    fn can_form_from_faces(&self, faces: &[Face]) -> bool {
        if faces.len() < 6 {
            return false;
        }

        // Check if we have 6 faces that can form a cube
        // Each face should be adjacent to exactly 4 other faces
        for i in 0..faces.len() {
            let mut adjacent_count = 0;
            for j in 0..faces.len() {
                if i != j && faces[i].can_connect_to(&faces[j]) {
                    adjacent_count += 1;
                }
            }
            if adjacent_count != 4 {
                return false;
            }
        }

        true
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        for slot in &self.slots {
            if let Some(slot) = slot {
                hasher.update(slot.as_bytes());
            }
        }
        format!("{:x}", hasher.finalize())
    }
}

impl Stack {
    pub fn new(level: u32) -> Self {
        Self {
            level,
            blocks: Vec::new(),
            faces: Vec::new(),
            cubes: Vec::new(),
        }
    }

    fn try_form_cube(&mut self) -> Option<Cube> {
        let complete_faces: Vec<Face> = self.faces.iter()
            .filter(|face| face.is_complete())
            .cloned()
            .collect();

        if complete_faces.len() >= 6 {
            let mut cube = Cube::new(CUBE_SIZE);
            if cube.can_form_from_faces(&complete_faces) {
                let avg_pos = complete_faces.iter().fold([0.0; 3], |mut acc, face| {
                    acc[0] += face.position[0];
                    acc[1] += face.position[1];
                    acc[2] += face.position[2];
                    acc
                });
                cube.position = [
                    avg_pos[0] / 6.0,
                    avg_pos[1] / 6.0,
                    avg_pos[2] / 6.0,
                ];
                return Some(cube);
            }
        }
        None
    }
}