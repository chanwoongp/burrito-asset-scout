use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetBlockByNumberResponse {
    #[serde(rename = "baseFeePerGas")]
    pub base_fee_per_gas: String,
    #[serde(rename = "blobGasUsed")]
    pub blob_gas_used: String,
    pub difficulty: String,
    #[serde(rename = "excessBlobGas")]
    pub excess_blob_gas: String,
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
    pub parent_beacon_block_root: String,
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: String,
    #[serde(rename = "requestsHash")]
    pub requests_hash: String,
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
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    pub from: String,
    pub gas: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: String,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: String,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: String,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    pub to: String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String,
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    pub v: String,
    #[serde(rename = "yParity")]
    pub y_parity: String,
    pub r: String,
    pub s: String,
}