use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use themelio_nodeprot::{ValClient, ValClientSnapshot};
use themelio_structs::{
    Address, BlockHeight, CoinData, CoinDataHeight, CoinID, CoinValue, Denom, NetID, Transaction,
    TxHash, TxKind,
};
use thiserror::Error;
use tmelcrypt::Ed25519SK;

use crate::{error::InvalidPassword, signer::Signer};

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

#[async_trait]
/// Used by MelwalletProtocol
pub trait MelwalletdHelpers {
    async fn list_wallets(&self) -> BTreeMap<String, WalletSummary>;
    fn get_signer(&self, name: &str) -> Option<Arc<dyn Signer>>;
    fn unlock(&self, name: &str, pwd: Option<String>) -> Option<()>;
    fn get_secret_key(
        &self,
        name: &str,
        pwd: Option<String>,
    ) -> Result<Option<Ed25519SK>, InvalidPassword>;
    async fn get_wallet(&self, name: &str) -> Option<Box<dyn Melwallet + Send + Sync>>;
    fn lock(&self, name: &str);
    async fn create_wallet(
        &self,
        name: &str,
        key: Ed25519SK,
        pwd: Option<String>,
    ) -> anyhow::Result<()>;
    fn client(&self) -> ValClient;
    fn get_network(&self) -> NetID;
}

#[async_trait]
pub trait Melwallet {
    fn address(&self) -> Address;
    async fn get_transaction(
        &self,
        txhash: TxHash,
        snapshot: ValClientSnapshot,
    ) -> Result<Option<Transaction>, DatabaseError>;
    async fn get_cached_transaction(&self, txhash: TxHash) -> Option<Transaction>;
    async fn is_pending(&self, txhash: TxHash) -> bool;
    async fn get_balances(&self) -> BTreeMap<Denom, CoinValue>;
    async fn get_transaction_history(&self) -> Vec<(TxHash, Option<BlockHeight>)>;
    async fn get_coin_mapping(
        &self,
        confirmed: bool,
        ignore_pending: bool,
    ) -> BTreeMap<CoinID, CoinData>;

    #[allow(clippy::too_many_arguments)]
    async fn prepare(
        &self,
        inputs: Vec<CoinID>,
        outputs: Vec<CoinData>,
        fee_multiplier: u128,
        sign: Arc<Box<dyn Fn(Transaction) -> anyhow::Result<Transaction> + Send + Sync>>,
        nobalance: Vec<Denom>,
        fee_ballast: usize,

        snap: ValClientSnapshot,
    ) -> anyhow::Result<Transaction>;
    async fn commit_sent(&self, txn: Transaction, timeout: BlockHeight) -> anyhow::Result<()>;
    async fn get_one_coin(&self, coin_id: CoinID) -> Option<CoinData>;
    async fn get_coin_confirmation(&self, coin_id: CoinID) -> Option<CoinDataHeight>;
    async fn network_sync(&self, snapshot: ValClientSnapshot) -> anyhow::Result<()>;
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
pub struct PoolInfo {
    pub result: u128,
    pub price_impact: f64,
    pub poolkey: String,
}
#[derive(Serialize, Deserialize)]
pub struct TxBalance(pub bool, pub TxKind, pub BTreeMap<String, i128>);
