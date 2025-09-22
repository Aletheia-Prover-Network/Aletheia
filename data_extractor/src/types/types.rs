use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub number: String,
    pub hash: String,
    pub parentHash: String,
    pub stateRoot: String,
    pub transactions: Vec<TransactionOrHash>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: Option<String>,
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TransactionOrHash {
    Full(Transaction),
    Hash(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub transactionHash: String,
    pub gasUsed: String,
    pub status: String,
    pub logs: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountProof {
    pub address: String,
    pub balance: String,
    pub nonce: String,
    pub codeHash: String,
    pub storageHash: String,
    pub accountProof: Vec<String>,
    pub storageProof: Vec<StorageProof>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageProof {
    pub key: String,
    pub value: String,
    pub proof: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountState {
    pub balance_before: String,
    pub balance_after: String,
    pub nonce_before: String,
    pub nonce_after: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockWitness {
    pub block_header: Block,
    pub prev_block_header: Block,
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<TransactionReceipt>,
    pub pre_state: HashMap<String, AccountState>,
    pub post_state: HashMap<String, AccountState>,
    pub merkle_proofs: HashMap<String, (AccountProof, AccountProof)>,
}