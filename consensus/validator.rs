use std::sync::{Arc, Mutex};
use std::thread;

use crate::blockchain::{Blockchain, Block};
use crate::consensus::{Consensus, PoWConsensus, PoSConsensus};
use crate::node::Node;
use crate::storage::{Storage, StorageError};

pub trait Validator {
    fn new(node: Arc<Node>, consensus: Arc<dyn Consensus>) -> Self;
    fn start(&self);
    fn validate_block(&self, block: Block) -> Result<bool, StorageError>;
}

pub struct BlockValidator {
    node: Arc<Node>,
    consensus: Arc<dyn Consensus>,
    blockchain: Arc<Mutex<Blockchain>>,
    storage: Arc<Mutex<Storage>>,
}

impl Validator for BlockValidator {
    fn new(node: Arc<Node>, consensus: Arc<dyn Consensus>) -> Self {
        BlockValidator {
            node,
            consensus,
            blockchain: node.get_blockchain(),
            storage: node.get_storage(),
        }
    }

    fn start(&self) {
        thread::spawn(move || {
            loop {
                self.validate_blocks();
                thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }

    fn validate_block(&self, block: Block) -> Result<bool, StorageError> {
        // Validate the block using the consensus algorithm
        self.consensus.propose_block(block)
    }
}

impl BlockValidator {
    fn validate_blocks(&self) {
        // Get the latest blocks from the blockchain
        let blocks = self.blockchain.lock().unwrap().get_latest_blocks(10);

        // Validate each block
        for block in blocks {
            match self.validate_block(block.clone()) {
                Ok(true) => {
                    // Block is valid, add it to the blockchain
                    self.storage.lock().unwrap().add_block(block).unwrap();
                }
                Ok(false) => {
                    // Block is invalid, discard it
                }
                Err(err) => {
                    // Error occurred during validation, log and discard
                    println!("Error validating block: {}", err);
                }
            }
        }
    }
}

pub struct TransactionValidator {
    node: Arc<Node>,
    consensus: Arc<dyn Consensus>,
    blockchain: Arc<Mutex<Blockchain>>,
    storage: Arc<Mutex<Storage>>,
}

impl Validator for TransactionValidator {
    fn new(node: Arc<Node>, consensus: Arc<dyn Consensus>) -> Self {
        TransactionValidator {
            node,
            consensus,
            blockchain: node.get_blockchain(),
            storage: node.get_storage(),
        }
    }

    fn start(&self) {
        thread::spawn(move || {
            loop {
                self.validate_transactions();
                thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }

    fn validate_block(&self, block: Block) -> Result<bool, StorageError> {
        // Validate the transactions in the block using the consensus algorithm
        self.consensus.propose_block(block)
    }
}

impl TransactionValidator {
    fn validate_transactions(&self) {
        // Get the latest transactions from the blockchain
        let transactions = self.blockchain.lock().unwrap().get_latest_transactions(10);

        // Validate each transaction
        for transaction in transactions {
            match self.validate_block(transaction.clone().into()) {
                Ok(true) => {
                    // Transaction is valid, add it to the blockchain
                    self.storage.lock().unwrap().add_transaction(transaction).unwrap();
                }
                Ok(false) => {
                    // Transaction is invalid, discard it
                }
                Err(err) => {
                    // Error occurred during validation, log and discard
                    println!("Error validating transaction: {}", err);
                }
            }
        }
    }
}
