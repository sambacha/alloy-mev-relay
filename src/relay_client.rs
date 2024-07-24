//use alloy_rs::prelude::*;
use crate::relay_clients::RelayClients;use std::error::Error;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::{ 
    network::{AnyNetwork, EthereumWallet},
    primitives::{address, Address, U128, U256, U64},
};
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize RelayClients with URLs
    let mut relay_clients = RelayClients::new(vec![
        "https://relay.ultrasound.money".to_string(),
        "https://agnostic-relay.net".to_string(),
        "https://boost-relay.flashbots.net".to_string(),
        "https://bloxroute.max-profit.blxrbdn.com".to_string(),
        "https://mainnet.aestus.live".to_string(),
        "https://titanrelay.xyz".to_string(),
    ]);

    let mut bid_manager_receiver = relay_clients.bid_manager.subscribe_to_top_bids().await;

    // Spawn a task to handle received messages from the bid manager
    tokio::spawn(async move {
        while let Some(data) = bid_manager_receiver.recv().await {
            println!("New Highest Bid: {}", data);
        }
    });

    let rpc_url = "wss://mainnet.infura.io/ws/v3/97498194812e457a9305b7ac71dd724b";
    // Create the provider.
    let ws = WsConnect::new(rpc_url); 
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to new blocks
//    
    // Subscribe to blocks.
  //  let mut stream = provider.subscribe_blocks().await?;
  let sub = provider.subscribe_blocks().await?;

  let mut stream = sub.into_stream().take(4);

  println!("Awaiting blocks...");
    // Take the stream and print the block number upon receiving a new block.
    let handle = tokio::spawn(async move {
        while let Some(block) = stream.next().await {
            println!(
                "Latest block number: {}",
                block.header.number.expect("Failed to get block number")
            );
        }
    });

    handle.await?;

    Ok(())
}