use heed::{types::*, Database, EnvOpenOptions};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

const TX_STACK_DB_NAME: &str = "tx_stack";

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub timestamp: i64,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub tx_type: String,
    pub signature: String,
}

#[derive(Debug, Error)]
pub enum TxStackError {
    #[error("Database error: {0}")]
    Db(#[from] heed::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] Box<bincode::ErrorKind>),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct TxStack {
    env: heed::Env,
    db: Database<Str, SerdeBincode<Transaction>>,
}

impl TxStack {
    pub fn new(path: &Path) -> Result<Self, TxStackError> {
        std::fs::create_dir_all(path)?;

        let env = unsafe { EnvOpenOptions::new().max_dbs(1).open(path)? };

        let mut wtxn = env.write_txn()?;
        let db = env.create_database(&mut wtxn, Some(TX_STACK_DB_NAME))?;
        wtxn.commit()?;

        Ok(TxStack { env, db })
    }

    pub fn push(&self, tx: &Transaction) -> Result<(), TxStackError> {
        let mut wtxn = self.env.write_txn()?;
        let key = self.next_key()?.to_string();

        self.db.put(&mut wtxn, &key, tx)?;
        wtxn.commit()?;
        Ok(())
    }

    fn next_key(&self) -> Result<u64, heed::Error> {
        let rtxn = self.env.read_txn()?;
        let iter = self.db.iter(&rtxn)?;

        Ok(iter
            .last()
            .transpose()?
            .map_or(0, |(k, _)| k.parse().unwrap_or(0))
            + 1)
    }
}
