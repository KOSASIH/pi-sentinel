use std::collections::{HashMap, HashSet};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::blockchain::{Block, Blockchain};
use crate::node::{Node, NodeId};
use crate::smart_contract::{SmartContract, PiSentinelSmartContract};
use crate::storage::{Storage, PiSentinelStorage};

pub trait P2P {
    fn new(node: Arc<Node>, storage: Arc<dyn Storage>) -> Self;
    fn start(&self) -> Result<(), P2PError>;
    fn connect(&self, addr: SocketAddr) -> Result<(), P2PError>;
    fn disconnect(&self, addr: SocketAddr) -> Result<(), P2PError>;
    fn send_block(&self, addr: SocketAddr, block: Block) -> Result<(), P2PError>;
    fn send_contract(&self, addr: SocketAddr, contract: Arc<dyn SmartContract>) -> Result<(), P2PError>;
    fn broadcast_block(&self, block: Block) -> Result<(), P2PError>;
    fn broadcast_contract(&self, contract: Arc<dyn SmartContract>) -> Result<(), P2PError>;
}

pub struct PiSentinelP2P {
    node: Arc<Node>,
    storage: Arc<dyn Storage>,
    connections: HashMap<SocketAddr, TcpStream>,
    listeners: HashSet<TcpListener>,
    lock: Mutex<()>,
}

impl P2P for PiSentinelP2P {
    fn new(node: Arc<Node>, storage: Arc<dyn Storage>) -> Self {
        PiSentinelP2P {
            node,
            storage,
            connections: HashMap::new(),
            listeners: HashSet::new(),
            lock: Mutex::new(()),
        }
    }

    fn start(&self) -> Result<(), P2PError> {
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

    fn connect(&self, addr: SocketAddr) -> Result<(), P2PError> {
        self.lock.lock().unwrap();
        let stream = TcpStream::connect(addr)?;
        self.connections.insert(addr, stream);
        Ok(())
    }

    fn disconnect(&self, addr: SocketAddr) -> Result<(), P2PError> {
        self.lock.lock().unwrap();
        self.connections.remove(&addr);
        Ok(())
    }

    fn send_block(&self, addr: SocketAddr, block: Block) -> Result<(), P2PError> {
        self.lock.lock().unwrap();
        if let Some(stream) = self.connections.get(&addr) {
            let data = serde_json::to_string(&block)?;
            stream.write_all(data.as_bytes())?;
            Ok(())
        } else {
            Err(P2PError::ConnectionNotFound)
        }
    }

    fn send_contract(&self, addr: SocketAddr, contract: Arc<dyn SmartContract>) -> Result<(), P2PError> {
        self.lock.lock().unwrap();
        if let Some(stream) = self.connections.get(&addr) {
            let data = serde_json::to_string(&contract)?;
            stream.write_all(data.as_bytes())?;
            Ok(())
        } else {
            Err(P2PError::ConnectionNotFound)
        }
    }

    fn broadcast_block(&self, block: Block) -> Result<(), P2PError> {
        self.lock.lock().unwrap();
        for (_, stream) in self.connections.iter() {
            self.send_block(stream, block.clone())?;
        }
        Ok(())
    }

    fn broadcast_contract(&self, contract: Arc<dyn SmartContract>) -> Result<(), P2PError> {
        self.lock.lock().unwrap();
        for (_, stream) in self.connections.iter() {
            self.send_contract(stream, contract.clone())?;
        }
        Ok(())
    }
}

impl PiSentinelP2P {
    fn handle_incoming_connection(&self, stream: TcpStream) {
        // Handle incoming connection logic here
        //...
    }
}

#[derive(Debug)]
enum P2PError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    ConnectionNotFound,
}

impl From<std::io::Error> for P2PError {
    fn from(err: std::io::Error) -> Self {
        P2PError::IoError(err)
    }
}

impl From<serde_json::Error> for P2PError {
    fn from(err: serde_json::Error) -> Self {
        P2PError::JsonError(err)
    }
}
