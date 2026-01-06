#!/usr/bin/env python3
"""
Example: Process blockchain events in Python

This script shows how to:
- Consume JSON output from the listener
- Process events in real-time
- Build custom logic on top of events
"""

import subprocess
import json
import sys
from datetime import datetime

def process_event(event):
    """Process a single event - customize this for your use case"""
    
    print(f"\n{'='*60}")
    print(f"🔔 Event Detected!")
    print(f"{'='*60}")
    print(f"Chain: {event['chain_name']} (ID: {event['chain_id']})")
    print(f"Block: {event['block_number']}")
    print(f"Time: {event['timestamp']}")
    print(f"Transaction: {event['transaction_hash']}")
    print(f"Contract: {event['contract_address']}")
    print(f"Topics: {len(event['topics'])}")
    
    # Your custom logic here
    # Examples:
    # - Send notification
    # - Execute a trade
    # - Update a database
    # - Trigger another workflow
    
    if event['block_number'] % 100 == 0:
        print("→ Milestone block!")
    
    return True

def main():
    """Main function to run the listener and process events"""
    
    # Configuration
    CHAIN_ID = '137'  # Polygon
    CONTRACT = '0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359'  # USDC on Polygon
    EVENT = 'Transfer(address,address,uint256)'
    
    print("Starting Smart Contract Event Listener Integration")
    print(f"Chain ID: {CHAIN_ID}")
    print(f"Contract: {CONTRACT}")
    print(f"Event: {EVENT}")
    print("\nWaiting for events...\n")
    
    # Start the Rust listener as a subprocess
    try:
        process = subprocess.Popen([
            'cargo', 'run', '--release', '--',
            '--chain-id', CHAIN_ID,
            '--contract', CONTRACT,
            '--output-format', 'json',
            '--event', EVENT
        ], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, bufsize=1)
        
        # Process events line by line
        for line in process.stdout:
            line = line.strip()
            if not line:
                continue
                
            try:
                # Parse JSON event
                event = json.loads(line)
                
                # Process the event
                process_event(event)
                
            except json.JSONDecodeError as e:
                # Skip non-JSON lines (like startup messages)
                continue
            except Exception as e:
                print(f"Error processing event: {e}", file=sys.stderr)
                continue
    
    except KeyboardInterrupt:
        print("\n\n👋 Shutting down gracefully...")
        process.terminate()
        sys.exit(0)
    except Exception as e:
        print(f"Fatal error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()
