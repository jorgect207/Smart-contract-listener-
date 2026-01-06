#!/usr/bin/env node
/**
 * Example: Webhook server to receive events
 * 
 * Run this server, then start the listener with:
 * cargo run --release -- \
 *   --chain-id 137 \
 *   --contract 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 \
 *   --webhook-url http://localhost:3000/webhook
 */

const express = require('express');
const app = express();
const PORT = 3000;

// Middleware to parse JSON
app.use(express.json());

// Statistics
let eventCount = 0;
let lastEvent = null;

// Webhook endpoint
app.post('/webhook', (req, res) => {
  const event = req.body;
  
  eventCount++;
  lastEvent = event;
  
  console.log('\n' + '='.repeat(60));
  console.log('🔔 Event Received!');
  console.log('='.repeat(60));
  console.log(`Chain: ${event.chain_name} (ID: ${event.chain_id})`);
  console.log(`Block: ${event.block_number}`);
  console.log(`Transaction: ${event.transaction_hash}`);
  console.log(`Contract: ${event.contract_address}`);
  console.log(`Time: ${event.timestamp}`);
  console.log(`Total events: ${eventCount}`);
  
  // Your custom logic here
  processEvent(event);
  
  // Respond to the listener
  res.status(200).json({ 
    status: 'success', 
    message: 'Event processed',
    event_count: eventCount
  });
});

// Status endpoint
app.get('/status', (req, res) => {
  res.json({
    status: 'running',
    event_count: eventCount,
    last_event: lastEvent
  });
});

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'healthy' });
});

function processEvent(event) {
  // Implement your custom logic here
  // Examples:
  // - Send notification to Discord/Slack
  // - Store in database
  // - Execute a trade
  // - Update analytics
  
  if (event.block_number % 100 === 0) {
    console.log('→ Milestone block detected!');
  }
  
  // Example: Send to Discord (uncomment and add your webhook)
  // sendToDiscord(event);
}

function sendToDiscord(event) {
  const fetch = require('node-fetch');
  const DISCORD_WEBHOOK = 'YOUR_DISCORD_WEBHOOK_URL';
  
  const message = {
    content: '🔔 New Blockchain Event',
    embeds: [{
      title: 'Event Detected',
      fields: [
        { name: 'Chain', value: event.chain_name, inline: true },
        { name: 'Block', value: String(event.block_number), inline: true },
        { name: 'Transaction', value: event.transaction_hash.substring(0, 20) + '...' }
      ],
      color: 3447003,
      timestamp: event.timestamp
    }]
  };
  
  fetch(DISCORD_WEBHOOK, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(message)
  }).catch(console.error);
}

// Start server
app.listen(PORT, () => {
  console.log('🚀 Webhook Server Started');
  console.log('='.repeat(60));
  console.log(`Listening on port ${PORT}`);
  console.log(`Webhook endpoint: http://localhost:${PORT}/webhook`);
  console.log(`Status endpoint: http://localhost:${PORT}/status`);
  console.log(`Health check: http://localhost:${PORT}/health`);
  console.log('='.repeat(60));
  console.log('\nWaiting for events...\n');
});
