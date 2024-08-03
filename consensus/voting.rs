use std::sync::{Arc, Mutex};
use std::collections::{HashMap, HashSet};

use crate::blockchain::{Blockchain, Block};
use crate::consensus::{Consensus, PoWConsensus, PoSConsensus};
use crate::node::Node;
use crate::storage::{Storage, StorageError};

pub trait Voting {
    fn new(node: Arc<Node>, consensus: Arc<dyn Consensus>) -> Self;
    fn start(&self);
    fn vote(&self, block: Block) -> Result<bool, StorageError>;
}

pub struct LeaderBasedVoting {
    node: Arc<Node>,
    consensus: Arc<dyn Consensus>,
    blockchain: Arc<Mutex<Blockchain>>,
    storage: Arc<Mutex<Storage>>,
    leaders: HashSet<Arc<Node>>,
}

impl Voting for LeaderBasedVoting {
    fn new(node: Arc<Node>, consensus: Arc<dyn Consensus>) -> Self {
        LeaderBasedVoting {
            node,
            consensus,
            blockchain: node.get_blockchain(),
            storage: node.get_storage(),
            leaders: HashSet::new(),
        }
    }

    fn start(&self) {
        thread::spawn(move || {
            loop {
                self.vote_on_blocks();
                thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }

    fn vote(&self, block: Block) -> Result<bool, StorageError> {
        // Check if the block is valid according to the consensus algorithm
        if self.consensus.propose_block(block.clone()).is_err() {
            return Ok(false);
        }

        // Get the leaders from the blockchain
        let leaders = self.blockchain.lock().unwrap().get_leaders();

        // Check if the block is voted by the majority of leaders
        let mut votes = HashMap::new();
        for leader in leaders {
            if let Ok(vote) = self.storage.lock().unwrap().get_vote(leader, block.clone()) {
                *votes.entry(vote).or_insert(0) += 1;
            }
        }

        // If the block is voted by the majority of leaders, add it to the blockchain
        if votes.get(&true).unwrap_or(&0) > leaders.len() / 2 {
            self.storage.lock().unwrap().add_block(block).unwrap();
            return Ok(true);
        }

        Ok(false)
    }
}

impl LeaderBasedVoting {
    fn vote_on_blocks(&self) {
        // Get the latest blocks from the blockchain
        let blocks = self.blockchain.lock().unwrap().get_latest_blocks(10);

        // Vote on each block
        for block in blocks {
            self.vote(block.clone()).unwrap();
        }
    }
}

pub struct ByzantineFaultTolerantVoting {
    node: Arc<Node>,
    consensus: Arc<dyn Consensus>,
    blockchain: Arc<Mutex<Blockchain>>,
    storage: Arc<Mutex<Storage>>,
}

impl Voting for ByzantineFaultTolerantVoting {
    fn new(node: Arc<Node>, consensus: Arc<dyn Consensus>) -> Self {
        ByzantineFaultTolerantVoting {
            node,
            consensus,
            blockchain: node.get_blockchain(),
            storage: node.get_storage(),
        }
    }

    fn start(&self) {
        thread::spawn(move || {
            loop {
                self.vote_on_blocks();
                thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }

    fn vote(&self, block: Block) -> Result<bool, StorageError> {
        // Check if the block is valid according to the consensus algorithm
        if self.consensus.propose_block(block.clone()).is_err() {
            return Ok(false);
        }

        // Use Byzantine Fault Tolerant algorithm to vote on the block
        // ...
        Ok(true)
    }
}

impl ByzantineFaultTolerantVoting {
    fn vote_on_blocks(&self) {
        // Get the latest blocks from the blockchain
        let blocks = self.blockchain.lock().unwrap().get_latest_blocks(10);

        // Vote on each block
        for block in blocks {
            self.vote(block.clone()).unwrap();
        }
    }
    }
