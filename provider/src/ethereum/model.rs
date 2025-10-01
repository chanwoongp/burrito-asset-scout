use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetBlockByNumberResponse {
    #[serde(rename = "baseFeePerGas")]
    pub base_fee_per_gas: Option<String>,
    #[serde(rename = "blobGasUsed")]
    pub blob_gas_used: Option<String>,
    pub difficulty: String,
    #[serde(rename = "excessBlobGas")]
    pub excess_blob_gas: Option<String>,
    #[serde(rename = "extraData")]
    pub extra_data: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    pub hash: String,
    #[serde(rename = "logsBloom")]
    pub logs_bloom: String,
    pub miner: String,
    #[serde(rename = "mixHash")]
    pub mix_hash: String,
    pub nonce: String,
    pub number: String,
    #[serde(rename = "parentBeaconBlockRoot")]
    pub parent_beacon_block_root: Option<String>,
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: String,
    #[serde(rename = "requestsHash")]
    pub requests_hash: Option<String>,
    pub size: String,
    #[serde(rename = "stateRoot")]
    pub state_root: String,
    pub timestamp: String,
    pub transactions: Vec<GetTransactionByHashResponse>,
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetTransactionByHashResponse {
    #[serde(rename = "blockHash")]
    pub block_hash: Option<String>,
    #[serde(rename = "blockNumber")]
    pub block_number: Option<String>,
    pub from: String,
    pub gas: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<String>,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: Option<String>,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: Option<String>,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    pub to: Option<String>,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<String>,
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "chainId")]
    pub chain_id: Option<String>,
    pub v: String,
    #[serde(rename = "yParity")]
    pub y_parity: Option<String>,
    pub r: String,
    pub s: String,
}
