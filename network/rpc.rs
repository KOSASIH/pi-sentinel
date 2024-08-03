use std::collections::{HashMap, HashSet};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::blockchain::{Block, Blockchain};
use crate::node::{Node, NodeId};
use crate::smart_contract::{SmartContract, PiSentinelSmartContract};
use crate::storage::{Storage, PiSentinelStorage};

pub trait RPC {
    fn new(node: Arc<Node>, storage: Arc<dyn Storage>) -> Self;
    fn start(&self) -> Result<(), RPCError>;
    fn call(&self, method: &str, params: Vec<String>) -> Result<String, RPCError>;
}

pub struct PiSentinelRPC {
    node: Arc<Node>,
    storage: Arc<dyn Storage>,
    listeners: HashSet<TcpListener>,
    lock: Mutex<()>,
}

impl RPC for PiSentinelRPC {
    fn new(node: Arc<Node>, storage: Arc<dyn Storage>) -> Self {
        PiSentinelRPC {
            node,
            storage,
            listeners: HashSet::new(),
            lock: Mutex::new(()),
        }
    }

    fn start(&self) -> Result<(), RPCError> {
        self.lock.lock().unwrap();
        let listener = TcpListener::bind(self.node.get_addr())?;
        self.listeners.insert(listener);
        thread::spawn(move || {
            for stream in listener.incoming() {
                self.handle_incoming_connection(stream);
            }
        });
        Ok(())
    }

    fn call(&self, method: &str, params: Vec<String>) -> Result<String, RPCError> {
        self.lock.lock().unwrap();
        match method {
            "get_block" => {
                let block_hash = params.get(0).unwrap();
                let block = self.storage.get_block(block_hash)?;
                let data = serde_json::to_string(&block)?;
                Ok(data)
            }
            "get_contract" => {
                let contract_id = params.get(0).unwrap();
                let contract = self.storage.get_contract(contract_id)?;
                let data = serde_json::to_string(&contract)?;
                Ok(data)
            }
            "send_transaction" => {
                let tx = params.get(0).unwrap();
                self.node.send_transaction(tx)?;
                Ok("Transaction sent successfully".to_string())
            }
            _ => Err(RPCError::MethodNotFound),
        }
    }
}

impl PiSentinelRPC {
    fn handle_incoming_connection(&self, stream: TcpStream) {
        // Handle incoming connection logic here
        //...
    }
}

#[derive(Debug)]
enum RPCError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    MethodNotFound,
}

impl From<std::io::Error> for RPCError {
    fn from(err: std::io::Error) -> Self {
        RPCError::IoError(err)
    }
}

impl From<serde_json::Error> for RPCError {
    fn from(err: serde_json::Error) -> Self {
        RPCError::JsonError(err)
    }
    }
