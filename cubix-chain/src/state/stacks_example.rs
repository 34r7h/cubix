use std::path::Path;
use std::fs;
use rand::Rng;
use cubix_chain::state::stacks::{StackManager, Transaction, TransactionMeta};
use heed::EnvOpenOptions;

fn generate_random_transaction() -> Transaction {
    let mut rng = rand::thread_rng();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    
    let from = vec![format!("0x{:x}", rng.gen::<u64>())];
    let to = vec![format!("0x{:x}", rng.gen::<u64>())];
    
    Transaction {
        timestamp,
        from,
        to,
        meta: TransactionMeta {
            tx_type: "asset".to_string(),
            sig: format!("0x{:x}", rng.gen::<u64>()),
        },
    }
}

fn print_detailed_state(stack_manager: &StackManager) {
    println!("\nDetailed Stack State:");
    for (level, stack) in &stack_manager.stacks {
        println!("\nLevel {}:", level);
        println!("Blocks ({}):", stack.blocks.len());
        if !stack.blocks.is_empty() {
            for (i, tx) in stack.blocks.iter().enumerate() {
                let from = tx.from.first().map(|s| &s[..]).unwrap_or("-");
                let to = tx.to.first().map(|s| &s[..]).unwrap_or("-");
                println!("  {}: {} -> {} ({})", i, from, to, tx.meta.tx_type);
            }
        }

        println!("\nFaces ({}):", stack.faces.len());
        for (i, face) in stack.faces.iter().enumerate() {
            let filled = face.slots.iter().filter(|x| x.is_some()).count();
            println!("  Face {}: {}/9 filled", i, filled);
            for (j, slot) in face.slots.iter().enumerate() {
                if let Some(hash) = slot {
                    println!("    Slot {}: {:.10}...", j, hash);
                }
            }
        }

        println!("\nCubes ({}):", stack.cubes.len());
        for (i, cube) in stack.cubes.iter().enumerate() {
            let filled = cube.slots.iter().filter(|x| x.is_some()).count();
            println!("  Cube {}: {}/3 filled", i, filled);
            for (j, slot) in cube.slots.iter().enumerate() {
                if let Some(hash) = slot {
                    println!("    Slot {}: {:.10}...", j, hash);
                }
            }
        }

        let completed_faces = stack.faces.iter()
            .filter(|face| face.slots.iter().all(|x| x.is_some()))
            .count();
        let completed_cubes = stack.cubes.iter()
            .filter(|cube| cube.slots.iter().all(|x| x.is_some()))
            .count();
        
        println!("\nSummary:");
        println!("  Total transactions: {}", stack.blocks.len());
        println!("  Total faces: {} ({} complete)", stack.faces.len(), completed_faces);
        println!("  Total cubes: {} ({} complete)", stack.cubes.len(), completed_cubes);
        println!("  Total slots in faces: {}", stack.faces.len() * 9);
        println!("  Total slots in cubes: {}", stack.cubes.len() * 3);
        println!("  Total filled slots in faces: {}", stack.faces.iter().map(|face| face.slots.iter().filter(|x| x.is_some()).count()).sum::<usize>());
        println!("  Total filled slots in cubes: {}", stack.cubes.iter().map(|cube| cube.slots.iter().filter(|x| x.is_some()).count()).sum::<usize>());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("tmp.mdb");
    let _ = std::fs::remove_dir_all(path);
    std::fs::create_dir_all(path)?;

    let mut manager = StackManager::new(path)?;

    // Process 130 transactions
    for i in 0..130 {
        let tx = Transaction {
            from: vec![format!("from{}", i)],
            to: vec![format!("to{}", i)],
            meta: TransactionMeta {
                tx_type: "transfer".to_string(),
                sig: format!("sig{}", i),
            },
            timestamp: 0,
        };
        manager.add_transaction(tx)?;
    }

    // Print state after 130 transactions
    println!("State after 130 transactions:");
    for (level, stack) in &manager.stacks {
        println!("Level {}:", level);
        println!("  Blocks: {}", stack.blocks.len());
        println!("  Faces: {}", stack.faces.len());
        println!("  Cubes: {}", stack.cubes.len());

        println!("  Face details:");
        for (i, face) in stack.faces.iter().enumerate() {
            let filled = face.slots.iter().filter(|x| x.is_some()).count();
            println!("    Face {}: {} slots filled", i, filled);
            for (j, slot) in face.slots.iter().enumerate() {
                if let Some(hash) = slot {
                    println!("      Slot {}: {}", j, hash);
                }
            }
        }

        println!("  Cube details:");
        for (i, cube) in stack.cubes.iter().enumerate() {
            let filled = cube.slots.iter().filter(|x| x.is_some()).count();
            println!("    Cube {}: {} slots filled", i, filled);
            for (j, slot) in cube.slots.iter().enumerate() {
                if let Some(hash) = slot {
                    println!("      Slot {}: {}", j, hash);
                }
            }
        }

        let complete_faces = stack.faces.iter()
            .filter(|face| face.slots.iter().all(|x| x.is_some()))
            .count();
        let complete_cubes = stack.cubes.iter()
            .filter(|cube| cube.slots.iter().all(|x| x.is_some()))
            .count();

        println!("  Complete faces: {}", complete_faces);
        println!("  Complete cubes: {}", complete_cubes);
        println!("  Total slots in faces: {}", stack.faces.len() * 9);
        println!("  Total slots in cubes: {}", stack.cubes.len() * 3);
        println!("  Total filled slots in faces: {}", stack.faces.iter().map(|face| face.slots.iter().filter(|x| x.is_some()).count()).sum::<usize>());
        println!("  Total filled slots in cubes: {}", stack.cubes.iter().map(|cube| cube.slots.iter().filter(|x| x.is_some()).count()).sum::<usize>());
    }

    // Process 10 more transactions
    for i in 130..140 {
        let tx = Transaction {
            from: vec![format!("from{}", i)],
            to: vec![format!("to{}", i)],
            meta: TransactionMeta {
                tx_type: "transfer".to_string(),
                sig: format!("sig{}", i),
            },
            timestamp: 0,
        };
        manager.add_transaction(tx)?;
    }

    // Print state after 140 transactions
    println!("\nState after 140 transactions:");
    for (level, stack) in &manager.stacks {
        println!("Level {}:", level);
        println!("  Blocks: {}", stack.blocks.len());
        println!("  Faces: {}", stack.faces.len());
        println!("  Cubes: {}", stack.cubes.len());

        println!("  Face details:");
        for (i, face) in stack.faces.iter().enumerate() {
            let filled = face.slots.iter().filter(|x| x.is_some()).count();
            println!("    Face {}: {} slots filled", i, filled);
            for (j, slot) in face.slots.iter().enumerate() {
                if let Some(hash) = slot {
                    println!("      Slot {}: {}", j, hash);
                }
            }
        }

        println!("  Cube details:");
        for (i, cube) in stack.cubes.iter().enumerate() {
            let filled = cube.slots.iter().filter(|x| x.is_some()).count();
            println!("    Cube {}: {} slots filled", i, filled);
            for (j, slot) in cube.slots.iter().enumerate() {
                if let Some(hash) = slot {
                    println!("      Slot {}: {}", j, hash);
                }
            }
        }

        let complete_faces = stack.faces.iter()
            .filter(|face| face.slots.iter().all(|x| x.is_some()))
            .count();
        let complete_cubes = stack.cubes.iter()
            .filter(|cube| cube.slots.iter().all(|x| x.is_some()))
            .count();

        println!("  Complete faces: {}", complete_faces);
        println!("  Complete cubes: {}", complete_cubes);
        println!("  Total slots in faces: {}", stack.faces.len() * 9);
        println!("  Total slots in cubes: {}", stack.cubes.len() * 3);
        println!("  Total filled slots in faces: {}", stack.faces.iter().map(|face| face.slots.iter().filter(|x| x.is_some()).count()).sum::<usize>());
        println!("  Total filled slots in cubes: {}", stack.cubes.iter().map(|cube| cube.slots.iter().filter(|x| x.is_some()).count()).sum::<usize>());
    }

    // Process 10 more transactions
    for i in 140..150 {
        let tx = Transaction {
            from: vec![format!("from{}", i)],
            to: vec![format!("to{}", i)],
            meta: TransactionMeta {
                tx_type: "transfer".to_string(),
                sig: format!("sig{}", i),
            },
            timestamp: 0,
        };
        manager.add_transaction(tx)?;
    }

    // Clean up
    std::fs::remove_dir_all(path)?;

    Ok(())
}
