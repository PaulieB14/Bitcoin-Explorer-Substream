use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use hex;

// Import generated protobuf types
pub mod bitcoin_esplora {
    include!(concat!(env!("OUT_DIR"), "/bitcoin_esplora.v1.rs"));
}

// Use the generated protobuf types
use bitcoin_esplora::*;

// Import real Bitcoin protobuf types
use substreams_bitcoin::pb::btc::v1::{Block, Transaction, Vin, Vout};

/// Enhanced block metadata in Esplora API format
/// Based on substreams-bitcoin-main but with Esplora API compatibility
#[substreams::handlers::map]
fn map_block_esplora(clock: Clock, block: Block) -> Result<Clock, Error> {
    let block_info = BlockEsplora {
        id: block.hash.clone(),
        height: block.height as u32,
        version: block.version as u32,
        timestamp: block.time as u64,
        bits: block.bits.parse::<u32>().unwrap_or(0),
        nonce: block.nonce,
        tx_count: block.tx.len() as u32,
        size: block.size as u64,
        weight: block.weight as u64,
        merkle_root: block.merkle_root.clone(),
        previous_hash: "".to_string(), // Not available in this structure
        difficulty: calculate_difficulty(&block),
        mediantime: block.mediantime.to_string(),
    };

    substreams::log::info!("=== ENHANCED BLOCK DATA ===");
    substreams::log::info!("Block Hash: {}", block_info.id);
    substreams::log::info!("Block Height: {}", block_info.height);
    substreams::log::info!("Transaction Count: {}", block_info.tx_count);
    substreams::log::info!("Block Size: {} bytes", block_info.size);
    substreams::log::info!("Block Weight: {} weight units", block_info.weight);
    substreams::log::info!("Difficulty: {}", block_info.difficulty);

    Ok(clock)
}

/// Enhanced transaction data with Esplora API format
/// Based on substreams-bitcoin-main but with full transaction details
#[substreams::handlers::map]
fn map_transactions_esplora(clock: Clock, block: Block) -> Result<Clock, Error> {
    let mut transactions = Vec::new();
    
    for tx in &block.tx {
        let transaction = TransactionEsplora {
            txid: tx.txid.clone(),
            version: tx.version,
            locktime: tx.locktime,
            size: tx.size as u64,
            weight: tx.weight as u64,
            fee: calculate_tx_fee(tx),
            inputs: tx.vin.iter().map(|input| process_tx_input(input)).collect(),
            outputs: tx.vout.iter().map(|output| process_tx_output(output)).collect(),
            status: Some(TxStatusEsplora {
                confirmed: true,
                block_height: block.height as u32,
                block_hash: block.hash.clone(),
                block_time: block.time as u64,
            }),
            witness: vec![], // TODO: Process witness data
        };
        transactions.push(transaction);
    }

    substreams::log::info!("=== ENHANCED TRANSACTION DATA ===");
    substreams::log::info!("Processed {} transactions", transactions.len());
    
    // Log first few transactions for debugging
    for (i, tx) in transactions.iter().take(3).enumerate() {
        substreams::log::info!("TX {}: {} - {} inputs, {} outputs, fee: {} satoshis", 
                              i + 1, tx.txid, tx.inputs.len(), tx.outputs.len(), tx.fee);
    }

    Ok(clock)
}

