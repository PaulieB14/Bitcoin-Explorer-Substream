# Bitcoin Esplora Complete Substream

A comprehensive Bitcoin blockchain data processor that mimics the complete functionality of the Esplora API. This Substream processes Bitcoin blocks and extracts all the data that Esplora provides through its REST API.

## Features

### Complete Esplora API Coverage

This Substream captures and processes all data types available through the Esplora API:

#### 📦 **Block Data**
- Block headers (hash, height, timestamp, difficulty)
- Transaction count and block size
- Merkle root and previous block hash
- Mining pool information
- Block validation status

#### 💸 **Transaction Data**
- Complete transaction details (txid, version, locktime, size, weight, fee)
- Full input/output data with scripts
- Witness data for SegWit transactions
- Confirmation status and block information
- Merkle proofs and spending status

#### 🏠 **Address Data**
- Address balances and UTXO information
- Transaction history for addresses
- Mempool transactions for addresses
- Address type analysis (P2PKH, P2WPKH, P2SH, etc.)
- Privacy metrics and clustering

#### ⏳ **Mempool Data**
- Pending transactions and their details
- Fee distribution and mempool statistics
- Transaction priority and confirmation estimates
- Mempool size and congestion metrics

#### 📊 **Network Statistics**
- Fee estimates for different confirmation targets
- Network hash rate and difficulty
- Transaction volume and throughput
- Address activity patterns

#### 🪙 **Asset Data (Liquid/Elements)**
- Asset information and statistics
- Peg-in/peg-out transactions
- Issuance and burn tracking
- Asset registry data

## Data Storage

The Substream stores processed data in FoundationalStore for efficient querying:

### Storage Keys
- `block:*` - Block information
- `tx:*` - Transaction data
- `address:*` - Address information
- `mempool:*` - Mempool data
- `network:*` - Network statistics
- `fee_estimate:*` - Fee estimates

### Example Queries
```rust
// Get block information
let block_hash = store.get_last("block:hash");
let block_height = store.get_last("block:height");

// Get transaction data
let tx_fee = store.get_last("tx:0:fee");
let tx_size = store.get_last("tx:0:size");

// Get address information
let address_balance = store.get_last("address:0:balance");
let address_type = store.get_last("address:0:type");

// Get mempool data
let mempool_count = store.get_last("mempool:count");
let mempool_vsize = store.get_last("mempool:vsize");

// Get network statistics
let total_tx_count = store.get_last("network:total_tx_count");
let avg_fee_rate = store.get_last("network:avg_fee_rate");

// Get fee estimates
let fee_1_block = store.get_last("fee_estimate:1");
let fee_6_blocks = store.get_last("fee_estimate:6");
```

## Usage

### Building the Substream
```bash
# Build the project
cargo build --release --target wasm32-unknown-unknown

# Package the Substream
substreams pack
```

### Running the Substream
```bash
# Run with console output
substreams run map_esplora_data --start-block 800000

# Run with GUI
substreams gui map_esplora_data --start-block 800000

# Get package information
substreams info
```

### Integration with FoundationalStore
```bash
# Set up FoundationalStore endpoint
export SUBSTREAMS_API_TOKEN=your_token_here

# Run with FoundationalStore integration
substreams run map_esplora_data --start-block 800000 --store-module store_esplora_data
```

## Data Models

### EsploraData
The main data structure containing all processed Bitcoin data:
- `BlockInfo` - Complete block information
- `TransactionInfo[]` - All transaction data
- `AddressInfo[]` - Address information and statistics
- `MempoolInfo` - Mempool data and statistics
- `NetworkStats` - Network statistics
- `FeeEstimate[]` - Fee estimates for different targets
- `AssetInfo[]` - Asset information (Liquid/Elements)

### Key Features
- **Complete API Coverage** - Mimics all Esplora API endpoints
- **Real-time Processing** - Processes blocks as they're mined
- **Efficient Storage** - Uses FoundationalStore for fast queries
- **Scalable Architecture** - Handles Bitcoin's growing transaction volume
- **Privacy Analysis** - Includes privacy metrics and address clustering
- **Fee Optimization** - Provides fee estimates and mempool analysis

## Comparison with Esplora API

| Esplora API Endpoint | Substream Data | Storage Key |
|---------------------|----------------|-------------|
| `GET /block/:hash` | `BlockInfo` | `block:*` |
| `GET /tx/:txid` | `TransactionInfo` | `tx:*` |
| `GET /address/:address` | `AddressInfo` | `address:*` |
| `GET /mempool` | `MempoolInfo` | `mempool:*` |
| `GET /fee-estimates` | `FeeEstimate[]` | `fee_estimate:*` |
| `GET /blocks/tip/hash` | `NetworkStats.tip_hash` | `network:tip_hash` |

## Use Cases

### Personal Bitcoin Dashboard
- Portfolio tracking across all addresses
- Transaction monitoring and history
- Fee optimization recommendations
- Privacy analysis for transactions

### Bitcoin Network Analytics
- Network health monitoring
- Transaction volume analysis
- Fee market trends
- Mining pool statistics

### DeFi Integration
- Cross-chain bridge monitoring
- Liquid Network activity
- Asset issuance tracking
- Confidential transaction analysis

## Development

### Adding New Features
1. Define new protobuf messages in `proto/esplora.proto`
2. Implement processing logic in `src/main.rs`
3. Add storage keys in the store handler
4. Update documentation

### Testing
```bash
# Run with test data
substreams run map_esplora_data --start-block 800000 --stop-block 800010

# Validate output
substreams run map_esplora_data --start-block 800000 --output json | jq .
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Support

For questions and support, please open an issue on GitHub.
