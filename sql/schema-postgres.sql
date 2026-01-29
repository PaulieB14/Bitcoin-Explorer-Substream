-- Bitcoin Esplora Enhanced - PostgreSQL Schema
-- For use with substreams-sink-sql

-- Blocks table
CREATE TABLE IF NOT EXISTS blocks (
    id VARCHAR(64) PRIMARY KEY,
    height INTEGER NOT NULL,
    version INTEGER,
    timestamp BIGINT NOT NULL,
    bits INTEGER,
    nonce BIGINT,
    tx_count INTEGER,
    size BIGINT,
    weight BIGINT,
    merkle_root VARCHAR(64),
    previous_hash VARCHAR(64),
    difficulty DOUBLE PRECISION,
    mediantime VARCHAR(32)
);

CREATE INDEX IF NOT EXISTS idx_blocks_height ON blocks(height);
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks(timestamp);

-- Transactions table
CREATE TABLE IF NOT EXISTS transactions (
    txid VARCHAR(64) PRIMARY KEY,
    block_hash VARCHAR(64) REFERENCES blocks(id),
    block_height INTEGER,
    version INTEGER,
    locktime BIGINT,
    size BIGINT,
    weight BIGINT,
    fee BIGINT,
    block_time BIGINT
);

CREATE INDEX IF NOT EXISTS idx_tx_block_hash ON transactions(block_hash);
CREATE INDEX IF NOT EXISTS idx_tx_block_height ON transactions(block_height);
CREATE INDEX IF NOT EXISTS idx_tx_fee ON transactions(fee);

-- Transaction inputs
CREATE TABLE IF NOT EXISTS tx_inputs (
    id SERIAL PRIMARY KEY,
    txid VARCHAR(64) REFERENCES transactions(txid),
    input_index INTEGER,
    prev_txid VARCHAR(64),
    prev_vout INTEGER,
    is_coinbase BOOLEAN DEFAULT FALSE,
    scriptsig TEXT,
    scriptsig_asm TEXT,
    sequence BIGINT,
    witness TEXT[]
);

CREATE INDEX IF NOT EXISTS idx_inputs_txid ON tx_inputs(txid);
CREATE INDEX IF NOT EXISTS idx_inputs_prev_txid ON tx_inputs(prev_txid);

-- Transaction outputs
CREATE TABLE IF NOT EXISTS tx_outputs (
    id SERIAL PRIMARY KEY,
    txid VARCHAR(64) REFERENCES transactions(txid),
    output_index INTEGER,
    value BIGINT,
    scriptpubkey TEXT,
    scriptpubkey_asm TEXT,
    scriptpubkey_type VARCHAR(50),
    scriptpubkey_address VARCHAR(100)
);

CREATE INDEX IF NOT EXISTS idx_outputs_txid ON tx_outputs(txid);
CREATE INDEX IF NOT EXISTS idx_outputs_address ON tx_outputs(scriptpubkey_address);
CREATE INDEX IF NOT EXISTS idx_outputs_value ON tx_outputs(value);

-- Addresses summary
CREATE TABLE IF NOT EXISTS addresses (
    address VARCHAR(100) PRIMARY KEY,
    address_type VARCHAR(50),
    funded_txo_count BIGINT DEFAULT 0,
    funded_txo_sum BIGINT DEFAULT 0,
    spent_txo_count BIGINT DEFAULT 0,
    spent_txo_sum BIGINT DEFAULT 0,
    first_seen_block INTEGER,
    last_seen_block INTEGER
);

CREATE INDEX IF NOT EXISTS idx_addresses_type ON addresses(address_type);
CREATE INDEX IF NOT EXISTS idx_addresses_funded_sum ON addresses(funded_txo_sum);

-- Network stats per block
CREATE TABLE IF NOT EXISTS network_stats (
    block_height INTEGER PRIMARY KEY,
    block_hash VARCHAR(64),
    total_tx_count BIGINT,
    total_fees BIGINT,
    active_addresses INTEGER,
    avg_fee_rate DOUBLE PRECISION,
    fee_estimate_1 DOUBLE PRECISION,
    fee_estimate_6 DOUBLE PRECISION,
    fee_estimate_144 DOUBLE PRECISION
);

CREATE INDEX IF NOT EXISTS idx_stats_hash ON network_stats(block_hash);

-- Cursors for sink tracking
CREATE TABLE IF NOT EXISTS cursors (
    id VARCHAR(255) PRIMARY KEY,
    cursor TEXT,
    block_num BIGINT,
    block_id VARCHAR(64)
);
