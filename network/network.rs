use std::collections::{HashMap, HashSet};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::blockchain::{Block, Blockchain};
use crate::node::{Node, NodeId};
use crate::smart_contract::{SmartContract, PiSentinelSmartContract};

pub trait Network {
    fn new(node: Arc<Node>) -> Self;
    fn connect(&self, addr: SocketAddr) -> Result<(), NetworkError>;
    fn broadcast_block(&self, block: Block) -> Result<(), NetworkError>;
    fn broadcast_contract(&self, contract: Arc<dyn SmartContract>) -> Result<(), NetworkError>;
    fn send_block(&self, addr: SocketAddr, block: Block) -> Result<(), NetworkError>;
    fn send_contract(&self, addr: SocketAddr, contract: Arc<dyn SmartContract>) -> Result<(), NetworkError>;
    fn start_listening(&self) -> Result<(), NetworkError>;
}

pub struct PiSentinelNetwork {
    node: Arc<Node>,
    connections: HashMap<SocketAddr, TcpStream>,
    listeners: HashSet<TcpListener>,
    lock: Mutex<()>,
}

impl Network for PiSentinelNetwork {
    fn new(node: Arc<Node>) -> Self {
        PiSentinelNetwork {
            node,
            connections: HashMap::new(),
            listeners: HashSet::new(),
            lock: Mutex::new(()),
        }
    }

    fn connect(&self, addr: SocketAddr) -> Result<(), NetworkError> {
        self.lock.lock().unwrap();
        let stream = TcpStream::connect(addr)?;
        self.connections.insert(addr, stream);
        Ok(())
    }

    fn broadcast_block(&self, block: Block) -> Result<(), NetworkError> {
        self.lock.lock().unwrap();
        for (_, stream) in self.connections.iter() {
            self.send_block(stream, block.clone())?;
        }
        Ok(())
    }

    fn broadcast_contract(&self, contract: Arc<dyn SmartContract>) -> Result<(), NetworkError> {
        self.lock.lock().unwrap();
        for (_, stream) in self.connections.iter() {
            self.send_contract(stream, contract.clone())?;
        }
        Ok(())
    }

    fn send_block(&self, addr: SocketAddr, block: Block) -> Result<(), NetworkError> {
        self.lock.lock().unwrap();
        if let Some(stream) = self.connections.get(&addr) {
            let data = serde_json::to_string(&block)?;
            stream.write_all(data.as_bytes())?;
            Ok(())
        } else {
            Err(NetworkError::ConnectionNotFound)
        }
    }

    fn send_contract(&self, addr: SocketAddr, contract: Arc<dyn SmartContract>) -> Result<(), NetworkError> {
        self.lock.lock().unwrap();
        if let Some(stream) = self.connections.get(&addr) {
            let data = serde_json::to_string(&contract)?;
            stream.write_all(data.as_bytes())?;
            Ok(())
        } else {
            Err(NetworkError::ConnectionNotFound)
        }
    }

    fn start_listening(&self) -> Result<(), NetworkError> {
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
}

impl PiSentinelNetwork {
    fn handle_incoming_connection(&self, stream: TcpStream) {
        // Handle incoming connection logic here
        //...
    }
}

#[derive(Debug)]
enum NetworkError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    ConnectionNotFound,
}

impl From<std::io::Error> for NetworkError {
    fn from(err: std::io::Error) -> Self {
        NetworkError::IoError(err)
    }
}

impl From<serde_json::Error> for NetworkError {
    fn from(err: serde_json::Error) -> Self {
        NetworkError::JsonError(err)
    }
                     }
