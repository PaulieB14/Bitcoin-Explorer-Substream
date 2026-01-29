-- Bitcoin Esplora Enhanced - ClickHouse Schema
-- For use with substreams-sink-sql (ClickHouse)

-- Blocks table
CREATE TABLE IF NOT EXISTS blocks (
    id String,
    height UInt32,
    version UInt32,
    timestamp UInt64,
    bits UInt32,
    nonce UInt64,
    tx_count UInt32,
    size UInt64,
    weight UInt64,
    merkle_root String,
    previous_hash String,
    difficulty Float64,
    mediantime String
) ENGINE = ReplacingMergeTree()
ORDER BY (height, id)
PARTITION BY toYYYYMM(toDateTime(timestamp));

-- Transactions table
CREATE TABLE IF NOT EXISTS transactions (
    txid String,
    block_hash String,
    block_height UInt32,
    version UInt32,
    locktime UInt64,
    size UInt64,
    weight UInt64,
    fee UInt64,
    block_time UInt64
) ENGINE = ReplacingMergeTree()
ORDER BY (block_height, txid)
PARTITION BY toYYYYMM(toDateTime(block_time));

-- Transaction inputs
CREATE TABLE IF NOT EXISTS tx_inputs (
    txid String,
    input_index UInt32,
    prev_txid String,
    prev_vout UInt32,
    is_coinbase UInt8,
    scriptsig String,
    scriptsig_asm String,
    sequence UInt64,
    witness Array(String)
) ENGINE = ReplacingMergeTree()
ORDER BY (txid, input_index);

-- Transaction outputs
CREATE TABLE IF NOT EXISTS tx_outputs (
    txid String,
    output_index UInt32,
    value UInt64,
    scriptpubkey String,
    scriptpubkey_asm String,
    scriptpubkey_type String,
    scriptpubkey_address String
) ENGINE = ReplacingMergeTree()
ORDER BY (txid, output_index);

-- Create index for address lookups
ALTER TABLE tx_outputs ADD INDEX idx_address scriptpubkey_address TYPE bloom_filter GRANULARITY 4;

-- Addresses summary (aggregated)
CREATE TABLE IF NOT EXISTS addresses (
    address String,
    address_type String,
    funded_txo_count UInt64,
    funded_txo_sum UInt64,
    spent_txo_count UInt64,
    spent_txo_sum UInt64,
    first_seen_block UInt32,
    last_seen_block UInt32
) ENGINE = ReplacingMergeTree()
ORDER BY address;

-- Network stats per block
CREATE TABLE IF NOT EXISTS network_stats (
    block_height UInt32,
    block_hash String,
    total_tx_count UInt64,
    total_fees UInt64,
    active_addresses UInt32,
    avg_fee_rate Float64,
    fee_estimate_1 Float64,
    fee_estimate_6 Float64,
    fee_estimate_144 Float64
) ENGINE = ReplacingMergeTree()
ORDER BY block_height;

-- Cursors for sink tracking
CREATE TABLE IF NOT EXISTS cursors (
    id String,
    cursor String,
    block_num UInt64,
    block_id String
) ENGINE = ReplacingMergeTree()
ORDER BY id;

-- Materialized view for address balances
CREATE MATERIALIZED VIEW IF NOT EXISTS address_balances_mv
ENGINE = SummingMergeTree()
ORDER BY address
AS SELECT
    scriptpubkey_address AS address,
    sum(value) AS total_received
FROM tx_outputs
WHERE scriptpubkey_address != ''
GROUP BY scriptpubkey_address;
