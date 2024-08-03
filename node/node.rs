use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::blockchain::{Blockchain, Block};
use crate::config::Config;
use crate::network::{Network, Peer};
use crate::storage::{Storage, StorageError};

pub struct Node {
    config: Arc<Config>,
    blockchain: Arc<Mutex<Blockchain>>,
    network: Arc<Mutex<Network>>,
    storage: Arc<Mutex<Storage>>,
}

impl Node {
    pub fn new(config: Arc<Config>) -> Self {
        Node {
            config,
            blockchain: Arc::new(Mutex::new(Blockchain::new())),
            network: Arc::new(Mutex::new(Network::new())),
            storage: Arc::new(Mutex::new(Storage::new())),
        }
    }

    pub fn start(&self) {
        self.start_listening();
        self.start_syncing();
    }

    fn start_listening(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.config.node.address, self.config.node.port)).unwrap();
        thread::spawn(move || {
            for stream in listener.incoming() {
                self.handle_incoming_connection(stream.unwrap());
            }
        });
    }

    fn start_syncing(&self) {
        thread::spawn(move || {
            loop {
                self.sync_with_peers();
                thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }

    fn handle_incoming_connection(&self, stream: TcpStream) {
        // Handle incoming connection from a peer
        // ...
    }

    fn sync_with_peers(&self) {
        // Sync with peers to ensure blockchain consistency
        // ...
    }

    pub fn add_block(&self, block: Block) -> Result<(), StorageError> {
        self.storage.lock().unwrap().add_block(block)
    }

    pub fn get_blockchain(&self) -> Arc<Mutex<Blockchain>> {
        self.blockchain.clone()
    }

    pub fn get_network(&self) -> Arc<Mutex<Network>> {
        self.network.clone()
    }

    pub fn get_storage(&self) -> Arc<Mutex<Storage>> {
        self.storage.clone()
    }
        }
