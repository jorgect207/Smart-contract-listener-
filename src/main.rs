use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about = "Smart Contract Event Listener", long_about = None)]
struct Args {
    /// Smart contract address to listen to
    #[arg(short, long)]
    contract: String,

    /// Chain ID (e.g., 1=Ethereum, 137=Polygon, 42161=Arbitrum, 8453=Base, 56=BSC)
    #[arg(long)]
    chain_id: Option<u64>,

    /// RPC endpoint URL (optional, overrides chain-id)
    #[arg(short, long)]
    rpc_url: Option<String>,

    /// Event signature to filter (optional, e.g., "Transfer(address,address,uint256)")
    /// If not provided, will listen to all events
    #[arg(short, long)]
    event: Option<String>,

    /// Start block number (optional, defaults to latest)
    #[arg(short, long)]
    start_block: Option<u64>,

    /// Poll interval in milliseconds (default: 1000ms = 1 second)
    #[arg(short, long, default_value = "1000")]
    poll_interval_ms: u64,

    /// Output format: pretty, json, or compact
    #[arg(long, default_value = "pretty")]
    output_format: String,

    /// Output file path (optional, writes to stdout if not provided)
    #[arg(long)]
    output_file: Option<String>,

    /// Webhook URL to POST events to (optional)
    #[arg(long)]
    webhook_url: Option<String>,
}

/// Structured event data for JSON output and integrations
#[derive(Debug, Serialize, Deserialize, Clone)]
struct EventData {
    timestamp: String,
    chain_id: Option<u64>,
    chain_name: String,
    block_number: u64,
    transaction_hash: String,
    log_index: u64,
    contract_address: String,
    topics: Vec<String>,
    data: String,
    event_signature: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if exists
    dotenv::dotenv().ok();

    let args = Args::parse();

    // Get RPC URL: priority is --rpc-url > --chain-id > RPC_URL env
    let (rpc_url, chain_name) = if let Some(url) = args.rpc_url {
        (url, "Custom".to_string())
    } else if let Some(chain_id) = args.chain_id {
        get_rpc_url_from_chain_id(chain_id)?
    } else if let Ok(url) = std::env::var("RPC_URL") {
        (url, "Custom".to_string())
    } else {
        anyhow::bail!("Must provide --chain-id, --rpc-url, or set RPC_URL environment variable");
    };

    println!(" Starting Smart Contract Event Listener");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Chain: {}", chain_name);
    println!(" Contract: {}", args.contract);
    println!(" RPC: {}", mask_api_key(&rpc_url));
    
