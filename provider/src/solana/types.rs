use crate::types::IBlockData;

pub struct BlockHeader {
    pub block_height: u64,
    pub block_time: u64,
    pub blockhash: String,
    pub parent_lot: u64,
    pub previous_blockhash: String,
}

pub struct Transaction {
    pub block_time: u64,
    pub slot: u64,
}

pub struct BlockData;

impl IBlockData for BlockData {
    type BlockHeader = BlockHeader;
    type Transaction = Transaction;
}
