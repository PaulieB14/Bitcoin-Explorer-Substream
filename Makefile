# Bitcoin Esplora Enhanced Substream Makefile

.PHONY: build pack run gui info clean test check fmt help

# Build the Substream
build:
	cargo build --release --target wasm32-unknown-unknown

# Package the Substream
pack: build
	substreams pack

# Run block data module
run-blocks: pack
	substreams run -e btc.substreams.pinax.network:443 map_block_esplora --start-block 800000 --stop-block +10

# Run transactions module
run-txs: pack
	substreams run -e btc.substreams.pinax.network:443 map_transactions_esplora --start-block 800000 --stop-block +10

# Run addresses module
run-addrs: pack
	substreams run -e btc.substreams.pinax.network:443 map_addresses_esplora --start-block 800000 --stop-block +10

# Run network stats module
run-stats: pack
	substreams run -e btc.substreams.pinax.network:443 map_network_stats --start-block 800000 --stop-block +10

# Run with GUI
gui: pack
	substreams gui -e btc.substreams.pinax.network:443 map_block_esplora --start-block 800000

# Get package information
info: pack
	substreams info

# Test with specific block range
test: pack
	substreams run -e btc.substreams.pinax.network:443 map_block_esplora --start-block 800000 --stop-block 800010

# Clean build artifacts
clean:
	cargo clean
	rm -f *.spkg

# Check code
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Run tests
test-unit:
	cargo test

# Help
help:
	@echo "Bitcoin Esplora Enhanced Substream v0.2.0"
	@echo ""
	@echo "Commands:"
	@echo "  build      - Build the Substream WASM"
	@echo "  pack       - Package the Substream (.spkg)"
	@echo "  run-blocks - Run block data module"
	@echo "  run-txs    - Run transactions module"
	@echo "  run-addrs  - Run addresses module"
	@echo "  run-stats  - Run network stats module"
	@echo "  gui        - Run with GUI"
	@echo "  info       - Show package information"
	@echo "  test       - Test with block range 800000-800010"
	@echo "  clean      - Clean build artifacts"
	@echo "  check      - Check code"
	@echo "  fmt        - Format code"
	@echo "  test-unit  - Run unit tests"
