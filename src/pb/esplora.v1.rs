/// Complete Bitcoin data structure mimicking ALL Esplora API endpoints
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EsploraCompleteData {
    /// Block information (GET /block/:hash)
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<BlockInfo>,
    /// Block summaries (GET /blocks)
    #[prost(message, repeated, tag = "2")]
    pub block_summaries: ::prost::alloc::vec::Vec<BlockSummary>,
    /// Block transactions (GET /block/:hash/txs)
    #[prost(message, repeated, tag = "3")]
    pub block_transactions: ::prost::alloc::vec::Vec<TransactionInfo>,
    /// Transaction information (GET /tx/:txid)
    #[prost(message, repeated, tag = "4")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionInfo>,
    /// Transaction hex (GET /tx/:txid/hex)
    #[prost(message, repeated, tag = "5")]
    pub transaction_hexes: ::prost::alloc::vec::Vec<TransactionHex>,
    /// Transaction status (GET /tx/:txid/status)
    #[prost(message, repeated, tag = "6")]
    pub transaction_statuses: ::prost::alloc::vec::Vec<TransactionStatus>,
    /// Address information (GET /address/:address)
    #[prost(message, repeated, tag = "7")]
    pub addresses: ::prost::alloc::vec::Vec<AddressInfo>,
    /// Address transactions (GET /address/:address/txs)
    #[prost(message, repeated, tag = "8")]
    pub address_transactions: ::prost::alloc::vec::Vec<AddressTransaction>,
    /// Address UTXOs (GET /address/:address/utxo)
    #[prost(message, repeated, tag = "9")]
    pub address_utxos: ::prost::alloc::vec::Vec<AddressUtxo>,
    /// Mempool information (GET /mempool)
    #[prost(message, optional, tag = "10")]
    pub mempool: ::core::option::Option<MempoolInfo>,
    /// Mempool recent (GET /mempool/recent)
    #[prost(message, repeated, tag = "11")]
    pub mempool_recent: ::prost::alloc::vec::Vec<MempoolTransaction>,
    /// Mempool txids (GET /mempool/txids)
    #[prost(string, repeated, tag = "12")]
    pub mempool_txids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Network tip height (GET /blocks/tip/height)
    #[prost(message, optional, tag = "13")]
    pub network_tip: ::core::option::Option<NetworkTip>,
    /// Fee estimates (GET /fee-estimates)
    #[prost(message, repeated, tag = "14")]
    pub fee_estimates: ::prost::alloc::vec::Vec<FeeEstimate>,
    /// Network statistics (GET /stats)
    #[prost(message, optional, tag = "15")]
    pub network_stats: ::core::option::Option<NetworkStats>,
    /// Search results (GET /search/:query)
    #[prost(message, repeated, tag = "16")]
    pub search_results: ::prost::alloc::vec::Vec<SearchResult>,
    /// Webhook events for real-time notifications
    #[prost(message, repeated, tag = "17")]
    pub webhook_events: ::prost::alloc::vec::Vec<WebhookEvent>,
    /// Asset information for Liquid/Elements
    #[prost(message, repeated, tag = "18")]
    pub assets: ::prost::alloc::vec::Vec<AssetInfo>,
    /// Liquid/Elements specific endpoints
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
    #[prost(uint32, tag = "2")]
    pub height: u32,
    #[prost(uint32, tag = "3")]
    pub version: u32,
    #[prost(uint64, tag = "4")]
    pub timestamp: u64,
    #[prost(uint32, tag = "5")]
    pub bits: u32,
    #[prost(uint32, tag = "6")]
    pub nonce: u32,
    #[prost(uint32, tag = "7")]
    pub tx_count: u32,
    #[prost(uint64, tag = "8")]
    pub size: u64,
    #[prost(uint64, tag = "9")]
    pub weight: u64,
    #[prost(string, tag = "10")]
    pub merkle_root: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub previous_hash: ::prost::alloc::string::String,
    #[prost(double, tag = "12")]
    pub difficulty: f64,
    #[prost(string, tag = "13")]
    pub mediantime: ::prost::alloc::string::String,
}
/// Block summary (GET /blocks)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockSummary {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub height: u32,
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
    #[prost(uint32, tag = "4")]
    pub tx_count: u32,
    #[prost(uint64, tag = "5")]
    pub size: u64,
    #[prost(uint64, tag = "6")]
    pub weight: u64,
}
/// Transaction information (GET /tx/:txid)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub version: u32,
    #[prost(uint32, tag = "3")]
    pub locktime: u32,
    #[prost(uint64, tag = "4")]
    pub size: u64,
    #[prost(uint64, tag = "5")]
    pub weight: u64,
    #[prost(uint64, tag = "6")]
    pub fee: u64,
    #[prost(message, repeated, tag = "7")]
    pub inputs: ::prost::alloc::vec::Vec<TxInput>,
    #[prost(message, repeated, tag = "8")]
    pub outputs: ::prost::alloc::vec::Vec<TxOutput>,
    #[prost(message, optional, tag = "9")]
    pub status: ::core::option::Option<TxStatus>,
    #[prost(string, repeated, tag = "10")]
    pub witness: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Transaction input
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInput {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub vout: u32,
    #[prost(bool, tag = "3")]
    pub is_coinbase: bool,
    #[prost(string, tag = "4")]
    pub scriptsig: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub scriptsig_asm: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "6")]
    pub witness: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "7")]
    pub sequence: u32,
    /// Previous output
    #[prost(message, optional, tag = "8")]
    pub prevout: ::core::option::Option<TxOutput>,
}
/// Transaction output
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutput {
    #[prost(string, tag = "1")]
    pub scriptpubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub scriptpubkey_asm: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub scriptpubkey_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub scriptpubkey_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub value: u64,
}
/// Transaction status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxStatus {
    #[prost(bool, tag = "1")]
    pub confirmed: bool,
    #[prost(uint32, optional, tag = "2")]
    pub block_height: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "4")]
    pub block_time: ::core::option::Option<u64>,
}
/// Transaction hex (GET /tx/:txid/hex)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionHex {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub hex: ::prost::alloc::string::String,
}
/// Transaction status (GET /tx/:txid/status)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionStatus {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub confirmed: bool,
    #[prost(uint32, optional, tag = "3")]
    pub block_height: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "4")]
    pub block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "5")]
    pub block_time: ::core::option::Option<u64>,
}
/// Address information (GET /address/:address)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address_type: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub funded_txo_count: u64,
    #[prost(uint64, tag = "4")]
    pub funded_txo_sum: u64,
    #[prost(uint64, tag = "5")]
    pub spent_txo_count: u64,
    #[prost(uint64, tag = "6")]
    pub spent_txo_sum: u64,
    #[prost(uint64, tag = "7")]
    pub mempool_tx_count: u64,
    #[prost(uint64, tag = "8")]
    pub mempool_funded_txo_count: u64,
    #[prost(uint64, tag = "9")]
    pub mempool_funded_txo_sum: u64,
    #[prost(uint64, tag = "10")]
    pub mempool_spent_txo_count: u64,
    #[prost(uint64, tag = "11")]
    pub mempool_spent_txo_sum: u64,
    #[prost(message, repeated, tag = "12")]
    pub utxos: ::prost::alloc::vec::Vec<UtxoInfo>,
}
/// UTXO information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxoInfo {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub vout: u32,
    #[prost(uint64, tag = "3")]
    pub value: u64,
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<TxStatus>,
}
/// Address transaction (GET /address/:address/txs)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressTransaction {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub txid: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub value: u64,
    #[prost(uint64, tag = "4")]
    pub fee: u64,
    #[prost(uint64, tag = "5")]
    pub block_height: u64,
    #[prost(uint64, tag = "6")]
    pub block_time: u64,
}
/// Address UTXO (GET /address/:address/utxo)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressUtxo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub txid: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub vout: u32,
    #[prost(uint64, tag = "4")]
    pub value: u64,
    #[prost(uint64, tag = "5")]
    pub block_height: u64,
    #[prost(uint64, tag = "6")]
    pub block_time: u64,
}
/// Mempool information (GET /mempool)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MempoolInfo {
    #[prost(uint32, tag = "1")]
    pub count: u32,
    #[prost(uint64, tag = "2")]
    pub vsize: u64,
    #[prost(uint64, tag = "3")]
    pub total_fee: u64,
    #[prost(message, repeated, tag = "4")]
    pub fee_histogram: ::prost::alloc::vec::Vec<FeeHistogramEntry>,
}
/// Fee histogram entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeHistogramEntry {
    #[prost(double, tag = "1")]
    pub feerate: f64,
    #[prost(uint64, tag = "2")]
    pub vsize: u64,
}
/// Mempool transaction (GET /mempool/recent)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MempoolTransaction {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub fee: u64,
    #[prost(uint64, tag = "3")]
    pub vsize: u64,
    #[prost(uint64, tag = "4")]
    pub value: u64,
    #[prost(uint64, tag = "5")]
    pub time: u64,
}
/// Network tip (GET /blocks/tip/height)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkTip {
    #[prost(uint32, tag = "1")]
    pub height: u32,
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
}
/// Fee estimates (GET /fee-estimates)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeEstimate {
    #[prost(uint32, tag = "1")]
    pub confirmation_target_blocks: u32,
    #[prost(double, tag = "2")]
    pub feerate_sat_per_vb: f64,
}
/// Network statistics (GET /stats)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkStats {
    #[prost(uint32, tag = "1")]
    pub block_height: u32,
    #[prost(string, tag = "2")]
    pub block_hash: ::prost::alloc::string::String,
    /// Confirmation target -> feerate
    #[prost(map = "string, double", tag = "3")]
    pub fee_estimates: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    #[prost(uint32, tag = "4")]
    pub mempool_count: u32,
    #[prost(uint64, tag = "5")]
    pub mempool_vsize: u64,
    #[prost(uint64, tag = "6")]
    pub mempool_total_fee: u64,
    #[prost(uint64, tag = "7")]
    pub total_tx_count: u64,
    #[prost(uint64, tag = "8")]
    pub total_fees: u64,
    #[prost(uint32, tag = "9")]
    pub active_addresses: u32,
    #[prost(double, tag = "10")]
    pub avg_fee_rate: f64,
}
/// Search result (GET /search/:query)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResult {
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// "address", "tx", "block"
    #[prost(string, tag = "2")]
    pub result_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub result_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Webhook event for real-time notifications
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookEvent {
    /// "block", "tx", "address"
    #[prost(string, tag = "1")]
    pub event_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub event_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub timestamp: u64,
    /// JSON string of event data
    #[prost(string, tag = "4")]
    pub data: ::prost::alloc::string::String,
}
/// Asset information for Liquid/Elements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetInfo {
    #[prost(string, tag = "1")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub tx_count: u64,
    #[prost(uint64, tag = "3")]
    pub peg_in_count: u64,
    #[prost(uint64, tag = "4")]
    pub peg_in_amount: u64,
    #[prost(uint64, tag = "5")]
    pub peg_out_count: u64,
    #[prost(uint64, tag = "6")]
    pub peg_out_amount: u64,
    #[prost(uint64, tag = "7")]
    pub burn_count: u64,
    #[prost(uint64, tag = "8")]
    pub burned_amount: u64,
}
/// Liquid asset (GET /asset/:assetid)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidAsset {
    #[prost(string, tag = "1")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub precision: u64,
    #[prost(uint64, tag = "5")]
    pub tx_count: u64,
    #[prost(uint64, tag = "6")]
    pub issued_amount: u64,
    #[prost(uint64, tag = "7")]
    pub burned_amount: u64,
    #[prost(bool, tag = "8")]
    pub reissuable: bool,
}
/// Peg-in information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PegIn {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub amount: u64,
    #[prost(uint64, tag = "3")]
    pub block_height: u64,
    #[prost(uint64, tag = "4")]
    pub block_time: u64,
}
/// Peg-out information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PegOut {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub amount: u64,
    #[prost(uint64, tag = "3")]
    pub block_height: u64,
    #[prost(uint64, tag = "4")]
    pub block_time: u64,
}
