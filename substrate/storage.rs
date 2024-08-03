use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

use serde::{Serialize, Deserialize};

use crate::blockchain::{Block, Blockchain};
use crate::smart_contract::{SmartContract, PiSentinelSmartContract};

pub trait Storage {
    fn new(path: &str) -> Self;
    fn store_block(&self, block: Block) -> Result<(), StorageError>;
    fn get_block(&self, block_hash: &str) -> Result<Option<Block>, StorageError>;
    fn store_contract(&self, contract_id: &str, contract: Arc<dyn SmartContract>) -> Result<(), StorageError>;
    fn get_contract(&self, contract_id: &str) -> Result<Option<Arc<dyn SmartContract>>, StorageError>;
    fn store_blockchain(&self, blockchain: Blockchain) -> Result<(), StorageError>;
    fn get_blockchain(&self) -> Result<Blockchain, StorageError>;
}

pub struct PiSentinelStorage {
    path: String,
    blocks: HashMap<String, Block>,
    contracts: HashMap<String, Arc<dyn SmartContract>>,
    blockchain: Blockchain,
    lock: Mutex<()>,
}

impl Storage for PiSentinelStorage {
    fn new(path: &str) -> Self {
        PiSentinelStorage {
            path: path.to_string(),
            blocks: HashMap::new(),
            contracts: HashMap::new(),
            blockchain: Blockchain::new(),
            lock: Mutex::new(()),
        }
    }

    fn store_block(&self, block: Block) -> Result<(), StorageError> {
        self.lock.lock().unwrap();
        self.blocks.insert(block.hash.clone(), block);
        self.write_to_disk()?;
        Ok(())
    }

    fn get_block(&self, block_hash: &str) -> Result<Option<Block>, StorageError> {
        self.lock.lock().unwrap();
        Ok(self.blocks.get(block_hash).cloned())
    }

    fn store_contract(&self, contract_id: &str, contract: Arc<dyn SmartContract>) -> Result<(), StorageError> {
        self.lock.lock().unwrap();
        self.contracts.insert(contract_id.to_string(), contract);
        self.write_to_disk()?;
        Ok(())
    }

    fn get_contract(&self, contract_id: &str) -> Result<Option<Arc<dyn SmartContract>>, StorageError> {
        self.lock.lock().unwrap();
        Ok(self.contracts.get(contract_id).cloned())
    }

    fn store_blockchain(&self, blockchain: Blockchain) -> Result<(), StorageError> {
        self.lock.lock().unwrap();
        self.blockchain = blockchain;
        self.write_to_disk()?;
        Ok(())
    }

    fn get_blockchain(&self) -> Result<Blockchain, StorageError> {
        self.lock.lock().unwrap();
        Ok(self.blockchain.clone())
    }
}

impl PiSentinelStorage {
    fn write_to_disk(&self) -> Result<(), StorageError> {
        let file = OpenOptions::new()
           .write(true)
           .create(true)
           .open(self.path.clone())?;

        let data = serde_json::to_string_pretty(&self)?;
        file.write_all(data.as_bytes())?;

        Ok(())
    }

    fn read_from_disk(&self) -> Result<(), StorageError> {
        let file = File::open(self.path.clone())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let data: PiSentinelStorage = serde_json::from_str(&contents)?;
        self.blocks = data.blocks;
        self.contracts = data.contracts;
        self.blockchain = data.blockchain;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum StorageError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    ContractNotFound,
    BlockNotFound,
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::IoError(err)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::JsonError(err)
    }
    }
