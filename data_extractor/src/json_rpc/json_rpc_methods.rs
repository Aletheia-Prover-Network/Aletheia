use serde_json::json;

use std::time::Duration;

use reqwest::{Client, Error as ReqwestError};
use crate::types::types::{AccountProof, Block, Transaction, TransactionOrHash, TransactionReceipt, AccountState, BlockWitness};


const RPC_URL: &str = "http://127.0.0.1:8545";



pub async fn rpc_batch(client: &Client, calls: Vec<serde_json::Value>) -> Result<Vec<serde_json::Value>, ReqwestError> {
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

pub async fn get_block(client: &Client, block_number: &str, full: bool) -> Result<Block, ReqwestError> {
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

pub async fn get_transaction_receipt(client: &Client, tx_hash: &str) -> Result<TransactionReceipt, ReqwestError> {
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

pub async fn get_account_state(
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

pub async fn get_account_proof(
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