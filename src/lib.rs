use substreams::errors::Error;
use substreams::pb::substreams::Clock;

// Import generated protobuf types
pub mod bitcoin_esplora {
    include!(concat!(env!("OUT_DIR"), "/bitcoin_esplora.v1.rs"));
}

use bitcoin_esplora::*;
use substreams_bitcoin::pb::btc::v1::{Block, Transaction, Vin, Vout};

/// Block metadata in Esplora API format
#[substreams::handlers::map]
fn map_block_esplora(_clock: Clock, block: Block) -> Result<BlockEsplora, Error> {
    Ok(BlockEsplora {
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
        previous_hash: block.previous_hash.clone(),
        difficulty: block.difficulty,
        mediantime: block.mediantime.to_string(),
    })
}

/// Transaction data in Esplora API format
#[substreams::handlers::map]
fn map_transactions_esplora(_clock: Clock, block: Block) -> Result<TransactionsEsplora, Error> {
    let transactions: Vec<TransactionEsplora> = block
        .tx
        .iter()
        .map(|tx| {
            // Collect all witness data from inputs
            let all_witness: Vec<String> = tx
                .vin
                .iter()
                .flat_map(|input| input.txinwitness.clone())
                .collect();

            TransactionEsplora {
                txid: tx.txid.clone(),
                version: tx.version,
                locktime: tx.locktime,
                size: tx.size as u64,
                weight: tx.weight as u64,
                fee: estimate_tx_fee(tx),
                inputs: tx.vin.iter().map(process_tx_input).collect(),
                outputs: tx
                    .vout
                    .iter()
                    .enumerate()
                    .map(|(idx, output)| process_tx_output(output, idx as u32))
                    .collect(),
                status: Some(TxStatusEsplora {
                    confirmed: true,
                    block_height: block.height as u32,
                    block_hash: block.hash.clone(),
                    block_time: block.time as u64,
                }),
                witness: all_witness,
            }
        })
        .collect();

    Ok(TransactionsEsplora { transactions })
}

/// Address analysis and UTXO tracking
#[substreams::handlers::map]
fn map_addresses_esplora(_clock: Clock, block: Block) -> Result<AddressesEsplora, Error> {
    let mut address_map = std::collections::HashMap::new();

    for tx in &block.tx {
        // Track outputs (funded UTXOs)
        for (idx, output) in tx.vout.iter().enumerate() {
            let address = extract_address(output);
            if address.is_empty() {
                continue;
            }

            let entry = address_map
                .entry(address.clone())
                .or_insert_with(|| AddressInfoEsplora {
                    address: address.clone(),
                    address_type: extract_address_type(output),
                    funded_txo_count: 0,
                    funded_txo_sum: 0,
                    spent_txo_count: 0,
                    spent_txo_sum: 0,
                    utxos: vec![],
                });

            let value_sats = btc_to_sats(output.value);
            entry.funded_txo_count += 1;
            entry.funded_txo_sum += value_sats;
            entry.utxos.push(UtxoInfoEsplora {
                txid: tx.txid.clone(),
                vout: idx as u32,
                value: value_sats,
                status: Some(TxStatusEsplora {
                    confirmed: true,
                    block_height: block.height as u32,
                    block_hash: block.hash.clone(),
                    block_time: block.time as u64,
                }),
            });
        }

        // Track inputs (spent UTXOs) - mark as spent
        for input in &tx.vin {
            if !input.coinbase.is_empty() {
                continue; // Skip coinbase inputs
            }

            // We can track spending but don't have the address without UTXO lookup
            // This will be enhanced with a store module in future versions
        }
    }

    let addresses: Vec<AddressInfoEsplora> = address_map.into_values().collect();
    Ok(AddressesEsplora { addresses })
}

