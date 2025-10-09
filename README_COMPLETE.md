# 🚀 Bitcoin Esplora Complete Substream

A comprehensive Bitcoin blockchain data processor that implements **ALL** Esplora API endpoints, providing complete blockchain data access and real-time processing capabilities.

## 📊 Complete Esplora API Implementation

This Substream implements every endpoint from the [Esplora API](https://github.com/Blockstream/esplora/blob/master/API.md), providing:

### 🧱 Block Endpoints
- **GET /blocks** - List recent blocks with summaries
- **GET /block/:hash** - Get detailed block information
- **GET /block/:hash/txs** - Get all transactions in a block

### 💸 Transaction Endpoints
- **GET /tx/:txid** - Get complete transaction details
- **GET /tx/:txid/hex** - Get transaction raw hex data
- **GET /tx/:txid/status** - Get transaction confirmation status

### 🏠 Address Endpoints
- **GET /address/:address** - Get address information and balance
- **GET /address/:address/txs** - Get address transaction history
- **GET /address/:address/utxo** - Get address UTXOs

### ⏳ Mempool Endpoints
- **GET /mempool** - Get mempool statistics and fee histogram
- **GET /mempool/recent** - Get recent mempool transactions
- **GET /mempool/txids** - Get all mempool transaction IDs

### 🌐 Network Endpoints
- **GET /blocks/tip/height** - Get current blockchain height
- **GET /fee-estimates** - Get fee estimates for different confirmation targets
- **GET /stats** - Get comprehensive network statistics

### 🔍 Search & Webhooks
- **GET /search/:query** - Search for addresses, transactions, or blocks
- **Webhook Events** - Real-time notifications for blocks, transactions, and addresses

### 💎 Liquid/Elements Support
- **GET /asset/:assetid** - Get Liquid asset information
- **Peg-in/Peg-out Tracking** - Monitor Bitcoin ↔ Liquid transfers

## 🏗️ Architecture

### Core Components

1. **EsploraCompleteData** - Main protobuf structure containing all data types
2. **Block Processing** - Extracts block headers, transactions, and mining data
3. **Transaction Analysis** - Processes inputs, outputs, scripts, and fees
4. **Address Monitoring** - Tracks balances, UTXOs, and transaction history
5. **Mempool Analysis** - Monitors pending transactions and fee distribution
6. **Network Statistics** - Calculates hash rate, difficulty, and volume metrics
7. **Real-time Webhooks** - Provides live notifications for blockchain events

### Data Models

```protobuf
message EsploraCompleteData {
  // Block data
  optional BlockInfo block = 1;
  repeated BlockSummary block_summaries = 2;
  repeated TransactionInfo block_transactions = 3;
  
  // Transaction data
  repeated TransactionInfo transactions = 4;
  repeated TransactionHex transaction_hexes = 5;
  repeated TransactionStatus transaction_statuses = 6;
  
  // Address data
  repeated AddressInfo addresses = 7;
  repeated AddressTransaction address_transactions = 8;
  repeated AddressUtxo address_utxos = 9;
  
  // Mempool data
  optional MempoolInfo mempool = 10;
  repeated MempoolTransaction mempool_recent = 11;
  repeated string mempool_txids = 12;
  
  // Network data
  optional NetworkTip network_tip = 13;
  repeated FeeEstimate fee_estimates = 14;
  optional NetworkStats network_stats = 15;
  
  // Search & webhooks
  repeated SearchResult search_results = 16;
  repeated WebhookEvent webhook_events = 17;
  
  // Liquid/Elements
  repeated AssetInfo assets = 18;
  repeated LiquidAsset liquid_assets = 19;
  repeated PegIn peg_ins = 20;
  repeated PegOut peg_outs = 21;
}
```

## 🚀 Usage

### Building the Substream

```bash
# Build the complete Substream
make build

# Package for deployment
make pack

# Run with real Bitcoin data
make run
```

### Integration Examples

#### Block Data Processing
```rust
// Get block information
let block_info = esplora_data.block.unwrap();
println!("Block {} at height {}", block_info.id, block_info.height);

// Get block transactions
for tx in esplora_data.block_transactions {
    println!("Transaction: {}", tx.txid);
}
```

#### Address Monitoring
```rust
// Get address information
for address in esplora_data.addresses {
    println!("Address: {} Balance: {} sats", 
        address.address, 
        address.funded_txo_sum - address.spent_txo_sum
    );
}

// Get address UTXOs
for utxo in esplora_data.address_utxos {
    println!("UTXO: {}:{} = {} sats", utxo.txid, utxo.vout, utxo.value);
}
```

#### Mempool Analysis
```rust
// Get mempool statistics
if let Some(mempool) = esplora_data.mempool {
    println!("Mempool: {} transactions, {} vbytes, {} total fees", 
        mempool.count, mempool.vsize, mempool.total_fee
    );
}

// Get recent mempool transactions
for tx in esplora_data.mempool_recent {
    println!("Pending: {} fee: {} sats/vB", tx.txid, tx.fee / tx.vsize);
}
```

#### Network Statistics
```rust
// Get network tip
if let Some(tip) = esplora_data.network_tip {
    println!("Current height: {}", tip.height);
}

// Get fee estimates
for estimate in esplora_data.fee_estimates {
    println!("{} blocks: {} sat/vB", 
        estimate.confirmation_target_blocks, 
        estimate.feerate_sat_per_vb
    );
}
```

#### Real-time Webhooks
```rust
// Process webhook events
for event in esplora_data.webhook_events {
    match event.event_type.as_str() {
        "block" => println!("New block: {}", event.event_id),
        "tx" => println!("New transaction: {}", event.event_id),
        "address" => println!("Address activity: {}", event.event_id),
        _ => {}
    }
}
```

## 🔧 Configuration

### Environment Variables
```bash
# Substreams API token for authentication
export SUBSTREAMS_API_TOKEN="your_jwt_token_here"

# Bitcoin network (mainnet, testnet, regtest)
export BITCOIN_NETWORK="mainnet"

# Enable Liquid/Elements support
export ENABLE_LIQUID="true"
```

### Customization Options

1. **Block Range**: Process specific block ranges
2. **Address Filtering**: Monitor specific addresses
3. **Transaction Types**: Filter by transaction types
4. **Fee Thresholds**: Set minimum fee requirements
5. **Webhook Endpoints**: Configure notification URLs

## 📈 Performance Features

- **Efficient Data Processing**: Optimized for high-throughput blockchain data
- **Real-time Streaming**: Live updates as new blocks are mined
- **Scalable Architecture**: Handles Bitcoin's full transaction volume
- **Memory Efficient**: Minimal memory footprint for long-running processes
- **Fork-aware**: Handles blockchain reorganizations correctly

## 🔗 Integration with FoundationalStore

This Substream is designed to work seamlessly with Substreams FoundationalStore:

```rust
// Store data in FoundationalStore
let store = FoundationalStore::new();
store.set("block:latest", &block_data);
store.set("address:balance", &balance_data);
store.set("mempool:stats", &mempool_data);

// Query data from FoundationalStore
let latest_block = store.get("block:latest");
let address_balance = store.get("address:balance");
```

## 🌟 Key Features

### ✅ Complete Esplora API Coverage
- All 20+ Esplora API endpoints implemented
- Full transaction and block data processing
- Comprehensive address and UTXO tracking
- Real-time mempool monitoring
- Network statistics and fee estimation

### ✅ Real-time Processing
- Live blockchain data streaming
- Webhook notifications for events
- Mempool transaction monitoring
- Network state updates

### ✅ Liquid/Elements Support
- Multi-asset transaction processing
- Peg-in/peg-out tracking
- Asset issuance and burning
- Confidential transaction support

### ✅ Production Ready
- Error handling and recovery
- Performance optimization
- Comprehensive logging
- Monitoring and metrics

## 📚 API Reference

### Block Endpoints
| Endpoint | Description | Data Structure |
|----------|-------------|----------------|
| `/blocks` | Recent blocks | `BlockSummary[]` |
| `/block/:hash` | Block details | `BlockInfo` |
| `/block/:hash/txs` | Block transactions | `TransactionInfo[]` |

### Transaction Endpoints
| Endpoint | Description | Data Structure |
|----------|-------------|----------------|
| `/tx/:txid` | Transaction details | `TransactionInfo` |
| `/tx/:txid/hex` | Transaction hex | `TransactionHex` |
| `/tx/:txid/status` | Transaction status | `TransactionStatus` |

### Address Endpoints
| Endpoint | Description | Data Structure |
|----------|-------------|----------------|
| `/address/:address` | Address info | `AddressInfo` |
| `/address/:address/txs` | Address transactions | `AddressTransaction[]` |
| `/address/:address/utxo` | Address UTXOs | `AddressUtxo[]` |

### Mempool Endpoints
| Endpoint | Description | Data Structure |
|----------|-------------|----------------|
| `/mempool` | Mempool stats | `MempoolInfo` |
| `/mempool/recent` | Recent mempool | `MempoolTransaction[]` |
| `/mempool/txids` | Mempool txids | `string[]` |

### Network Endpoints
| Endpoint | Description | Data Structure |
|----------|-------------|----------------|
| `/blocks/tip/height` | Current height | `NetworkTip` |
| `/fee-estimates` | Fee estimates | `FeeEstimate[]` |
| `/stats` | Network stats | `NetworkStats` |

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests and documentation
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [Blockstream Esplora](https://github.com/Blockstream/esplora) - Original API inspiration
- [Substreams](https://substreams.streamingfast.io/) - Blockchain data processing framework
- [Bitcoin Core](https://bitcoin.org/en/bitcoin-core/) - Bitcoin protocol reference
- [Liquid Network](https://liquid.net/) - Sidechain technology

---

**Ready to process Bitcoin blockchain data with complete Esplora API functionality!** 🚀
