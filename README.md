# Smart Contract Event Listener

<div align="center">

**A high-performance, real-time blockchain event monitoring tool built in Rust**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)


[Features](#features) • [Quick Start](#quick-start) • [Documentation](#documentation) • [Examples](#examples)

</div>

---

## Overview

A production-ready tool for monitoring smart contract events across any EVM-compatible blockchain. Built with Rust for performance and reliability, this listener provides real-time event tracking with minimal resource usage.

### Key Capabilities

- **Multi-Chain Support**: Works seamlessly with Ethereum, Polygon, Arbitrum, Base, BSC, Optimism, Avalanche, and more
- **Intelligent Polling**: Efficient block-by-block processing with configurable intervals
- **Event Filtering**: Monitor all events or filter by specific event signatures
- **Type Safety**: Leverages Rust's type system to prevent common errors
- **Zero Configuration**: Simple CLI interface with sensible defaults

## Features

| Feature | Description |
|---------|-------------|
| 🌐 **Multi-Chain** | Support for all EVM-compatible blockchains |
| ⚡ **Real-Time** | Near-instant event detection with configurable polling |
| 🎯 **Selective Monitoring** | Filter by contract address and event signature |
| 🔄 **Resumable** | Track progress automatically, resume from last block |
| 🛡️ **Type-Safe** | Rust's compile-time guarantees prevent runtime errors |
| 📊 **Multiple Output Formats** | JSON, pretty, or compact format for any use case |
| 🔗 **Webhook Support** | Send events to HTTP endpoints in real-time |
| 💾 **File Output** | Save events to file for processing or analysis |
| 🔧 **Configurable** | Flexible configuration via CLI or environment variables |
| 🚀 **Extensible** | Build automation and integrations on top |

## Quick Start

### Prerequisites

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **RPC Endpoint** - Get free API keys from:
  - [Alchemy](https://www.alchemy.com/) (Recommended)
  - [Infura](https://infura.io/)
  - [QuickNode](https://www.quicknode.com/)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd listener

# Build the release binary
cargo build --release

# Binary will be available at ./target/release/listener
```

### Configuration

Create a `.env` file in the project root:

```bash
# Copy the example file
cp env.example .env

# Edit with your RPC endpoints
nano .env
```

Add your RPC URLs:

```env
# Ethereum Mainnet
ETHEREUM_RPC_URL=https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY

# Polygon
POLYGON_RPC_URL=https://polygon-mainnet.g.alchemy.com/v2/YOUR_API_KEY

# Arbitrum
ARBITRUM_RPC_URL=https://arb-mainnet.g.alchemy.com/v2/YOUR_API_KEY
```

### Basic Usage

```bash
# Monitor all events from a contract on Polygon
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359

# Monitor specific Transfer events
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --event "Transfer(address,address,uint256)"
```

## Documentation

### Command Line Interface

```
listener [OPTIONS] --contract <CONTRACT>
```

#### Required Arguments

| Argument | Description |
|----------|-------------|
| `--contract`, `-c` | Smart contract address to monitor (20-byte hex address) |

#### Optional Arguments

| Argument | Default | Description |
|----------|---------|-------------|
| `--chain-id` | - | Chain ID (1=Ethereum, 137=Polygon, etc.) |
| `--rpc-url`, `-r` | - | RPC endpoint URL (overrides chain-id) |
| `--event`, `-e` | All events | Event signature to filter (e.g., "Transfer(address,address,uint256)") |
| `--start-block`, `-s` | Latest | Block number to start monitoring from |
| `--poll-interval`, `-p` | 2 | Polling interval in seconds |
| `--output-format` | pretty | Output format: `pretty`, `json`, or `compact` |
| `--output-file` | - | File path to save events (JSON Lines format) |
| `--webhook-url` | - | HTTP endpoint to POST events to |

**Note**: Either `--chain-id` or `--rpc-url` must be provided.

### Supported Networks

#### Mainnets

| Network | Chain ID | Environment Variable | Block Time |
|---------|----------|---------------------|------------|
| Ethereum | `1` | `ETHEREUM_RPC_URL` | ~12s |
| Polygon | `137` | `POLYGON_RPC_URL` | ~2s |
| Arbitrum One | `42161` | `ARBITRUM_RPC_URL` | ~0.3s |
| Base | `8453` | `BASE_RPC_URL` | ~2s |
| Binance Smart Chain | `56` | `BSC_RPC_URL` | ~3s |
| Optimism | `10` | `OPTIMISM_RPC_URL` | ~2s |
| Avalanche C-Chain | `43114` | `AVALANCHE_RPC_URL` | ~2s |
| Fantom | `250` | `FANTOM_RPC_URL` | ~1s |

#### Testnets

| Network | Chain ID | Environment Variable |
|---------|----------|---------------------|
| Sepolia | `11155111` | `SEPOLIA_RPC_URL` |
| Mumbai | `80001` | `MUMBAI_RPC_URL` |

### Event Signature Format

Event signatures must match Solidity event declarations without parameter names:

```solidity
// Solidity Event
event Transfer(address indexed from, address indexed to, uint256 value);

// Listener Signature
"Transfer(address,address,uint256)"
```

**Important**: Omit parameter names and the `indexed` keyword—include only types in order.

## Integration & Automation

The listener provides multiple output formats and integration methods for building automation:

### JSON Output (for scripts/automation)

```bash
# Output events as JSON (one per line)
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --output-format json
```

### Save to File

```bash
# Save all events to a file (JSONL format)
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --output-format json \
  --output-file events.jsonl
```

### Webhook Integration

```bash
# Send events to your webhook endpoint
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --webhook-url https://your-server.com/webhook
```

### Pipe to Other Tools

```bash
# Use with jq, grep, or other Unix tools
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --output-format json | \
  jq -r '.block_number'
```

**📖 For complete integration examples, see [INTEGRATION.md](INTEGRATION.md)**

Examples include:
- Python/Node.js integration scripts
- Trading bots
- Discord/Telegram notifications
- Database storage
- Multi-chain aggregation

## Examples

### Monitor Token Transfers

```bash
# USDC transfers on Ethereum
cargo run --release -- \
  --chain-id 1 \
  --contract 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 \
  --event "Transfer(address,address,uint256)"

# USDC transfers on Polygon
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --event "Transfer(address,address,uint256)"
```

### Monitor NFT Activity

```bash
# Bored Ape Yacht Club transfers
cargo run --release -- \
  --chain-id 1 \
  --contract 0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D \
  --event "Transfer(address,address,uint256)"
```

### Monitor DeFi Protocols

```bash
# Uniswap V2 Swap events
cargo run --release -- \
  --chain-id 1 \
  --contract 0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc \
  --event "Swap(address,uint256,uint256,uint256,uint256,address)"

# Monitor all events from a contract
cargo run --release -- \
  --chain-id 1 \
  --contract 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
```

### Historical Event Analysis

```bash
# Start from a specific block
cargo run --release -- \
  --chain-id 1 \
  --contract 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 \
  --start-block 18000000 \
  --event "Transfer(address,address,uint256)"
```

### Custom RPC Endpoint

```bash
# Use a custom RPC endpoint (bypasses chain-id)
cargo run --release -- \
  --contract 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 \
  --rpc-url https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY
```

### Adjust Polling Frequency

```bash
# Poll every 5 seconds (reduce API usage)
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --poll-interval 5
```

## How It Works

### Architecture

```
┌─────────────────┐
│  Configuration  │  ← CLI args + .env file
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   RPC Client    │  ← Connect to blockchain
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Polling Loop   │  ← Check for new blocks
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Event Filtering │  ← Apply contract + event filters
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Event Display   │  ← Format and output events
└─────────────────┘
```

### Processing Flow

1. **Initialization**: Parse CLI arguments and connect to RPC endpoint
2. **Block Tracking**: Determine starting block (user-specified or latest)
3. **Polling**: Query blockchain at regular intervals for new blocks
4. **Filtering**: Apply contract address and event signature filters
5. **Processing**: Retrieve and display matching events
6. **Advancement**: Update current block pointer and repeat

### Block Processing Strategy

The listener uses an **incremental processing** strategy:

- Processes each block exactly **once**
- Tracks progress with `current_block` pointer
- Only queries **new blocks** since last check
- Efficient—no redundant API calls or duplicate processing

**Example Timeline:**
```
Start: Block 1000
Poll 1: Process blocks 1000-1005 → current_block = 1006
Poll 2: Process blocks 1006-1010 → current_block = 1011
Poll 3: No new blocks (1011 = 1011) → wait
Poll 4: Process blocks 1011-1015 → current_block = 1016
```

## Performance Considerations

### RPC Rate Limits

Free-tier RPC endpoints typically allow:
- **Alchemy**: 300 requests/second (free tier)
- **Infura**: 100,000 requests/day (free tier)
- **QuickNode**: Varies by plan

**Recommendations:**
- Use `--poll-interval` to control request frequency
- Monitor high-activity contracts during off-peak hours
- Upgrade to paid plans for production use

### Resource Usage

Typical resource consumption:
- **Memory**: ~10-50 MB
- **CPU**: <1% (during polling intervals)
- **Network**: 1-5 KB per request

### Optimization Tips

1. **Increase poll interval** for slower chains (Ethereum: 10-12s recommended)
2. **Use event filters** to reduce data transfer
3. **Start from recent blocks** unless historical data is needed
4. **Run multiple instances** for multiple contracts (separate processes)

## Output Format

When an event is detected:

```
╔════════════════════════════════════════════════════════════
║ Event Detected!
║ Time: 2026-01-06 15:30:45
║ Block: 18500123
║ Transaction: 0xabc123...def789
║ Log Index: 42
╠════════════════════════════════════════════════════════════
║ Topics:
║   [0] 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
║   [1] 0x000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48
║   [2] 0x000000000000000000000000742d35cc6634c0532925a3b844bc9e7595f0beb2
║ Data: 00000000000000000000000000000000000000000000000000000000000f4240
╚════════════════════════════════════════════════════════════
```

**Event Components:**
- **Topics[0]**: Event signature hash (keccak256 of signature)
- **Topics[1-3]**: Indexed parameters (if any)
- **Data**: Non-indexed parameters (hex encoded)

## Troubleshooting

### Connection Issues

**Error**: `Failed to connect to RPC endpoint`

**Solutions:**
- Verify RPC URL is correct and accessible
- Check API key is valid and not expired
- Test endpoint manually:
  ```bash
  curl -X POST https://YOUR_RPC_URL \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
  ```
- Check network connectivity and firewall settings

### Authentication Errors

**Error**: `Deserialization Error: Must be authenticated!`

**Solutions:**
- Replace placeholder `YOUR_API_KEY` with actual API key in `.env`
- Verify API key has necessary permissions
- For free public RPCs, update `.env`:
  ```env
  POLYGON_RPC_URL=https://polygon-rpc.com/
  ```

### Invalid Contract Address

**Error**: `Invalid contract address`

**Solutions:**
- Ensure address is 42 characters (including `0x` prefix)
- Verify address is checksummed correctly
- Confirm contract exists on target network
- Check for typos in address

### No Events Detected

**Possible Causes:**
1. Contract is not emitting events
2. Wrong event signature
3. Starting block is after events occurred

**Solutions:**
- Remove `--event` filter to see all events
- Verify event signature matches contract ABI
- Use `--start-block` to scan historical blocks
- Check contract is active on selected chain

### High API Usage

**Issue**: Hitting rate limits

**Solutions:**
- Increase `--poll-interval` (e.g., 5-10 seconds)
- Use paid RPC tier for production
- Monitor only during specific time windows
- Filter events more specifically

## Development

### Building from Source

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized for performance)
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Structure

```
listener/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml           # Dependencies and metadata
├── .env                 # Environment configuration (user-created)
├── env.example          # Environment template
├── examples.sh          # Example commands
└── README.md            # This file
```

### Dependencies

Core dependencies:
- `ethers`: Ethereum and blockchain interaction
- `tokio`: Async runtime
- `clap`: CLI argument parsing
- `anyhow`: Error handling
- `chrono`: Timestamp formatting

## FAQ

**Q: Can I monitor multiple contracts simultaneously?**  
A: Run multiple instances of the listener in separate terminals, each monitoring a different contract.

**Q: Does it work with testnets?**  
A: Yes! Supports Sepolia, Mumbai, and other EVM testnets via custom RPC URLs.

**Q: Can I save events to a file?**  
A: Use shell redirection: `cargo run -- [options] > events.log`

**Q: What happens if I stop and restart?**  
A: Specify `--start-block` to resume from where you left off, or omit to start from the latest block.

**Q: Can I decode the event data?**  
A: Currently outputs raw hex. Event decoding can be added using the contract ABI.

**Q: Is this suitable for production?**  
A: Yes, but consider adding state persistence, graceful shutdown, and proper logging for production use.



## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### Guidelines

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.

## Acknowledgments

Built with:
- [ethers-rs](https://github.com/gakonst/ethers-rs) - Ethereum library for Rust
- [tokio](https://tokio.rs/) - Asynchronous runtime
- [clap](https://github.com/clap-rs/clap) - Command line argument parser

---

<div align="center">

**Made with ❤️ using Rust**

[⬆ Back to Top](#smart-contract-event-listener)

</div>
