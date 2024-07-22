#[cfg(test)]
mod tests {
    use super::*;
    use ethers::types::Address;
    use std::str::FromStr;

    #[test]
    fn test_bid_trace_serialization() {
        let bid_trace = BidTrace {
            slot: U256::from(12345),
            parent_hash: "0xparenthash".to_string(),
            block_hash: "0xblockhash".to_string(),
            builder_pubkey: "0xbuilderpubkey".to_string(),
            proposer_pubkey: "0xproposerpubkey".to_string(),
            proposer_fee_recipient: Address::from_str("0x0000000000000000000000000000000000000000")
                .unwrap(),
            gas_limit: U256::from(1000000),
            gas_used: U256::from(500000),
            value: U256::from(1000000000000000000),
            block_number: U256::from(12345),
            num_tx: U256::from(10),
            timestamp: U256::from(1609459200),
        };

        let serialized = serde_json::to_string(&bid_trace).unwrap();
        let deserialized: BidTrace = serde_json::from_str(&serialized).unwrap();

        assert_eq!(bid_trace, deserialized);
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use ethers::types::Address;
        use serde_json;
        use std::str::FromStr;

        #[test]
        fn test_bid_trace_serialization_with_timestamp_ms() {
            let bid_trace = BidTrace {
                slot: U256::from(12345),
                parent_hash: "0xparenthash".to_string(),
                block_hash: "0xblockhash".to_string(),
                builder_pubkey: "0xbuilderpubkey".to_string(),
                proposer_pubkey: "0xproposerpubkey".to_string(),
                proposer_fee_recipient: Address::from_str(
                    "0x0000000000000000000000000000000000000000",
                )
                .unwrap(),
                gas_limit: U256::from(1000000),
                gas_used: U256::from(500000),
                value: U256::from(1000000000000000000),
                block_number: U256::from(12345),
                num_tx: U256::from(10),
                timestamp: U256::from(1609459200),
                timestamp_ms: U256::from(1609459200123),
            };

            let serialized = serde_json::to_string(&bid_trace).unwrap();
            let deserialized: BidTrace = serde_json::from_str(&serialized).unwrap();

            assert_eq!(bid_trace, deserialized);
        }

        #[test]
        fn test_bid_trace_default_values() {
            let bid_trace = BidTrace {
                slot: U256::default(),
                parent_hash: String::new(),
                block_hash: String::new(),
                builder_pubkey: String::new(),
                proposer_pubkey: String::new(),
                proposer_fee_recipient: Address::default(),
                gas_limit: U256::default(),
                gas_used: U256::default(),
                value: U256::default(),
                block_number: U256::default(),
                num_tx: U256::default(),
                timestamp: U256::default(),
                timestamp_ms: U256::default(),
            };

            assert_eq!(bid_trace.slot, U256::default());
            assert_eq!(bid_trace.parent_hash, "");
            assert_eq!(bid_trace.block_hash, "");
            assert_eq!(bid_trace.builder_pubkey, "");
            assert_eq!(bid_trace.proposer_pubkey, "");
            assert_eq!(bid_trace.proposer_fee_recipient, Address::default());
            assert_eq!(bid_trace.gas_limit, U256::default());
            assert_eq!(bid_trace.gas_used, U256::default());
            assert_eq!(bid_trace.value, U256::default());
            assert_eq!(bid_trace.block_number, U256::default());
            assert_eq!(bid_trace.num_tx, U256::default());
            assert_eq!(bid_trace.timestamp, U256::default());
            assert_eq!(bid_trace.timestamp_ms, U256::default());
        }

        #[test]
        fn test_bid_trace_invalid_address() {
            let result = Address::from_str("invalid_address");
            assert!(result.is_err());
        }
    }
}
