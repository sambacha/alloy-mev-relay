/// Imports the `Address` and `U256` types from the `alloy_rs::types` module.
/// These types are likely used throughout the codebase to represent Ethereum addresses
/// and 256-bit unsigned integers, respectively.
use alloy_rs::types::{Address, U256};
use serde::{Deserialize, Deserializer, Serialize};
/// types
///
///
///
///
use std::{
    fmt,
    hash::{Hash, Hasher},
};

// Define a custom deserialization function for U256 from string
fn deserialize_u256_from_string<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    U256::from_dec_str(&s).map_err(serde::de::Error::custom)
}

// Define the BidTrace struct
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BidTrace {
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub slot: U256,
    pub parent_hash: String,
    pub block_hash: String,
    pub builder_pubkey: String,
    pub proposer_pubkey: String,
    pub proposer_fee_recipient: Address,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub gas_limit: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub gas_used: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub value: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub block_number: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub num_tx: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub timestamp: U256,
    #[serde(deserialize_with = "deserialize_u256_from_string")]
    pub timestamp_ms: U256,
    // Add support for additional information in BidTrace responses
    pub additional_info: Option<String>,
}

// Define the BidResponse struct
#[derive(Debug, Clone)]
pub struct BidResponse {
    pub relay_url: String,
    pub bid_traces: Vec<BidTrace>,
}

// Implement Display for BidResponse
impl fmt::Display for BidResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bid_trace in &self.bid_traces {
            writeln!(
                f,
                "BidTrace {{ relay_url: {}, block_number: {}, builder_pubkey: {}, value: {}, num_tx: {}, timestamp_ms: {} }}",
                self.relay_url, bid_trace.block_number, bid_trace.builder_pubkey, bid_trace.value, bid_trace.num_tx, bid_trace.timestamp_ms
            )?;
        }
        Ok(())
    }
}

// Implement Display for BidTrace
impl fmt::Display for BidTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BidTrace {{ slot: {}, parent_hash: {}, block_hash: {}, builder_pubkey: {}, proposer_pubkey: {}, proposer_fee_recipient: {}, gas_limit: {}, gas_used: {}, value: {}, block_number: {}, num_tx: {}, timestamp: {} }}",
            self.slot, self.parent_hash, self.block_hash, self.builder_pubkey, self.proposer_pubkey, self.proposer_fee_recipient, self.gas_limit, self.gas_used, self.value, self.block_number, self.num_tx, self.timestamp
        )
    }
}

// Implement Hash for BidTrace
impl Hash for BidTrace {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.builder_pubkey.hash(state);
    }
}

// Implement Ord and PartialOrd for BidTrace
impl Ord for BidTrace {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for BidTrace {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Add assertions for BidTrace
impl BidTrace {
    pub fn new(
        slot: U256,
        parent_hash: String,
        block_hash: String,
        builder_pubkey: String,
        proposer_pubkey: String,
        proposer_fee_recipient: Address,
        gas_limit: U256,
        gas_used: U256,
        value: U256,
        block_number: U256,
        num_tx: U256,
        timestamp: U256,
        timestamp_ms: U256,
        additional_info: Option<String>,
    ) -> Self {
        assert!(slot > U256::zero());
        assert!(gas_limit > U256::zero());
        assert!(gas_used <= gas_limit);
        BidTrace {
            slot,
            parent_hash,
            block_hash,
            builder_pubkey,
            proposer_pubkey,
            proposer_fee_recipient,
            gas_limit,
            gas_used,
            value,
            block_number,
            num_tx,
            timestamp,
            timestamp_ms,
            additional_info,
        }
    }
}