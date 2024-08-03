use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use crate::blockchain::{Blockchain, Block};
use crate::consensus::{Consensus, PoWConsensus, PoSConsensus};
use crate::node::Node;
use crate::smart_contract::{SmartContract, PiSentinelSmartContract};
use crate::storage::{Storage, StorageError};

pub trait ContractManager {
    fn new(node: Arc<Node>, consensus: Arc<dyn Consensus>) -> Self;
    fn deploy_contract(&self, contract: Arc<dyn SmartContract>) -> Result<(), StorageError>;
    fn execute_contract(&self, contract_id: &str, block: Block) -> Result<bool, StorageError>;
    fn get_contract(&self, contract_id: &str) -> Option<Arc<dyn SmartContract>>;
}

pub struct PiSentinelContractManager {
    node: Arc<Node>,
    consensus: Arc<dyn Consensus>,
    blockchain: Arc<Mutex<Blockchain>>,
    storage: Arc<Mutex<Storage>>,
    contracts: HashMap<String, Arc<dyn SmartContract>>,
}

impl ContractManager for PiSentinelContractManager {
    fn new(node: Arc<Node>, consensus: Arc<dyn Consensus>) -> Self {
        PiSentinelContractManager {
            node,
            consensus,
            blockchain: node.get_blockchain(),
            storage: node.get_storage(),
            contracts: HashMap::new(),
        }
    }

    fn deploy_contract(&self, contract: Arc<dyn SmartContract>) -> Result<(), StorageError> {
        let contract_id = contract.get_node().get_id().to_string();
        self.contracts.insert(contract_id.clone(), contract);
        self.storage.lock().unwrap().store_contract(contract_id, contract)?;
        Ok(())
    }

    fn execute_contract(&self, contract_id: &str, block: Block) -> Result<bool, StorageError> {
        if let Some(contract) = self.contracts.get(contract_id) {
            contract.execute(block)
        } else {
            Err(StorageError::ContractNotFound)
        }
    }

    fn get_contract(&self, contract_id: &str) -> Option<Arc<dyn SmartContract>> {
        self.contracts.get(contract_id).cloned()
    }
}
