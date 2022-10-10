use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use futures::Future;
use serde::{Serialize, Deserialize};
use themelio_nodeprot::{ValClientSnapshot, ValClient};
use themelio_structs::{Transaction, Address, TxHash, Denom, CoinValue, BlockHeight, CoinID, CoinData, CoinDataHeight, NetID, TxKind, PoolKey, Header};
use tmelcrypt::Ed25519SK;

use crate::{error::InvalidPassword, signer::Signer};



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
pub trait MelwalletdHelpers<T: Melwallet> {
    async fn list_wallets(&self) -> BTreeMap<String, WalletSummary>;
    fn get_signer(&self, name: &str) -> Option<Arc<dyn Signer>>;
    fn unlock(&self, name: &str, pwd: Option<String>) -> Option<()>;
    fn get_secret_key(&self, name: &str, pwd: Option<String>) -> Result<Option<Ed25519SK>, InvalidPassword>;
    async fn get_wallet(&self, name: &str) -> Option<T>;
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
        fut_snapshot: impl Future<Output = anyhow::Result<ValClientSnapshot>>,
    ) -> Result<Option<Transaction>, melnet::MelnetError>;
    async fn get_cached_transaction(&self, txhash: TxHash) -> Option<Transaction>;
    async fn is_pending(&self, txhash: TxHash) -> bool;
    async fn get_balances(&self) -> BTreeMap<Denom, CoinValue>;
    async fn get_transaction_history(&self) -> Vec<(TxHash, Option<BlockHeight>)>;
    async fn get_coin_mapping(
        &self,
        confirmed: bool,
        ignore_pending: bool,
    ) -> BTreeMap<CoinID, CoinData>;
    async fn prepare(
        &self,
        inputs: Vec<CoinID>,
        outputs: Vec<CoinData>,
        fee_multiplier: u128,
        sign: impl Fn(Transaction) -> anyhow::Result<Transaction>,
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

