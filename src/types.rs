use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    /// Method
    pub method: String,
    /// Params
    pub params: Value,
    /// Id
    pub id: i32,
    /// JsonRpc
    pub jsonrpc: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum JsonResult {
    /// String
    String(String),
    /// JsonResult
    JsonRpcResult(JsonRpcResult),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TransactionResult {
    /// String
    String(String),
    /// Transaction
    Transaction(Transaction),
}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResultJson {
    /// JsonResult
    JsonResult(JsonResult),
    /// LogsFilter
    LogsFilter(Vec<LogsFilter>)
}


#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JsonRpc {
    /// JSON RPC Version
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    /// Chain Id
    pub id: i32,
    /// Json RPC Results
    pub result: Option<ResultJson>,
    /// Json RPC Result Error
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JsonRpcResult {
    /// base fee per gas
    #[serde(rename = "baseFeePerGas")]
    pub base_fee_per_gas: Option<String>,
    /// difficulty
    pub difficulty: Option<String>,
    /// extra Data
    #[serde(rename = "extraData")]
    pub extra_data: Option<String>,
    /// gas Limit
    #[serde(rename = "gasLimit")]
    pub gas_limit: Option<String>,
    /// gas Used
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<String>,
    /// hash
    pub hash: Option<String>,
    /// logs Bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<String>,
    /// miner
    pub miner: Option<String>,
    /// mix hash
    #[serde(rename = "mixHash")]
    pub mix_hash: Option<String>,
    /// nonce
    pub nonce: Option<String>,
    /// number
    pub number: Option<String>,
    /// parent Hash
    #[serde(rename = "parentHash")]
    pub parent_hash: Option<String>,
    /// receipts Root
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: Option<String>,
    /// sha3Uncles
    #[serde(rename = "sha3Uncles")]
    pub sha3_uncles: Option<String>,
    /// size
    pub size: Option<String>,
    /// stateRoot
    #[serde(rename = "stateRoot")]
    pub state_root: Option<String>,
    /// timestamp
    pub timestamp: Option<String>,
    /// totalDifficulty
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: Option<String>,
    /// transactions
    pub transactions: Option<Vec<TransactionResult>>,
    /// transactionsRoot
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: Option<String>,
    /// uncles
    pub uncles: Option<Vec<String>>,
    /// blockHash
    #[serde(rename = "blockHash")]
    pub block_hash: Option<String>,
    /// blockNumber
    #[serde(rename = "blockNumber")]
    pub block_number: Option<String>,
    /// contractAddress
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
    /// cumulativeGasUsed
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: Option<String>,
    /// effectiveGasPrice
    #[serde(rename = "effectiveGasPrice")]
    pub effective_gas_price: Option<String>,
    /// from
    pub from: Option<String>,
    /// logs
    pub logs: Option<Vec<TransactionReceiptLogs>>,
    /// status
    pub status: Option<String>,
    /// to
    pub to: Option<String>,
    /// transactionHash
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    /// transactionIndex
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<String>,
    /// type
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Transaction {
    /// Block Hash
    #[serde(rename = "blockHash")]
    pub block_hash: Option<String>,
    /// Block Number
    #[serde(rename = "blockNumber")]
    pub block_number: Option<String>,
    /// from
    pub from: Option<String>,
    /// gas
    pub gas: Option<String>,
    /// gas Price
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<String>,
    /// max fee per gas
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<String>,
    ///max Priority Fee Per Gas
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<String>,
    /// hash
    pub hash: Option<String>,
    /// input
    pub input: Option<String>,
    /// nonce
    pub nonce: Option<String>,
    /// to
    pub to: Option<String>,
    /// transactionIndex
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<String>,
    /// value
    pub value: Option<String>,
    /// type
    pub r#type: Option<String>,
    /// accessList
    #[serde(rename = "accessList")]
    pub access_list: Option<Vec<AccessList>>,
    /// chainId
    #[serde(rename = "chainId")]
    pub chain_id: Option<String>,
    /// v
    pub v: Option<String>,
    /// r
    pub r: Option<String>,
    /// s
    pub s: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionReceiptLogs {
    /// address
    pub address: Option<String>,
    /// topics
    pub topics: Option<Vec<String>>,
    /// data
    pub data: Option<String>,
    /// blockNumber
    #[serde(rename = "blockNumber")]
    pub block_number: Option<String>,
    /// transactionHash
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    /// transactionIndex
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<String>,
    /// blockHash
    #[serde(rename = "blockHash")]
    pub block_hash: Option<String>,
    /// logIndex
    #[serde(rename = "logIndex")]
    pub log_index: Option<String>,
    /// removed
    pub removed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AccessList {
    /// Address
    pub address: String,
    /// storage Keys
    #[serde(rename = "storageKeys")]
    pub storage_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LogsFilter {
    /// Address
    pub address: Option<String>,
    /// Topics
    pub topics: Option<Vec<String>>,
    /// Data
    pub data: Option<String>,
    /// Block Number
    #[serde(rename="blockNumber")]
    pub block_number: Option<String>,
    /// transactionHash
    #[serde(rename="transactionHash")]
    pub transaction_hash: Option<String>,
    /// transactionIndex
    #[serde(rename="transactionIndex")]
    pub transaction_index: Option<String>,
    /// blockHash
    #[serde(rename="blockHash")]
    pub block_hash: Option<String>,
    /// logIndex
    #[serde(rename="logIndex")]
    pub log_index: Option<String>,
    /// Removed
    pub removed: Option<bool>
}
