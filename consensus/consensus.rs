use std::sync::{Arc, Mutex};
use std::thread;

use crate::blockchain::{Blockchain, Block};
use crate::node::Node;
use crate::storage::{Storage, StorageError};

pub trait Consensus {
    fn new(node: Arc<Node>) -> Self;
    fn start(&self);
    fn propose_block(&self, block: Block) -> Result<(), StorageError>;
}

pub struct PoWConsensus {
    node: Arc<Node>,
    blockchain: Arc<Mutex<Blockchain>>,
    storage: Arc<Mutex<Storage>>,
}

impl Consensus for PoWConsensus {
    fn new(node: Arc<Node>) -> Self {
        PoWConsensus {
            node,
            blockchain: node.get_blockchain(),
            storage: node.get_storage(),
        }
    }

    fn start(&self) {
        thread::spawn(move || {
            loop {
                self.mine_block();
                thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }

    fn propose_block(&self, block: Block) -> Result<(), StorageError> {
        self.storage.lock().unwrap().add_block(block)
    }
}

impl PoWConsensus {
    fn mine_block(&self) {
        // Mine a new block using Proof of Work (PoW) consensus algorithm
        //...
        let block = Block::new(/*... */);
        self.propose_block(block).unwrap();
    }
}

pub struct PoSConsensus {
    node: Arc<Node>,
    blockchain: Arc<Mutex<Blockchain>>,
    storage: Arc<Mutex<Storage>>,
}

impl Consensus for PoSConsensus {
    fn new(node: Arc<Node>) -> Self {
        PoSConsensus {
            node,
            blockchain: node.get_blockchain(),
            storage: node.get_storage(),
        }
    }

    fn start(&self) {
        thread::spawn(move || {
            loop {
                self.validate_block();
                thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }

    fn propose_block(&self, block: Block) -> Result<(), StorageError> {
        self.storage.lock().unwrap().add_block(block)
    }
}

impl PoSConsensus {
    fn validate_block(&self) {
        // Validate a new block using Proof of Stake (PoS) consensus algorithm
        //...
        let block = Block::new(/*... */);
        self.propose_block(block).unwrap();
    }
        }