    if let Some(ref event_sig) = args.event {
        println!(" Event: {}", event_sig);
    } else {
        println!(" Listening to: ALL events");
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Connect to the network
    let provider = Provider::<Http>::try_from(rpc_url.as_str())
        .context("Failed to connect to RPC endpoint")?;
    let provider = Arc::new(provider);

    // Parse contract address
    let contract_address: Address = args.contract.parse()
        .context("Invalid contract address")?;

    // Determine starting block
    let from_block = if let Some(block) = args.start_block {
        block
    } else {
        provider.get_block_number().await?.as_u64()
    };

    println!(" Starting from block: {}\n", from_block);

    // Create event filter
    let mut current_block = from_block;
    let poll_interval = std::time::Duration::from_millis(args.poll_interval_ms);

    loop {
        // Get the latest block number
        let latest_block = provider.get_block_number().await?.as_u64();

        if latest_block > current_block {
            // Create filter for the new blocks
            let filter = Filter::new()
                .address(contract_address)
                .from_block(current_block)
                .to_block(latest_block);

            // Apply event topic filter if specified
            let filter = if let Some(ref event_sig) = args.event {
                let _topic = compute_event_topic(event_sig);
                filter.event(event_sig)
            } else {
                filter
            };

            // Get logs
            match provider.get_logs(&filter).await {
                Ok(logs) => {
                    for log in &logs {
                        let event_data = log_to_event_data(
                            log,
                            args.chain_id,
                            &chain_name,
                            &contract_address,
                            args.event.as_deref(),
                        );
                        
                        // Output based on format
                        match args.output_format.as_str() {
                            "json" => print_json(&event_data)?,
                            "compact" => print_compact(&event_data),
                            _ => print_pretty(&event_data),
                        }
                        
                        // Write to file if specified
                        if let Some(ref file_path) = args.output_file {
                            write_to_file(file_path, &event_data)?;
                        }
                        
                        // Send to webhook if specified
                        if let Some(ref webhook) = args.webhook_url {
                            send_webhook(webhook, &event_data).await?;
                        }
                    }
                    
                    if logs.is_empty() && args.output_format == "pretty" {
                        print!("\r Listening... (Block: {}) ", latest_block);
                        std::io::Write::flush(&mut std::io::stdout()).ok();
                    }
                }
                Err(e) => {
                    eprintln!(" Error fetching logs: {}", e);
                }
            }

            current_block = latest_block + 1;
        }

        tokio::time::sleep(poll_interval).await;
    }
}

fn get_rpc_url_from_chain_id(chain_id: u64) -> Result<(String, String)> {
    let (env_var, chain_name) = match chain_id {
        1 => ("ETHEREUM_RPC_URL", "Ethereum Mainnet"),
        137 => ("POLYGON_RPC_URL", "Polygon"),
        42161 => ("ARBITRUM_RPC_URL", "Arbitrum One"),
        8453 => ("BASE_RPC_URL", "Base"),
        56 => ("BSC_RPC_URL", "Binance Smart Chain"),
        10 => ("OPTIMISM_RPC_URL", "Optimism"),
        43114 => ("AVALANCHE_RPC_URL", "Avalanche C-Chain"),
        250 => ("FANTOM_RPC_URL", "Fantom"),
        // Testnets
        11155111 => ("SEPOLIA_RPC_URL", "Sepolia Testnet"),
        80001 => ("MUMBAI_RPC_URL", "Mumbai Testnet"),
        _ => anyhow::bail!("Unsupported chain ID: {}. Add it to your .env file with CHAIN_{}_RPC_URL", chain_id, chain_id),
    };

    let rpc_url = std::env::var(env_var)
        .with_context(|| format!("Environment variable {} not found. Add it to your .env file", env_var))?;

    Ok((rpc_url, chain_name.to_string()))
}

fn mask_api_key(url: &str) -> String {
    // Mask API keys in URLs for privacy
    if let Some(pos) = url.rfind('/') {
        if pos + 1 < url.len() {
            let (base, key) = url.split_at(pos + 1);
            if key.len() > 8 {
                return format!("{}{}...{}", base, &key[..4], &key[key.len()-4..]);
            }
        }
    }
    url.to_string()
}

fn compute_event_topic(event_sig: &str) -> H256 {
    use ethers::utils::keccak256;
    let hash = keccak256(event_sig.as_bytes());
    H256::from_slice(&hash)
}

fn log_to_event_data(
    log: &Log,
    chain_id: Option<u64>,
    chain_name: &str,
    contract_address: &Address,
    event_signature: Option<&str>,
) -> EventData {
    EventData {
        timestamp: Local::now().to_rfc3339(),
        chain_id,
        chain_name: chain_name.to_string(),
        block_number: log.block_number.map(|n| n.as_u64()).unwrap_or(0),
        transaction_hash: log
            .transaction_hash
            .map(|h| format!("{:?}", h))
            .unwrap_or_default(),
        log_index: log.log_index.map(|n| n.as_u64()).unwrap_or(0),
        contract_address: format!("{:?}", contract_address),
        topics: log.topics.iter().map(|t| format!("{:?}", t)).collect(),
        data: hex::encode(&log.data),
        event_signature: event_signature.map(String::from),
    }
}

fn print_json(event: &EventData) -> Result<()> {
    println!("{}", serde_json::to_string(event)?);
    Ok(())
}

fn print_compact(event: &EventData) {
    println!(
        "[{}] Block {} | Tx {} | Contract {} | Topics: {}",
        event.timestamp,
        event.block_number,
        &event.transaction_hash[..10],
        &event.contract_address[..10],
        event.topics.len()
    );
}

fn print_pretty(event: &EventData) {
    println!("\n╔════════════════════════════════════════════════════════════");
    println!("║ Event Detected!");
    println!("║ Time: {}", event.timestamp);
    println!("║ Chain: {} (ID: {})", event.chain_name, event.chain_id.unwrap_or(0));
    println!("║ Block: {}", event.block_number);
    println!("║ Transaction: {}", event.transaction_hash);
    println!("║ Log Index: {}", event.log_index);
    println!("║ Contract: {}", event.contract_address);
    
    if let Some(ref sig) = event.event_signature {
        println!("║ Event: {}", sig);
    }
    
    println!("╠════════════════════════════════════════════════════════════");
    
    if !event.topics.is_empty() {
        println!("║ Topics:");
        for (i, topic) in event.topics.iter().enumerate() {
            println!("║   [{}] {}", i, topic);
        }
    }
    
    if !event.data.is_empty() {
        println!("║ Data: {}", event.data);
    }
    
    println!("╚════════════════════════════════════════════════════════════\n");
}

fn write_to_file(file_path: &str, event: &EventData) -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    let json = serde_json::to_string(event)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    
    writeln!(file, "{}", json)?;
    Ok(())
}

async fn send_webhook(url: &str, event: &EventData) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(event)
        .send()
        .await?;
    
    if !response.status().is_success() {
        eprintln!("⚠️  Webhook failed: {}", response.status());
    }
    
    Ok(())
}
