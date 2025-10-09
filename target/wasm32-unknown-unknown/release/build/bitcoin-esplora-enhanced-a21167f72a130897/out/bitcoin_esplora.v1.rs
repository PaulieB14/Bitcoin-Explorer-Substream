/// Enhanced block information in Esplora API format
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockEsplora {
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
/// Enhanced transaction data in Esplora API format
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsEsplora {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionEsplora>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionEsplora {
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
    pub inputs: ::prost::alloc::vec::Vec<TxInputEsplora>,
    /// transaction outputs
    #[prost(message, repeated, tag = "8")]
    pub outputs: ::prost::alloc::vec::Vec<TxOutputEsplora>,
    /// transaction status
    #[prost(message, optional, tag = "9")]
    pub status: ::core::option::Option<TxStatusEsplora>,
    /// witness data
    #[prost(string, repeated, tag = "10")]
    pub witness: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInputEsplora {
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
    pub prevout: ::core::option::Option<TxOutputEsplora>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutputEsplora {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxStatusEsplora {
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
/// Address analysis and UTXO tracking
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressesEsplora {
    #[prost(message, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<AddressInfoEsplora>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressInfoEsplora {
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
    /// UTXOs
    #[prost(message, repeated, tag = "7")]
    pub utxos: ::prost::alloc::vec::Vec<UtxoInfoEsplora>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtxoInfoEsplora {
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
    pub status: ::core::option::Option<TxStatusEsplora>,
}
/// Network statistics and fee estimates
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
