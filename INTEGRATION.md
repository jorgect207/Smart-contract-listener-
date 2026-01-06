# Integration Guide

This guide shows how to integrate the Smart Contract Event Listener with other applications and build automation on top of it.

## Table of Contents

- [Output Formats](#output-formats)
- [JSON Output](#json-output)
- [File Output](#file-output)
- [Webhook Integration](#webhook-integration)
- [Building on Top](#building-on-top)
- [Integration Examples](#integration-examples)

## Output Formats

The listener supports three output formats:

### 1. Pretty Format (Default)

Human-readable format with boxes and colors:

```bash
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --output-format pretty
```

### 2. JSON Format

Machine-readable JSON for programmatic processing:

```bash
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --output-format json
```

**Output:**
```json
{"timestamp":"2026-01-06T15:30:45Z","chain_id":137,"chain_name":"Polygon","block_number":50000123,"transaction_hash":"0xabc...","log_index":42,"contract_address":"0x3c499c...","topics":["0xddf252ad...","0x000000..."],"data":"00000000...","event_signature":"Transfer(address,address,uint256)"}
```

### 3. Compact Format

One-line summary per event:

```bash
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --output-format compact
```

**Output:**
```
[2026-01-06T15:30:45Z] Block 50000123 | Tx 0xabc12345 | Contract 0x3c499c54 | Topics: 3
```

## JSON Output

### Event Data Structure

```typescript
{
  "timestamp": string,        // ISO 8601 timestamp
  "chain_id": number,          // Chain ID (137, 1, etc.)
  "chain_name": string,        // Human-readable chain name
  "block_number": number,      // Block number where event occurred
  "transaction_hash": string,  // Transaction hash
  "log_index": number,         // Log index within the block
  "contract_address": string,  // Contract that emitted the event
  "topics": string[],          // Event topics (indexed parameters)
  "data": string,              // Event data (non-indexed parameters)
  "event_signature": string?   // Event signature (if filtered)
}
```

### Using JSON Output in Scripts

#### Bash/Shell Script

```bash
#!/bin/bash

# Pipe JSON output to jq for processing
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --output-format json \
  --event "Transfer(address,address,uint256)" | \
while read -r line; do
  # Extract block number
  block=$(echo $line | jq -r '.block_number')
  # Extract transaction hash
  tx=$(echo $line | jq -r '.transaction_hash')
  
  echo "Transfer at block $block: $tx"
  
  # Do something with the event
  # curl -X POST https://your-api.com/notify -d "$line"
done
```

#### Python Script

```python
#!/usr/bin/env python3
import subprocess
import json
import sys

# Start the listener
process = subprocess.Popen([
    'cargo', 'run', '--release', '--',
    '--chain-id', '137',
    '--contract', '0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359',
    '--output-format', 'json',
    '--event', 'Transfer(address,address,uint256)'
], stdout=subprocess.PIPE, text=True)

# Process events in real-time
for line in process.stdout:
    try:
        event = json.loads(line)
        
        print(f"Event detected at block {event['block_number']}")
        print(f"  Transaction: {event['transaction_hash']}")
        print(f"  Contract: {event['contract_address']}")
        
        # Build your logic here
        if event['block_number'] > 50000000:
            print("  → Important event!")
            # Send notification, execute trade, etc.
            
    except json.JSONDecodeError:
        continue
```

#### Node.js Script

```javascript
const { spawn } = require('child_process');

// Start the listener
const listener = spawn('cargo', [
  'run', '--release', '--',
  '--chain-id', '137',
  '--contract', '0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359',
  '--output-format', 'json',
  '--event', 'Transfer(address,address,uint256)'
]);

// Process events
listener.stdout.on('data', (data) => {
  const lines = data.toString().split('\n');
  
  lines.forEach(line => {
    if (!line.trim()) return;
    
    try {
      const event = JSON.parse(line);
      
      console.log(`Event at block ${event.block_number}`);
      console.log(`  Tx: ${event.transaction_hash}`);
      
      // Build your automation
      processEvent(event);
      
    } catch (err) {
      // Skip non-JSON lines
    }
  });
});

function processEvent(event) {
  // Your custom logic here
  // - Send Discord notification
  // - Execute a trade
  // - Update a database
  // - Trigger another workflow
}
```

## File Output

Save all events to a file (JSON format, one event per line):

```bash
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --output-format json \
  --output-file events.jsonl
```

### Reading from File

```python
#!/usr/bin/env python3
import json

# Read JSONL (JSON Lines) file
with open('events.jsonl', 'r') as f:
    for line in f:
        event = json.loads(line)
        print(f"Block {event['block_number']}: {event['transaction_hash']}")
```

### Both stdout and file

```bash
# Output to stdout AND save to file
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --output-format json \
  --output-file events.jsonl | \
  jq -r '.block_number'
```

## Webhook Integration

Send events to a webhook URL (HTTP POST with JSON payload):

```bash
cargo run --release -- \
  --chain-id 137 \
  --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
  --webhook-url https://your-server.com/webhook
```

### Webhook Server Example (Node.js + Express)

```javascript
const express = require('express');
const app = express();

app.use(express.json());

app.post('/webhook', (req, res) => {
  const event = req.body;
  
  console.log('Received event:', event);
  console.log('  Chain:', event.chain_name);
  console.log('  Block:', event.block_number);
  console.log('  Transaction:', event.transaction_hash);
  
  // Process the event
  if (event.chain_id === 137) {
    // Polygon event
    handlePolygonEvent(event);
  }
  
  // Acknowledge receipt
  res.status(200).send('OK');
});

function handlePolygonEvent(event) {
  // Your custom logic
  // - Send notification
  // - Execute trade
  // - Update database
}

app.listen(3000, () => {
  console.log('Webhook server listening on port 3000');
});
```

### Webhook Server Example (Python + Flask)

```python
from flask import Flask, request, jsonify

app = Flask(__name__)

@app.route('/webhook', methods=['POST'])
def webhook():
    event = request.json
    
    print(f"Received event at block {event['block_number']}")
    print(f"  Transaction: {event['transaction_hash']}")
    
    # Process the event
    process_event(event)
    
    return jsonify({'status': 'success'}), 200

def process_event(event):
    # Your custom logic here
    if event['chain_id'] == 137:
        # Polygon event
        handle_polygon_event(event)
    elif event['chain_id'] == 1:
        # Ethereum event
        handle_ethereum_event(event)

if __name__ == '__main__':
    app.run(port=3000)
```

## Building on Top

### Use Case 1: Trading Bot

```python
#!/usr/bin/env python3
import subprocess
import json
import requests

def execute_trade(event):
    """Execute a trade when specific event is detected"""
    print(f"Executing trade based on event at block {event['block_number']}")
    
    # Your trading logic
    # api.place_order(...)

# Start listener
process = subprocess.Popen([
    'cargo', 'run', '--release', '--',
    '--chain-id', '137',
    '--contract', '0xUniswapPoolAddress',
    '--output-format', 'json',
    '--event', 'Swap(address,uint256,uint256,uint256,uint256,address)'
], stdout=subprocess.PIPE, text=True)

for line in process.stdout:
    try:
        event = json.loads(line)
        
        # Analyze swap data
        if should_trade(event):
            execute_trade(event)
            
    except json.JSONDecodeError:
        continue
```

### Use Case 2: Discord/Telegram Notifications

```python
#!/usr/bin/env python3
import subprocess
import json
import requests

DISCORD_WEBHOOK = "https://discord.com/api/webhooks/YOUR_WEBHOOK"

def send_discord(event):
    """Send Discord notification"""
    message = {
        "content": f"🔔 New Event Detected!",
        "embeds": [{
            "title": "Smart Contract Event",
            "fields": [
                {"name": "Chain", "value": event['chain_name'], "inline": True},
                {"name": "Block", "value": str(event['block_number']), "inline": True},
                {"name": "Transaction", "value": event['transaction_hash'][:20] + "..."},
                {"name": "Contract", "value": event['contract_address'][:20] + "..."}
            ],
            "color": 3447003
        }]
    }
    
    requests.post(DISCORD_WEBHOOK, json=message)

# Start listener
process = subprocess.Popen([
    'cargo', 'run', '--release', '--',
    '--chain-id', '1',
    '--contract', '0xYourNFTContract',
    '--output-format', 'json',
    '--event', 'Transfer(address,address,uint256)'
], stdout=subprocess.PIPE, text=True)

for line in process.stdout:
    try:
        event = json.loads(line)
        send_discord(event)
    except json.JSONDecodeError:
        continue
```

### Use Case 3: Database Storage

```python
#!/usr/bin/env python3
import subprocess
import json
import sqlite3

# Setup database
conn = sqlite3.connect('events.db')
c = conn.cursor()
c.execute('''
    CREATE TABLE IF NOT EXISTS events (
        timestamp TEXT,
        chain_id INTEGER,
        block_number INTEGER,
        transaction_hash TEXT,
        contract_address TEXT,
        topics TEXT,
        data TEXT
    )
''')
conn.commit()

# Start listener
process = subprocess.Popen([
    'cargo', 'run', '--release', '--',
    '--chain-id', '137',
    '--contract', '0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359',
    '--output-format', 'json'
], stdout=subprocess.PIPE, text=True)

# Store events
for line in process.stdout:
    try:
        event = json.loads(line)
        
        c.execute('''
            INSERT INTO events VALUES (?, ?, ?, ?, ?, ?, ?)
        ''', (
            event['timestamp'],
            event['chain_id'],
            event['block_number'],
            event['transaction_hash'],
            event['contract_address'],
            json.dumps(event['topics']),
            event['data']
        ))
        conn.commit()
        
        print(f"Stored event from block {event['block_number']}")
        
    except json.JSONDecodeError:
        continue
```

### Use Case 4: Multi-Chain Aggregator

```bash
#!/bin/bash

# Monitor multiple chains simultaneously
cargo run --release -- --chain-id 1 --contract 0xETH_CONTRACT --output-format json > eth_events.jsonl &
cargo run --release -- --chain-id 137 --contract 0xPOLYGON_CONTRACT --output-format json > polygon_events.jsonl &
cargo run --release -- --chain-id 42161 --contract 0xARB_CONTRACT --output-format json > arb_events.jsonl &

# Aggregate them
tail -f eth_events.jsonl polygon_events.jsonl arb_events.jsonl | \
while read -r line; do
  echo "$line" | jq '{chain: .chain_name, block: .block_number, tx: .transaction_hash}'
done
```

## Integration Examples

### Example: Automated Market Maker Monitor

```python
#!/usr/bin/env python3
"""
Monitor Uniswap swaps and calculate price impact
"""
import subprocess
import json

process = subprocess.Popen([
    'cargo', 'run', '--release', '--',
    '--chain-id', '1',
    '--contract', '0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc',  # USDC/ETH pair
    '--output-format', 'json',
    '--event', 'Swap(address,uint256,uint256,uint256,uint256,address)'
], stdout=subprocess.PIPE, text=True)

for line in process.stdout:
    try:
        event = json.loads(line)
        
        # Parse swap data from topics and data
        print(f"Swap detected at block {event['block_number']}")
        print(f"  Transaction: {event['transaction_hash']}")
        
        # Calculate price impact, check for opportunities
        # analyze_swap(event)
        
    except json.JSONDecodeError:
        continue
```

### Example: NFT Sales Tracker

```python
#!/usr/bin/env python3
"""
Track NFT sales and send alerts for high-value transactions
"""
import subprocess
import json
import requests

PRICE_THRESHOLD = 10_000_000_000_000_000_000  # 10 ETH in wei

process = subprocess.Popen([
    'cargo', 'run', '--release', '--',
    '--chain-id', '1',
    '--contract', '0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D',  # BAYC
    '--output-format', 'json',
    '--event', 'Transfer(address,address,uint256)'
], stdout=subprocess.PIPE, text=True)

for line in process.stdout:
    try:
        event = json.loads(line)
        
        print(f"NFT Transfer at block {event['block_number']}")
        
        # Check if it's a sale (could query marketplace contracts)
        # if is_sale and price > PRICE_THRESHOLD:
        #     send_alert(event)
        
    except json.JSONDecodeError:
        continue
```

## Tips

1. **Use JSON format** for any programmatic integration
2. **Buffer handling**: Events stream in real-time, handle line-by-line
3. **Error handling**: Implement retries for webhook failures
4. **Multiple outputs**: Combine stdout, file, and webhook simultaneously
5. **Filtering**: Use `--event` to reduce noise and processing overhead
6. **Performance**: Higher `--poll-interval` reduces load but increases latency

## Need Help?

Check the main [README.md](README.md) for more information or open an issue on GitHub.
