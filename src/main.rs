use std::collections::{HashMap, HashSet};
use reqwest::{Client, Error as ReqwestError};
use Aletheia::types::types::{TransactionOrHash, BlockWitness};
use Aletheia::json_rpc::json_rpc_methods::{rpc_batch, get_block, get_transaction_receipt, get_account_state, get_account_proof};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // 1. Get current and previous block dynamically
    // Fetch the latest block
    let curr_block = get_block(&client, "latest", true).await?;
    // Parse the current block number as u64
    let curr_block_num_u64 = u64::from_str_radix(curr_block.number.trim_start_matches("0x"), 16)?;
    // Compute the previous block number
    // 2. Collect all addresses involved
    let mut addresses = HashSet::new();
    let mut full_transactions = Vec::new();
    for tx in &curr_block.transactions {
        if let TransactionOrHash::Full(t) = tx {
            addresses.insert(t.from.clone());
            if let Some(to) = &t.to {
                addresses.insert(to.clone());
            }
            full_transactions.push(t.clone());
        }
        // Optionally parse tx.input for more addresses (ERC-20, etc.)
    }
    for tx in &curr_block.transactions {
        if let TransactionOrHash::Full(t) = tx {
            addresses.insert(t.from.clone());
            if let Some(to) = &t.to {
                addresses.insert(to.clone());
            }
        }
    }
    // 3. Get previous block
    let prev_block_num_u64 = curr_block_num_u64.saturating_sub(1);
    let prev_block_num_hex = format!("0x{:x}", prev_block_num_u64);
    let prev_block = get_block(&client, &prev_block_num_hex, true).await?;

    // 4. Get transaction receipts
    let mut receipts = Vec::new();
    for tx in &full_transactions {
        let receipt = get_transaction_receipt(&client, &tx.hash).await?;
        receipts.push(receipt);
    }

    // 5. Get account states and merkle proofs
    let mut pre_state = HashMap::new();
    let mut post_state = HashMap::new();
    let mut merkle_proofs = HashMap::new();
    for addr in &addresses {
        let state = get_account_state(&client, addr, &prev_block_num_hex, &curr_block.number).await?;
        pre_state.insert(addr.clone(), state.clone());
        post_state.insert(addr.clone(), state);

        // For demonstration, we use empty storage keys
        let storage_keys: Vec<String> = vec![];
        let proof_prev = get_account_proof(&client, addr, &storage_keys, &prev_block_num_hex).await?;
        let proof_curr = get_account_proof(&client, addr, &storage_keys, &curr_block.number).await?;
        merkle_proofs.insert(addr.clone(), (proof_prev, proof_curr));
    }

    // 6. Package everything
    let witness = BlockWitness {
        block_header: curr_block.clone(),
        prev_block_header: prev_block,
        transactions: full_transactions,
        receipts,
        pre_state,
        post_state,
        merkle_proofs,
    };

    println!("{}", serde_json::to_string_pretty(&witness)?);

    Ok(())
}