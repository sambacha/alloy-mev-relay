use alloy_rs::prelude::*;
use block_bid_watcher::relay_clients::RelayClients;
use std::error::Error;

use alloy_rs::prelude::*;
use block_bid_watcher::relay_clients::RelayClients;
use serde::Deserialize;
#[tokio::main]
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Deserialize)]
struct Config {
    relay_urls: Vec<String>,
    provider_url: String,
}

async fn load_config() -> Result<Config, Box<dyn Error>> {
    let mut file = File::open("config.json").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load configuration
    let config = load_config().await?;

    // Initialize RelayClients with URLs
    let mut relay_clients = RelayClients::new(config.relay_urls);

    let mut bid_manager_receiver = relay_clients.bid_manager.subscribe_to_top_bids().await?;

    // Spawn a task to handle received messages from the bid manager
    tokio::spawn(async move {
        while let Some(data) = bid_manager_receiver.recv().await {
            println!("New Highest Bid: {}", data);
        }
    });

    // Connect to the WebSocket provider
    let provider = AlloyProvider::connect(&config.provider_url).await?;

    // Subscribe to new blocks
    let mut block_stream = provider.subscribe_blocks().await?;

    // Process new blocks as they come in
    while let Some(block) = block_stream.next().await {
        let block_number = block.number.expect("Block number not found in new block");
        println!("New block: {}", block_number);

        // Poll for each new block
        relay_clients
            .poll_for(block_number + U64::one(), 1, 12)
            .await
    }

    Ok(())
}
