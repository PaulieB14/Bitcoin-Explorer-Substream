# Bitcoin Esplora Complete Substream Makefile

.PHONY: build pack run gui info clean test

# Build the Substream
build:
	@echo "🔨 Building Bitcoin Esplora Complete Substream..."
	cargo build --release --target wasm32-unknown-unknown
	@echo "✅ Build complete!"

# Package the Substream
pack: build
	@echo "📦 Packaging Substream..."
	substreams pack
	@echo "✅ Package created: substreams.spkg"

# Run the Substream with console output
run: pack
	@echo "🚀 Running Bitcoin Esplora Complete Substream..."
	substreams run map_esplora_data --start-block 800000

# Run with GUI
gui: pack
	@echo "🖥️  Starting Substreams GUI..."
	substreams gui map_esplora_data --start-block 800000

# Get package information
info: pack
	@echo "📋 Package Information:"
	substreams info

# Run with specific block range
test: pack
	@echo "🧪 Testing with block range 800000-800010..."
	substreams run map_esplora_data --start-block 800000 --stop-block 800010

# Run with JSON output
json: pack
	@echo "📄 Running with JSON output..."
	substreams run map_esplora_data --start-block 800000 --output json

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -f substreams.spkg
	@echo "✅ Clean complete!"

# Install dependencies
install:
	@echo "📥 Installing dependencies..."
	cargo install substreams
	@echo "✅ Dependencies installed!"

# Check code
check:
	@echo "🔍 Checking code..."
	cargo check
	@echo "✅ Code check complete!"

# Format code
fmt:
	@echo "🎨 Formatting code..."
	cargo fmt
	@echo "✅ Code formatted!"

# Run tests
test-unit:
	@echo "🧪 Running unit tests..."
	cargo test
	@echo "✅ Tests complete!"

# Help
help:
	@echo "Bitcoin Esplora Complete Substream - Available Commands:"
	@echo ""
	@echo "  build     - Build the Substream"
	@echo "  pack      - Package the Substream"
	@echo "  run       - Run with console output"
	@echo "  gui       - Run with GUI"
	@echo "  info      - Show package information"
	@echo "  test      - Test with block range"
	@echo "  json      - Run with JSON output"
	@echo "  clean     - Clean build artifacts"
	@echo "  install   - Install dependencies"
	@echo "  check     - Check code"
	@echo "  fmt       - Format code"
	@echo "  test-unit - Run unit tests"
	@echo "  help      - Show this help"
