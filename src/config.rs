/// Provides configuration settings for the application, including relay URLs, provider URL, and public keys.
///
/// The `Config` struct is the main entry point for accessing configuration settings. It can be loaded from a JSON file
/// using the `new` method, and provides methods to retrieve specific configuration values, such as RPC URLs and public keys.
///
/// The configuration also includes some constant values for common relay URLs and the EVM cheat address.
// src/config.rs

use alloy_rs::types::Address;
use anyhow::Result;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub const ULTRASOUND_RELAY_URL: &str = "https://relay.ultrasound.money";
pub const AGNOSTIC_RELAY_URL: &str = "https://agnostic-relay.net";
pub const FLASHBOTS_RELAY_URL: &str = "https://boost-relay.flashbots.net";
pub const BLOXROUTE_RELAY_URL: &str = "https://bloxroute.max-profit.blxrbdn.com";
pub const AESTUS_RELAY_URL: &str = "https://mainnet.aestus.live";
pub const TITAN_RELAY_URL: &str = "https://titanrelay.xyz";

// TODO: Symbolic support
//pub const EVM_CHEAT_ADDR: Address = Address::from_str("0x000000000000000000636F6e736F6c652e6c6f67").unwrap();

#[derive(Deserialize)]
pub struct Config {
    pub relay_urls: Vec<String>,
    pub provider_url: String,
    pub comm: Common,
    pub rpc: Rpc,
    pub publickey: PublicKey,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Common {
    pub db_dir: String,
    pub cache_dir: String,
    pub output_dir: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
/// Represents the RPC URLs for various relays used by the application.
///
/// This struct contains the RPC URLs for the following relays:
/// - Ultrasound
/// - Agnostic
/// - Flashbots
/// - Bloxroute
/// - Aestus
/// - Titan
///
/// These RPC URLs are used to interact with the respective relays for various
/// blockchain-related operations, such as submitting transactions or querying
/// the state of the blockchain.
pub struct Rpc {
    pub ultrasound_rpc_url: String,
    pub agnostic_rpc_url: String,
    pub flashbots_rpc_url: String,
    pub bloxroute_rpc_url: String,
    pub aestus_rpc_url: String,
    pub titan_rpc_url: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PublicKey {
    pub ultrasound_public_keys: Vec<String>,
    pub agnostic_public_keys: Vec<String>,
    pub flashbots_public_keys: Vec<String>,
    pub bloxroute_public_keys: Vec<String>,
    pub aestus_public_keys: Vec<String>,
    pub titan_public_keys: Vec<String>,
}

/// Provides methods for loading and accessing configuration data for the application.
///
/// The `Config` struct is responsible for loading configuration data from a JSON file and
/// providing access to various configuration parameters, such as relay URLs, RPC URLs, and
/// public keys.
///
/// The `new` method is an asynchronous function that reads the configuration file and
/// deserializes the contents into a `Config` instance. The `rpc_url`, `rpc_urls`, and
/// `public_key` methods provide access to specific configuration parameters based on the
/// specified relay.
impl Config {
    pub async fn new(cfg_path: &str) -> Result<Self> {
        let mut file = File::open(cfg_path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let cfg: Config = serde_json::from_str(&contents)?;
        Ok(cfg)
    }

    /// Returns the RPC URL for the specified relay.
    ///
    /// This method takes a relay name as input and returns the corresponding RPC URL for that relay.
    /// The relay name is case-insensitive, and the method will panic if an unsupported relay name is provided.
    ///
    /// # Arguments
    /// * `relay` - The name of the relay, e.g. "ultrasound", "agnostic", "flashbots", etc.
    ///
    /// # Returns
    /// A reference to the RPC URL string for the specified relay.
    pub fn rpc_url(&self, relay: &str) -> &str {
        match relay.to_lowercase().as_str() {
            "ultrasound" => &self.rpc.ultrasound_rpc_url,
            "agnostic" => &self.rpc.agnostic_rpc_url,
            "flashbots" => &self.rpc.flashbots_rpc_url,
            "bloxroute" => &self.rpc.bloxroute_rpc_url,
            "aestus" => &self.rpc.aestus_rpc_url,
            "titan" => &self.rpc.titan_rpc_url,
            _ => panic!("Unsupported relay: {}", relay),
        }
    }

    /// Returns a HashMap containing the RPC URLs for each supported relay.
    ///
    /// The keys in the HashMap are the relay names (e.g. "ultrasound", "agnostic", "flashbots", etc.),
    /// and the values are the corresponding RPC URLs.
    pub fn rpc_urls(&self) -> HashMap<String, String> {
        [
            ("ultrasound", &self.rpc.ultrasound_rpc_url),
            ("agnostic", &self.rpc.agnostic_rpc_url),
            ("flashbots", &self.rpc.flashbots_rpc_url),
            ("bloxroute", &self.rpc.bloxroute_rpc_url),
            ("aestus", &self.rpc.aestus_rpc_url),
            ("titan", &self.rpc.titan_rpc_url),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
    }

    pub fn public_key(&self, relay: &str) -> &str {
        let keys = match relay.to_lowercase().as_str() {
            "ultrasound" => &self.publickey.ultrasound_public_keys,
            "agnostic" => &self.publickey.agnostic_public_keys,
            "flashbots" => &self.publickey.flashbots_public_keys,
            "bloxroute" => &self.publickey.bloxroute_public_keys,
            "aestus" => &self.publickey.aestus_public_keys,
            "titan" => &self.publickey.titan_public_keys,
            _ => panic!("Unsupported relay: {}", relay),
        };

        keys.choose(&mut rand::thread_rng()).unwrap_or(&keys[0])
    }
}

pub fn get_relay_url(relay: &str) -> &'static str {
    match relay.to_lowercase().as_str() {
        "ultrasound" => ULTRASOUND_RELAY_URL,
        "agnostic" => AGNOSTIC_RELAY_URL,
        "flashbots" => FLASHBOTS_RELAY_URL,
        "bloxroute" => BLOXROUTE_RELAY_URL,
        "aestus" => AESTUS_RELAY_URL,
        "titan" => TITAN_RELAY_URL,
        _ => panic!("Unsupported relay: {}", relay),
    }
}