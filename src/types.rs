use std::{collections::BTreeMap};


use serde::{Deserialize, Serialize};

use themelio_structs::{
    Address, CoinData, CoinID, CoinValue, Denom, NetID, TxKind,
};
use thiserror::Error;




#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("{0}")]
    NetworkError(#[from] themelio_nodeprot::ValClientError),
    #[error("{0}")]
    ExecutionError(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletSummary {
    pub total_micromel: CoinValue,
    pub detailed_balance: BTreeMap<String, CoinValue>,
    pub staked_microsym: CoinValue,
    pub network: NetID,
    #[serde(with = "stdcode::asstr")]
    pub address: Address,
    pub locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareTxArgs {
    #[serde(default)]
    pub kind: Option<TxKind>,
    pub inputs: Vec<CoinID>,
    pub outputs: Vec<CoinData>,
    #[serde(default, with = "stdcode::hexvec")]
    pub covenants: Vec<Vec<u8>>,
    pub data: Option<String>,
    #[serde(default)]
    pub nobalance: Vec<Denom>,
    #[serde(default)]
    pub fee_ballast: usize,
    pub signing_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SwapInfo {
    pub result: u128,
    pub price_impact: f64,
    pub poolkey: String,
}
#[derive(Serialize, Deserialize)]
pub struct TxBalance(pub bool, pub TxKind, pub BTreeMap<String, i128>);
