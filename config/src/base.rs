use serde::{Deserialize, Serialize};
use std::default::Default;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

use crate::types::{ChainConfig, Forks};
use crate::utils::{bytes_deserialize, bytes_serialize};

/// The base configuration for a network.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseConfig {
    pub rpc_bind_ip: IpAddr,
    pub rpc_port: u16,
    pub consensus_rpc: Option<String>,
    pub execution_rpc: String,
    #[serde(
        deserialize_with = "bytes_deserialize",
        serialize_with = "bytes_serialize"
    )]
    pub default_checkpoint: Vec<u8>,
    pub chain: ChainConfig,
    pub forks: Forks,
    pub max_checkpoint_age: u64,
    pub data_dir: Option<PathBuf>,
    pub load_external_fallback: bool,
    pub strict_checkpoint_age: bool,
}

impl Default for BaseConfig {
    fn default() -> Self {
        BaseConfig {
            rpc_bind_ip: IpAddr::V4(Ipv4Addr::LOCALHOST), // Default to "127.0.0.1"
            rpc_port: 0,
            consensus_rpc: None,
            execution_rpc: Default::default(),
            default_checkpoint: vec![],
            chain: Default::default(),
            forks: Default::default(),
            max_checkpoint_age: 0,
            data_dir: None,
            load_external_fallback: false,
            strict_checkpoint_age: false,
        }
    }
}
