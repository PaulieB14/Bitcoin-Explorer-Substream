/// Main output structure that contains all Esplora API data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EsploraCompleteData {
    /// Block information
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<BlockInfo>,
    #[prost(message, repeated, tag = "2")]
    pub block_summaries: ::prost::alloc::vec::Vec<BlockSummary>,
    #[prost(message, repeated, tag = "3")]
    pub block_transactions: ::prost::alloc::vec::Vec<TransactionInfo>,
    /// Transaction data
    #[prost(message, repeated, tag = "4")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionInfo>,
    #[prost(message, repeated, tag = "5")]
    pub transaction_hexes: ::prost::alloc::vec::Vec<TransactionHex>,
    #[prost(message, repeated, tag = "6")]
    pub transaction_statuses: ::prost::alloc::vec::Vec<TransactionStatus>,
    /// Address data
    #[prost(message, repeated, tag = "7")]
    pub addresses: ::prost::alloc::vec::Vec<AddressInfo>,
    #[prost(message, repeated, tag = "8")]
    pub address_transactions: ::prost::alloc::vec::Vec<AddressTransaction>,
    #[prost(message, repeated, tag = "9")]
    pub address_utxos: ::prost::alloc::vec::Vec<AddressUtxo>,
    /// Mempool data
    #[prost(message, optional, tag = "10")]
    pub mempool: ::core::option::Option<MempoolInfo>,
    #[prost(message, repeated, tag = "11")]
    pub mempool_recent: ::prost::alloc::vec::Vec<MempoolTransaction>,
    #[prost(string, repeated, tag = "12")]
    pub mempool_txids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Network data
    #[prost(message, optional, tag = "13")]
    pub network_tip: ::core::option::Option<NetworkTip>,
    #[prost(message, repeated, tag = "14")]
    pub fee_estimates: ::prost::alloc::vec::Vec<FeeEstimate>,
    #[prost(message, optional, tag = "15")]
    pub network_stats: ::core::option::Option<NetworkStats>,
    /// Search and webhooks
    #[prost(message, repeated, tag = "16")]
    pub search_results: ::prost::alloc::vec::Vec<SearchResult>,
    #[prost(message, repeated, tag = "17")]
    pub webhook_events: ::prost::alloc::vec::Vec<WebhookEvent>,
    /// Liquid/Elements data
    #[prost(message, repeated, tag = "18")]
    pub assets: ::prost::alloc::vec::Vec<AssetInfo>,
    #[prost(message, repeated, tag = "19")]
    pub liquid_assets: ::prost::alloc::vec::Vec<LiquidAsset>,
    #[prost(message, repeated, tag = "20")]
    pub peg_ins: ::prost::alloc::vec::Vec<PegIn>,
    #[prost(message, repeated, tag = "21")]
    pub peg_outs: ::prost::alloc::vec::Vec<PegOut>,
}
/// Block information (GET /block/:hash)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfo {
    /// block hash
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// block height
    #[prost(uint32, tag = "2")]
    pub height: u32,
    /// block version
    #[prost(uint32, tag = "3")]
    pub version: u32,
    /// block timestamp
    #[prost(uint64, tag = "4")]
    pub timestamp: u64,
    /// block bits
    #[prost(uint32, tag = "5")]
    pub bits: u32,
    /// block nonce
    #[prost(uint32, tag = "6")]
    pub nonce: u32,
    /// number of transactions
    #[prost(uint32, tag = "7")]
    pub tx_count: u32,
    /// block size in bytes
    #[prost(uint64, tag = "8")]
    pub size: u64,
    /// block weight
    #[prost(uint64, tag = "9")]
    pub weight: u64,
    /// merkle root
    #[prost(string, tag = "10")]
    pub merkle_root: ::prost::alloc::string::String,
    /// previous block hash
    #[prost(string, tag = "11")]
    pub previous_hash: ::prost::alloc::string::String,
    /// block difficulty
    #[prost(double, tag = "12")]
    pub difficulty: f64,
    /// median time
    #[prost(string, tag = "13")]
    pub mediantime: ::prost::alloc::string::String,
}
/// Block summary (GET /blocks)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockSummary {
    /// block hash
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// block height
    #[prost(uint32, tag = "2")]
    pub height: u32,
    /// block timestamp
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
    /// number of transactions
    #[prost(uint32, tag = "4")]
    pub tx_count: u32,
    /// block size
    #[prost(uint64, tag = "5")]
    pub size: u64,
    /// block weight
    #[prost(uint64, tag = "6")]
    pub weight: u64,
}
/// Transaction information (GET /tx/:txid)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    /// transaction ID
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    /// transaction version
    #[prost(uint32, tag = "2")]
    pub version: u32,
    /// locktime
    #[prost(uint32, tag = "3")]
    pub locktime: u32,
    /// transaction size
    #[prost(uint64, tag = "4")]
    pub size: u64,
    /// transaction weight
    #[prost(uint64, tag = "5")]
    pub weight: u64,
    /// fee in satoshis
    #[prost(uint64, tag = "6")]
    pub fee: u64,
    /// transaction inputs
    #[prost(message, repeated, tag = "7")]
    pub inputs: ::prost::alloc::vec::Vec<TxInput>,
    /// transaction outputs
    #[prost(message, repeated, tag = "8")]
    pub outputs: ::prost::alloc::vec::Vec<TxOutput>,
    /// transaction status
    #[prost(message, optional, tag = "9")]
    pub status: ::core::option::Option<TxStatus>,
    /// witness data
    #[prost(string, repeated, tag = "10")]
    pub witness: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Transaction input
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInput {
    /// previous transaction ID
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    /// output index
    #[prost(uint32, tag = "2")]
    pub vout: u32,
    /// is coinbase input
    #[prost(bool, tag = "3")]
    pub is_coinbase: bool,
    /// script signature
    #[prost(string, tag = "4")]
    pub scriptsig: ::prost::alloc::string::String,
    /// script signature asm
    #[prost(string, tag = "5")]
    pub scriptsig_asm: ::prost::alloc::string::String,
    /// witness data
    #[prost(string, repeated, tag = "6")]
    pub witness: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// sequence number
    #[prost(uint32, tag = "7")]
    pub sequence: u32,
    /// previous output
    #[prost(message, optional, tag = "8")]
    pub prevout: ::core::option::Option<TxOutput>,
}
/// Transaction output
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutput {
    /// script pubkey
    #[prost(string, tag = "1")]
    pub scriptpubkey: ::prost::alloc::string::String,
    /// script pubkey asm
    #[prost(string, tag = "2")]
    pub scriptpubkey_asm: ::prost::alloc::string::String,
    /// script pubkey type
    #[prost(string, tag = "3")]
    pub scriptpubkey_type: ::prost::alloc::string::String,
    /// script pubkey address
    #[prost(string, tag = "4")]
    pub scriptpubkey_address: ::prost::alloc::string::String,
    /// value in satoshis
    #[prost(uint64, tag = "5")]
    pub value: u64,
}
/// Transaction status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxStatus {
    /// is confirmed
    #[prost(bool, tag = "1")]
    pub confirmed: bool,
    /// block height (if confirmed)
    #[prost(uint32, tag = "2")]
    pub block_height: u32,
    /// block hash (if confirmed)
    #[prost(string, tag = "3")]
    pub block_hash: ::prost::alloc::string::String,
    /// block time (if confirmed)
    #[prost(uint64, tag = "4")]
    pub block_time: u64,
}
/// Transaction hex
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionHex {
    /// transaction ID
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    /// raw transaction hex
    #[prost(string, tag = "2")]
    pub hex: ::prost::alloc::string::String,
}
/// Transaction status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionStatus {
    /// transaction ID
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    /// is confirmed
    #[prost(bool, tag = "2")]
    pub confirmed: bool,
    /// block height (if confirmed)
    #[prost(uint32, tag = "3")]
    pub block_height: u32,
    /// block hash (if confirmed)
    #[prost(string, tag = "4")]
    pub block_hash: ::prost::alloc::string::String,
    /// block time (if confirmed)
    #[prost(uint64, tag = "5")]
    pub block_time: u64,
}
/// Address information (GET /address/:address)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressInfo {
    /// address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// address type
    #[prost(string, tag = "2")]
    pub address_type: ::prost::alloc::string::String,
    /// funded txo count
    #[prost(uint64, tag = "3")]
    pub funded_txo_count: u64,
    /// funded txo sum in satoshis
    #[prost(uint64, tag = "4")]
    pub funded_txo_sum: u64,
    /// spent txo count
    #[prost(uint64, tag = "5")]
    pub spent_txo_count: u64,
    /// spent txo sum in satoshis
    #[prost(uint64, tag = "6")]
    pub spent_txo_sum: u64,
    /// mempool tx count
    #[prost(uint64, tag = "7")]
    pub mempool_tx_count: u64,
    /// mempool funded txo count
    #[prost(uint64, tag = "8")]
    pub mempool_funded_txo_count: u64,
    /// mempool funded txo sum
    #[prost(uint64, tag = "9")]
    pub mempool_funded_txo_sum: u64,
    /// mempool spent txo count
    #[prost(uint64, tag = "10")]
    pub mempool_spent_txo_count: u64,
    /// mempool spent txo sum
    #[prost(uint64, tag = "11")]
    pub mempool_spent_txo_sum: u64,
    /// UTXOs
    #[prost(message, repeated, tag = "12")]
    pub utxos: ::prost::alloc::vec::Vec<UtxoInfo>,
}
/// UTXO information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxoInfo {
    /// transaction ID
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    /// output index
    #[prost(uint32, tag = "2")]
    pub vout: u32,
    /// value in satoshis
    #[prost(uint64, tag = "3")]
    pub value: u64,
    /// status
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<TxStatus>,
}
/// Address transaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressTransaction {
    /// address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// transaction ID
    #[prost(string, tag = "2")]
    pub txid: ::prost::alloc::string::String,
    /// value in satoshis
    #[prost(uint64, tag = "3")]
    pub value: u64,
    /// fee in satoshis
    #[prost(uint64, tag = "4")]
    pub fee: u64,
    /// block height
    #[prost(uint64, tag = "5")]
    pub block_height: u64,
    /// block time
    #[prost(uint64, tag = "6")]
    pub block_time: u64,
}
/// Address UTXO
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressUtxo {
    /// address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// transaction ID
    #[prost(string, tag = "2")]
    pub txid: ::prost::alloc::string::String,
    /// output index
    #[prost(uint32, tag = "3")]
    pub vout: u32,
    /// value in satoshis
    #[prost(uint64, tag = "4")]
    pub value: u64,
    /// block height
    #[prost(uint64, tag = "5")]
    pub block_height: u64,
    /// block time
    #[prost(uint64, tag = "6")]
    pub block_time: u64,
}
/// Mempool information (GET /mempool)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MempoolInfo {
    /// transaction count
    #[prost(uint32, tag = "1")]
    pub count: u32,
    /// total vsize
    #[prost(uint64, tag = "2")]
    pub vsize: u64,
    /// total fee in satoshis
    #[prost(uint64, tag = "3")]
    pub total_fee: u64,
    /// fee histogram
    #[prost(message, repeated, tag = "4")]
    pub fee_histogram: ::prost::alloc::vec::Vec<FeeHistogramEntry>,
}
/// Fee histogram entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeHistogramEntry {
    /// fee rate
    #[prost(double, tag = "1")]
    pub feerate: f64,
    /// vsize
    #[prost(uint64, tag = "2")]
    pub vsize: u64,
}
/// Mempool transaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MempoolTransaction {
    /// transaction ID
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    /// fee in satoshis
    #[prost(uint64, tag = "2")]
    pub fee: u64,
    /// vsize
    #[prost(uint64, tag = "3")]
    pub vsize: u64,
    /// value in satoshis
    #[prost(uint64, tag = "4")]
    pub value: u64,
    /// time
    #[prost(uint64, tag = "5")]
    pub time: u64,
}
/// Network tip (GET /blocks/tip/height)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkTip {
    /// block height
    #[prost(uint32, tag = "1")]
    pub height: u32,
    /// block hash
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
    /// timestamp
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
}
/// Fee estimate
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeEstimate {
    /// confirmation target
    #[prost(uint32, tag = "1")]
    pub confirmation_target_blocks: u32,
    /// fee rate in sat/vB
    #[prost(double, tag = "2")]
    pub feerate_sat_per_vb: f64,
}
/// Network statistics (GET /stats)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkStats {
    /// block height
    #[prost(uint32, tag = "1")]
    pub block_height: u32,
    /// block hash
    #[prost(string, tag = "2")]
    pub block_hash: ::prost::alloc::string::String,
    /// fee estimates
    #[prost(map = "string, double", tag = "3")]
    pub fee_estimates: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    /// mempool count
    #[prost(uint32, tag = "4")]
    pub mempool_count: u32,
    /// mempool vsize
    #[prost(uint64, tag = "5")]
    pub mempool_vsize: u64,
    /// mempool total fee
    #[prost(uint64, tag = "6")]
    pub mempool_total_fee: u64,
    /// total tx count
    #[prost(uint64, tag = "7")]
    pub total_tx_count: u64,
    /// total fees
    #[prost(uint64, tag = "8")]
    pub total_fees: u64,
    /// active addresses
    #[prost(uint32, tag = "9")]
    pub active_addresses: u32,
    /// average fee rate
    #[prost(double, tag = "10")]
    pub avg_fee_rate: f64,
}
/// Search result
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResult {
    /// search query
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// result type
    #[prost(string, tag = "2")]
    pub result_type: ::prost::alloc::string::String,
    /// result ID
    #[prost(string, tag = "3")]
    pub result_id: ::prost::alloc::string::String,
    /// description
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
/// Webhook event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookEvent {
    /// event type
    #[prost(string, tag = "1")]
    pub event_type: ::prost::alloc::string::String,
    /// event ID
    #[prost(string, tag = "2")]
    pub event_id: ::prost::alloc::string::String,
    /// timestamp
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
    /// event data
    #[prost(string, tag = "4")]
    pub data: ::prost::alloc::string::String,
}
/// Asset information (Liquid/Elements)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetInfo {
    /// asset ID
    #[prost(string, tag = "1")]
    pub asset_id: ::prost::alloc::string::String,
    /// transaction count
    #[prost(uint64, tag = "2")]
    pub tx_count: u64,
    /// peg in count
    #[prost(uint64, tag = "3")]
    pub peg_in_count: u64,
    /// peg in amount
    #[prost(uint64, tag = "4")]
    pub peg_in_amount: u64,
    /// peg out count
    #[prost(uint64, tag = "5")]
    pub peg_out_count: u64,
    /// peg out amount
    #[prost(uint64, tag = "6")]
    pub peg_out_amount: u64,
    /// burn count
    #[prost(uint64, tag = "7")]
    pub burn_count: u64,
    /// burned amount
    #[prost(uint64, tag = "8")]
    pub burned_amount: u64,
}
/// Liquid asset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidAsset {
    /// asset ID
    #[prost(string, tag = "1")]
    pub asset_id: ::prost::alloc::string::String,
    /// asset name
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// asset ticker
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    /// precision
    #[prost(uint64, tag = "4")]
    pub precision: u64,
    /// transaction count
    #[prost(uint64, tag = "5")]
    pub tx_count: u64,
    /// issued amount
    #[prost(uint64, tag = "6")]
    pub issued_amount: u64,
    /// burned amount
    #[prost(uint64, tag = "7")]
    pub burned_amount: u64,
    /// is reissuable
    #[prost(bool, tag = "8")]
    pub reissuable: bool,
}
/// Peg in
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PegIn {
    /// transaction ID
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    /// amount
    #[prost(uint64, tag = "2")]
    pub amount: u64,
    /// block height
    #[prost(uint64, tag = "3")]
    pub block_height: u64,
    /// block time
    #[prost(uint64, tag = "4")]
    pub block_time: u64,
}
/// Peg out
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PegOut {
    /// transaction ID
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    /// amount
    #[prost(uint64, tag = "2")]
    pub amount: u64,
    /// block height
    #[prost(uint64, tag = "3")]
    pub block_height: u64,
    /// block time
    #[prost(uint64, tag = "4")]
    pub block_time: u64,
}
