use serde::{Deserialize, Serialize};

use themelio_structs::{BlockHeight, CoinData, Transaction};

#[derive(Serialize, Deserialize, Clone, Debug)]
/// The status of a transaction that is currently in progress.
pub struct TransactionStatus {
    pub raw: Transaction,
    pub confirmed_height: Option<BlockHeight>,
    pub outputs: Vec<AnnCoinID>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// An "annotated" CoinID that marks whether or not this coin is a change output or not.
pub struct AnnCoinID {
    pub coin_data: CoinData,
    pub is_change: bool,
    pub coin_id: String,
}
