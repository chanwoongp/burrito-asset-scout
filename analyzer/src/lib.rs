pub mod native_coin_transfer;
pub mod token_transfer;

pub trait Analyzer {
    fn analyze(&self);
}
