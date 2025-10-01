use crate::types::IBlockData;

pub struct BlockHeader {
    pub hash: String,
    pub number: u64,
}

pub struct Transaction {
    pub hash: String,
    pub block_number: u64,
    pub from: String,
    pub to: String,
    pub value: u64,
}

pub struct BlockData;

impl IBlockData for BlockData {
    type BlockHeader = BlockHeader;
    type Transaction = Transaction;
}