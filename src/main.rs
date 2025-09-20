use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use reqwest::{Client, Error as ReqwestError};

const RPC_URL: &str = "http://127.0.0.1:8545"; // Change to your rollup RPC endpoint

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Block {
    number: String,
    hash: String,
    parentHash: String,
    stateRoot: String,
    transactions: Vec<TransactionOrHash>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Transaction {
    hash: String,
    from: String,
    to: Option<String>,
    input: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum TransactionOrHash {
    Full(Transaction),
    Hash(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionReceipt {
    transactionHash: String,
    gasUsed: String,
    status: String,
    logs: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountProof {
    address: String,
    balance: String,
    nonce: String,
    codeHash: String,
    storageHash: String,
    accountProof: Vec<String>,
    storageProof: Vec<StorageProof>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StorageProof {
    key: String,
    value: String,
    proof: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AccountState {
    balance_before: String,
    balance_after: String,
    nonce_before: String,
    nonce_after: String,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockWitness {
    block_header: Block,
    prev_block_header: Block,
    transactions: Vec<Transaction>,
    receipts: Vec<TransactionReceipt>,
    pre_state: HashMap<String, AccountState>,
    post_state: HashMap<String, AccountState>,
    merkle_proofs: HashMap<String, (AccountProof, AccountProof)>,
}

async fn rpc_batch(client: &Client, calls: Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>, ReqwestError> {
    let resp = client
        .post(RPC_URL)
        .json(&calls)
        .timeout(Duration::from_secs(10))
        .send()
        .await?
        .json::<Vec<serde_json::Value>>()
        .await?;
    Ok(resp)
}

async fn get_block(client: &Client, block_number: &str, full: bool) -> Result<Block, ReqwestError> {
    let params = json!([block_number, full]);
    let req = json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": params,
        "id": 1
    });
    let resp = client
        .post(RPC_URL)
        .json(&req)
        .timeout(Duration::from_secs(10))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(serde_json::from_value(resp["result"].clone()).unwrap())
}

async fn get_transaction_receipt(client: &Client, tx_hash: &str) -> Result<TransactionReceipt, ReqwestError> {
    let req = json!({
        "jsonrpc": "2.0",
        "method": "eth_getTransactionReceipt",
        "params": [tx_hash],
        "id": 1
    });
    let resp = client
        .post(RPC_URL)
        .json(&req)
        .timeout(Duration::from_secs(10))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(serde_json::from_value(resp["result"].clone()).unwrap())
}

async fn get_account_state(
    client: &Client,
    address: &str,
    prev_block: &str,
    curr_block: &str,
) -> Result<AccountState, ReqwestError> {
    let batch = vec![
        json!({"jsonrpc":"2.0","method":"eth_getBalance","params":[address, prev_block],"id":1}),
        json!({"jsonrpc":"2.0","method":"eth_getBalance","params":[address, curr_block],"id":2}),
        json!({"jsonrpc":"2.0","method":"eth_getTransactionCount","params":[address, prev_block],"id":3}),
        json!({"jsonrpc":"2.0","method":"eth_getTransactionCount","params":[address, curr_block],"id":4}),
        json!({"jsonrpc":"2.0","method":"eth_getCode","params":[address, curr_block],"id":5}),
    ];
    let resp = rpc_batch(client, batch).await?;
    Ok(AccountState {
        balance_before: resp[0]["result"].as_str().unwrap_or_default().to_string(),
        balance_after: resp[1]["result"].as_str().unwrap_or_default().to_string(),
        nonce_before: resp[2]["result"].as_str().unwrap_or_default().to_string(),
        nonce_after: resp[3]["result"].as_str().unwrap_or_default().to_string(),
        code: resp[4]["result"].as_str().unwrap_or_default().to_string(),
    })
}

async fn get_account_proof(
    client: &Client,
    address: &str,
    storage_keys: &[String],
    block: &str,
) -> Result<AccountProof, ReqwestError> {
    let req = json!({
        "jsonrpc": "2.0",
        "method": "eth_getProof",
        "params": [address, storage_keys, block],
        "id": 1
    });
    let resp = client
        .post(RPC_URL)
        .json(&req)
        .timeout(Duration::from_secs(10))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(serde_json::from_value(resp["result"].clone()).unwrap())
}

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