mod consensus;
mod geometry;
mod identity;
mod network;
mod state;

use clap::{Parser, Subcommand};
use state::stacks::{Transaction, TxStack};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(name = "cubix-chain")]
#[command(about = "Cubix Chain CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Submit {
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        amount: f64,
        #[arg(short = 'y', long)]
        tx_type: String,
    },
}

async fn submit_transaction(tx_stack: &TxStack, from: &str, to: &str, amount: f64, tx_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let tx = Transaction {
        timestamp,
        from: from.to_string(),
        to: to.to_string(),
        amount,
        tx_type: tx_type.to_string(),
        signature: "placeholder_signature".to_string(),
    };

    tx_stack.push(&tx)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let tx_stack = TxStack::new(Path::new("./tx_stack")).expect("Failed to initialize tx stack");

    let cli = Cli::parse();
    match cli.command {
        Commands::Submit { from, to, amount, tx_type } => {
            match submit_transaction(&tx_stack, &from, &to, amount, &tx_type).await {
                Ok(_) => println!("Transaction submitted successfully"),
                Err(e) => eprintln!("Error submitting transaction: {}", e),
            }
        }
    }
}
