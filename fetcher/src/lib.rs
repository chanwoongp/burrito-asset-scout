pub mod ethereum;

#[derive(Debug)]
pub struct BlockHeader {
    pub number: Option<u64>,
    pub hash: Option<String>,
}