/// Address analysis and UTXO tracking
#[substreams::handlers::map]
fn map_addresses_esplora(clock: Clock, block: Block) -> Result<Clock, Error> {
    let mut addresses = std::collections::HashMap::new();
    
    for tx in &block.tx {
        for output in &tx.vout {
            let address = output.script_pub_key.as_ref().map(|s| s.address.clone()).unwrap_or_default();
            if !address.is_empty() {
                let entry = addresses.entry(address.clone()).or_insert(AddressInfoEsplora {
                    address: address.clone(),
                    address_type: output.script_pub_key.as_ref().map(|s| s.r#type.clone()).unwrap_or_default(),
                    funded_txo_count: 0,
                    funded_txo_sum: 0,
                    spent_txo_count: 0,
                    spent_txo_sum: 0,
                    utxos: vec![],
                });
                entry.funded_txo_count += 1;
                entry.funded_txo_sum += (output.value * 100_000_000.0) as u64;
                
                // Add UTXO
                entry.utxos.push(UtxoInfoEsplora {
                    txid: tx.txid.clone(),
                    vout: 0, // Will be set properly in real implementation
                    value: (output.value * 100_000_000.0) as u64,
                    status: Some(TxStatusEsplora {
                        confirmed: true,
                        block_height: block.height as u32,
                        block_hash: block.hash.clone(),
                        block_time: block.time as u64,
                    }),
                });
            }
        }
    }

    let address_list: Vec<AddressInfoEsplora> = addresses.into_values().collect();
    
    substreams::log::info!("=== ENHANCED ADDRESS DATA ===");
    substreams::log::info!("Found {} unique addresses", address_list.len());
    
    // Log first few addresses for debugging
    for (i, addr) in address_list.iter().take(3).enumerate() {
        substreams::log::info!("Address {}: {} - {} UTXOs, {} satoshis", 
                              i + 1, addr.address, addr.funded_txo_count, addr.funded_txo_sum);
    }

    Ok(clock)
}

/// Network statistics and fee estimates
#[substreams::handlers::map]
fn map_network_stats(clock: Clock, block: Block) -> Result<Clock, Error> {
    let total_fees: u64 = block.tx.iter().map(|tx| calculate_tx_fee(tx)).sum();
    let avg_fee_rate = if !block.tx.is_empty() {
        total_fees as f64 / block.tx.len() as f64
    } else {
        0.0
    };
    
    let active_addresses = count_unique_addresses(&block);
    
    let stats = NetworkStats {
        block_height: block.height as u32,
        block_hash: block.hash.clone(),
        fee_estimates: std::collections::HashMap::new(), // TODO: Calculate real fee estimates
        mempool_count: 0, // TODO: Get mempool data
        mempool_vsize: 0,
        mempool_total_fee: 0,
        total_tx_count: block.tx.len() as u64,
        total_fees: total_fees,
        active_addresses,
        avg_fee_rate,
    };

    substreams::log::info!("=== NETWORK STATISTICS ===");
    substreams::log::info!("Block Height: {}", stats.block_height);
    substreams::log::info!("Total Transactions: {}", stats.total_tx_count);
    substreams::log::info!("Total Fees: {} satoshis", stats.total_fees);
    substreams::log::info!("Active Addresses: {}", stats.active_addresses);
    substreams::log::info!("Average Fee Rate: {} satoshis/tx", stats.avg_fee_rate);

    Ok(clock)
}

// Helper functions

fn process_tx_input(input: &Vin) -> TxInputEsplora {
    TxInputEsplora {
        txid: input.txid.clone(),
        vout: input.vout,
        is_coinbase: !input.coinbase.is_empty(),
        scriptsig: input.script_sig.as_ref().map(|s| s.hex.clone()).unwrap_or_default(),
        scriptsig_asm: input.script_sig.as_ref().map(|s| s.asm.clone()).unwrap_or_default(),
        witness: input.txinwitness.clone(),
        sequence: input.sequence,
        prevout: Some(TxOutputEsplora {
            scriptpubkey: String::new(),
            scriptpubkey_asm: String::new(),
            scriptpubkey_type: String::new(),
            scriptpubkey_address: String::new(),
            value: 0,
        }),
    }
}

fn process_tx_output(output: &Vout) -> TxOutputEsplora {
    let scriptpubkey = output.script_pub_key.as_ref().map(|s| s.hex.clone()).unwrap_or_default();
    let address = output.script_pub_key.as_ref().map(|s| s.address.clone()).unwrap_or_default();
    
    TxOutputEsplora {
        scriptpubkey,
        scriptpubkey_asm: output.script_pub_key.as_ref().map(|s| s.asm.clone()).unwrap_or_default(),
        scriptpubkey_type: output.script_pub_key.as_ref().map(|s| s.r#type.clone()).unwrap_or_default(),
        scriptpubkey_address: address,
        value: (output.value * 100_000_000.0) as u64, // Convert BTC to satoshis
    }
}

fn calculate_tx_fee(tx: &Transaction) -> u64 {
    // Simplified fee calculation - in real implementation, this would need to look up input values
    let input_total: u64 = tx.vin.len() as u64 * 100000; // Rough estimate
    let output_total: u64 = tx.vout.iter().map(|o| (o.value * 100_000_000.0) as u64).sum();
    
    if input_total > output_total {
        input_total - output_total
    } else {
        0
    }
}

fn calculate_difficulty(block: &Block) -> f64 {
    // Simplified difficulty calculation based on bits
    if let Ok(bits) = block.bits.parse::<u32>() {
        if bits > 0 {
            (0x1d00ffff as f64) / (bits as f64)
        } else {
            1.0
        }
    } else {
        1.0
    }
}

fn count_unique_addresses(block: &Block) -> u32 {
    let mut addresses = std::collections::HashSet::new();
    
    for tx in &block.tx {
        for output in &tx.vout {
            let address = output.script_pub_key.as_ref().map(|s| s.address.clone()).unwrap_or_default();
            if !address.is_empty() {
                addresses.insert(address);
            }
        }
    }
    
    addresses.len() as u32
}
