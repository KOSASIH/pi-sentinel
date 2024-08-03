use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub node: NodeConfig,
    pub consensus: ConsensusConfig,
    pub storage: StorageConfig,
    pub network: NetworkConfig,
    pub analytics: AnalyticsConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeConfig {
    pub id: String,
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsensusConfig {
    pub algorithm: String,
    pub block_time: u64,
    pub block_size: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StorageConfig {
    pub type_: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub protocol: String,
    pub peers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticsConfig {
    pub enabled: bool,
    pub prometheus: PrometheusConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrometheusConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            node: NodeConfig {
                id: "pi-sentinel-node".to_string(),
                address: "127.0.0.1".to_string(),
                port: 8080,
            },
            consensus: ConsensusConfig {
                algorithm: "ai-consensus".to_string(),
                block_time: 10,
                block_size: 1024,
            },
            storage: StorageConfig {
                type_: "local".to_string(),
                path: "./storage".to_string(),
            },
            network: NetworkConfig {
                protocol: "tcp".to_string(),
                peers: vec!["node1.pi.network".to_string(), "node2.pi.network".to_string()],
            },
            analytics: AnalyticsConfig {
                enabled: true,
                prometheus: PrometheusConfig {
                    url: "http://localhost:9090".to_string(),
                    username: "pi-sentinel".to_string(),
                    password: "password".to_string(),
                },
            },
        }
    }

    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }
    }
