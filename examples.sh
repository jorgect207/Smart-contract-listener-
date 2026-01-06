#!/bin/bash

# Examples of using the Smart Contract Event Listener with Chain IDs

echo "Smart Contract Event Listener - Examples"
echo "========================================="
echo ""
echo "Make sure your .env file has RPC URLs configured for the chains you want to use!"
echo "See env.example for the format."
echo ""

# Example 1: Ethereum - USDC
echo "Example 1: Listen to USDC transfers on Ethereum (Chain ID: 1)"
echo "cargo run --release -- \\"
echo "  --chain-id 1 \\"
echo "  --contract 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 \\"
echo "  --event \"Transfer(address,address,uint256)\""
echo ""

# Example 2: Polygon - USDC
echo "Example 2: Listen to USDC transfers on Polygon (Chain ID: 137)"
echo "cargo run --release -- \\"
echo "  --chain-id 137 \\"
echo "  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \\"
echo "  --event \"Transfer(address,address,uint256)\""
echo ""

# Example 3: Arbitrum - USDC
echo "Example 3: Listen to USDC on Arbitrum (Chain ID: 42161)"
echo "cargo run --release -- \\"
echo "  --chain-id 42161 \\"
echo "  --contract 0xaf88d065e77c8cC2239327C5EDb3A432268e5831"
echo ""

# Example 4: Base - USDC
echo "Example 4: Listen to USDC on Base (Chain ID: 8453)"
echo "cargo run --release -- \\"
echo "  --chain-id 8453 \\"
echo "  --contract 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
echo ""

# Example 5: BSC - USDT
echo "Example 5: Listen to USDT on BSC (Chain ID: 56)"
echo "cargo run --release -- \\"
echo "  --chain-id 56 \\"
echo "  --contract 0x55d398326f99059fF775485246999027B3197955"
echo ""

# Example 6: Listen to all events
echo "Example 6: Listen to ALL events from a contract"
echo "cargo run --release -- \\"
echo "  --chain-id 137 \\"
echo "  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359"
echo ""

# Example 7: Start from specific block
echo "Example 7: Start monitoring from a specific block"
echo "cargo run --release -- \\"
echo "  --chain-id 1 \\"
echo "  --contract 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 \\"
echo "  --start-block 18000000"
echo ""

# Example 8: Custom poll interval
echo "Example 8: Custom poll interval (check every 5 seconds)"
echo "cargo run --release -- \\"
echo "  --chain-id 137 \\"
echo "  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \\"
echo "  --poll-interval 5"
echo ""

echo "========================================="
echo "Quick Reference:"
echo ""
echo "Chain IDs:"
echo "  1      = Ethereum Mainnet"
echo "  137    = Polygon"
echo "  42161  = Arbitrum One"
echo "  8453   = Base"
echo "  56     = Binance Smart Chain"
echo "  10     = Optimism"
echo "  43114  = Avalanche C-Chain"
echo "  250    = Fantom"
echo ""
echo "Popular Contracts:"
echo "  Ethereum USDC:  0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
echo "  Polygon USDC:   0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359"
echo "  Arbitrum USDC:  0xaf88d065e77c8cC2239327C5EDb3A432268e5831"
echo "  Base USDC:      0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
echo "  BSC USDT:       0x55d398326f99059fF775485246999027B3197955"
echo ""
echo "Common Events:"
echo "  Transfer: Transfer(address,address,uint256)"
echo "  Approval: Approval(address,address,uint256)"
echo "  Swap:     Swap(address,uint256,uint256,uint256,uint256,address)"
echo "========================================="
