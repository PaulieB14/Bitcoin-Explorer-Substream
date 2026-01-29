# Bitcoin Esplora Enhanced Substream

Enhanced Bitcoin blockchain data processor providing [Esplora API](https://github.com/Blockstream/esplora/blob/master/API.md) compatible output using [Substreams](https://substreams.streamingfast.io/).

## Features

- **Block Metadata**: Hash, height, timestamp, difficulty, merkle root, previous hash
- **Transaction Details**: Full transaction data with inputs, outputs, and witness data
- **Address Tracking**: UTXO tracking per address with funded/spent counts
- **Network Statistics**: Fee estimates, active addresses, total fees per block
- **SQL Sink Ready**: PostgreSQL and ClickHouse schemas included

## Modules

| Module | Output | Description |
|--------|--------|-------------|
| `map_block_esplora` | `BlockEsplora` | Block metadata in Esplora format |
| `map_transactions_esplora` | `TransactionsEsplora` | All transactions with inputs/outputs/witness |
| `map_addresses_esplora` | `AddressesEsplora` | Address analysis and UTXO tracking |
| `map_network_stats` | `NetworkStats` | Network statistics and fee estimates |

## Quick Start

### Prerequisites

```bash
# Install Substreams CLI
brew install streamingfast/tap/substreams

# Authenticate
substreams auth
```

### Run

```bash
# Stream block data
substreams run -e btc.substreams.pinax.network:443 \
  https://github.com/PaulieB14/Bitcoin-Explorer-Substream/releases/download/v0.2.0/bitcoin-esplora-enhanced-v0.2.0.spkg \
  map_block_esplora -s 800000 -t +10

# Stream transactions
substreams run -e btc.substreams.pinax.network:443 \
  https://github.com/PaulieB14/Bitcoin-Explorer-Substream/releases/download/v0.2.0/bitcoin-esplora-enhanced-v0.2.0.spkg \
  map_transactions_esplora -s 800000 -t +10
```

### Build from Source

```bash
git clone https://github.com/PaulieB14/Bitcoin-Explorer-Substream.git
cd Bitcoin-Explorer-Substream
make build
make pack
```

## SQL Sink Usage

### PostgreSQL

```bash
# Setup database
substreams-sink-sql setup \
  "psql://user:pass@localhost:5432/bitcoin" \
  ./sql/schema-postgres.sql

# Run sink
substreams-sink-sql run \
  "psql://user:pass@localhost:5432/bitcoin" \
  bitcoin-esplora-enhanced-v0.2.0.spkg \
  map_block_esplora
```

### ClickHouse

```bash
# Setup database
substreams-sink-sql setup \
  "clickhouse://user:pass@localhost:9000/bitcoin" \
  ./sql/schema-clickhouse.sql

# Run sink
substreams-sink-sql run \
  "clickhouse://user:pass@localhost:9000/bitcoin" \
  bitcoin-esplora-enhanced-v0.2.0.spkg \
  map_block_esplora
```

## Output Examples

### Block Data

```json
{
  "id": "00000000000000000002a7c4c1e48d76c5a37902165a270156b7a8d72728a054",
  "height": 800000,
  "version": 536870912,
  "timestamp": 1690168629,
  "tx_count": 3721,
  "size": 1621583,
  "weight": 3993561,
  "merkle_root": "...",
  "previous_hash": "...",
  "difficulty": 53911173001054.59
}
```

### Transaction Data

```json
{
  "txid": "abc123...",
  "version": 2,
  "locktime": 0,
  "size": 225,
  "weight": 573,
  "fee": 4500,
  "inputs": [...],
  "outputs": [...],
  "witness": ["304402...", "0279be..."],
  "status": {
    "confirmed": true,
    "block_height": 800000
  }
}
```

## Limitations

- **Mempool Data**: Substreams processes finalized blocks only. Real-time mempool data requires a separate Bitcoin node connection.
- **Fee Calculation**: Accurate fees require UTXO lookup. Current implementation provides estimates based on transaction characteristics.

## Development

```bash
# Check code
make check

# Format code
make fmt

# Run tests
make test-unit

# Clean build
make clean
```

## License

Apache 2.0

## Links

- [Substreams Documentation](https://substreams.streamingfast.io/)
- [Esplora API Reference](https://github.com/Blockstream/esplora/blob/master/API.md)
- [Bitcoin Protocol](https://en.bitcoin.it/wiki/Protocol_documentation)