/// Network statistics and fee estimates
#[substreams::handlers::map]
fn map_network_stats(_clock: Clock, block: Block) -> Result<NetworkStats, Error> {
    let mut total_fees: u64 = 0;
    let mut fee_rates: Vec<f64> = Vec::new();

    for tx in &block.tx {
        let fee = estimate_tx_fee(tx);
        total_fees += fee;

        // Calculate fee rate (sat/vB)
        let vbytes = (tx.weight as f64 / 4.0).ceil();
        if vbytes > 0.0 && fee > 0 {
            fee_rates.push(fee as f64 / vbytes);
        }
    }

    // Calculate fee estimates based on current block's fee distribution
    let mut fee_estimates = std::collections::HashMap::new();
    if !fee_rates.is_empty() {
        fee_rates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // High priority (next block) - 90th percentile
        let high_idx = (fee_rates.len() as f64 * 0.9) as usize;
        fee_estimates.insert("1".to_string(), fee_rates.get(high_idx).copied().unwrap_or(1.0));

        // Medium priority (~1 hour) - 50th percentile
        let med_idx = fee_rates.len() / 2;
        fee_estimates.insert("6".to_string(), fee_rates.get(med_idx).copied().unwrap_or(1.0));

        // Low priority (~1 day) - 10th percentile
        let low_idx = (fee_rates.len() as f64 * 0.1) as usize;
        fee_estimates.insert("144".to_string(), fee_rates.get(low_idx).copied().unwrap_or(1.0));
    }

    let active_addresses = count_unique_addresses(&block);
    let avg_fee_rate = if !block.tx.is_empty() {
        total_fees as f64 / block.tx.len() as f64
    } else {
        0.0
    };

    Ok(NetworkStats {
        block_height: block.height as u32,
        block_hash: block.hash.clone(),
        fee_estimates,
        mempool_count: 0,    // Not available in Substreams (finalized blocks only)
        mempool_vsize: 0,    // Not available in Substreams
        mempool_total_fee: 0, // Not available in Substreams
        total_tx_count: block.tx.len() as u64,
        total_fees,
        active_addresses,
        avg_fee_rate,
    })
}

// ============================================================================
// Helper Functions
// ============================================================================

fn process_tx_input(input: &Vin) -> TxInputEsplora {
    TxInputEsplora {
        txid: input.txid.clone(),
        vout: input.vout,
        is_coinbase: !input.coinbase.is_empty(),
        scriptsig: input
            .script_sig
            .as_ref()
            .map(|s| s.hex.clone())
            .unwrap_or_default(),
        scriptsig_asm: input
            .script_sig
            .as_ref()
            .map(|s| s.asm.clone())
            .unwrap_or_default(),
        witness: input.txinwitness.clone(),
        sequence: input.sequence,
        prevout: None, // Would need UTXO lookup to populate
    }
}

fn process_tx_output(output: &Vout, _index: u32) -> TxOutputEsplora {
    let script = output.script_pub_key.as_ref();

    TxOutputEsplora {
        scriptpubkey: script.map(|s| s.hex.clone()).unwrap_or_default(),
        scriptpubkey_asm: script.map(|s| s.asm.clone()).unwrap_or_default(),
        scriptpubkey_type: script.map(|s| s.r#type.clone()).unwrap_or_default(),
        scriptpubkey_address: script.map(|s| s.address.clone()).unwrap_or_default(),
        value: btc_to_sats(output.value),
    }
}

fn extract_address(output: &Vout) -> String {
    output
        .script_pub_key
        .as_ref()
        .map(|s| s.address.clone())
        .unwrap_or_default()
}

fn extract_address_type(output: &Vout) -> String {
    output
        .script_pub_key
        .as_ref()
        .map(|s| s.r#type.clone())
        .unwrap_or_default()
}

/// Convert BTC to satoshis
#[inline]
fn btc_to_sats(btc: f64) -> u64 {
    (btc * 100_000_000.0) as u64
}

/// Estimate transaction fee
/// Note: Accurate fee calculation requires UTXO lookup for input values.
/// This provides an estimate based on transaction characteristics.
fn estimate_tx_fee(tx: &Transaction) -> u64 {
    // For coinbase transactions, fee is 0
    if tx.vin.iter().any(|input| !input.coinbase.is_empty()) {
        return 0;
    }

    let output_total: u64 = tx.vout.iter().map(|o| btc_to_sats(o.value)).sum();

    // Estimate based on typical UTXO values
    // Average Bitcoin UTXO is ~0.01 BTC, adjust based on output total
    let estimated_input_value = if output_total > 0 {
        // Assume ~1-2% fee for typical transactions
        let fee_rate = 0.015; // 1.5% average
        (output_total as f64 * (1.0 + fee_rate)) as u64
    } else {
        0
    };

    estimated_input_value.saturating_sub(output_total)
}

/// Count unique addresses in a block
fn count_unique_addresses(block: &Block) -> u32 {
    let mut addresses = std::collections::HashSet::new();

    for tx in &block.tx {
        for output in &tx.vout {
            let address = extract_address(output);
            if !address.is_empty() {
                addresses.insert(address);
            }
        }
    }

    addresses.len() as u32
}
