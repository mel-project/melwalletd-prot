use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use futures::Future;
use serde::{Serialize, Deserialize};
use themelio_nodeprot::{ValClientSnapshot, ValClient};
use themelio_stf::melvm::Covenant;
use themelio_structs::{Transaction, Address, TxHash, Denom, CoinValue, BlockHeight, CoinID, CoinData, CoinDataHeight, NetID, TxKind, PoolKey};
use thiserror::Error;
use tmelcrypt::Ed25519SK;

/// This trait is implemented by anything "secret key-like" that can sign a transaction. This includes secret keys, password-encumbered secret keys,
pub trait Signer: Send + Sync + 'static {
    /// Given a transaction, returns the signed version. Signing may fail (e.g. due to communication failure).
    fn sign_tx(&self, tx: Transaction, input_idx: usize) -> anyhow::Result<Transaction>;

    /// Covenant that checks for transactions signed with this Signer.
    fn covenant(&self) -> Covenant;
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
pub trait MelwalletdHelpers<T: Melwallet> {
    async fn list_wallets(&self) -> BTreeMap<String, WalletSummary>;
    fn get_signer(&self, name: &str) -> Option<Arc<dyn Signer>>;
    fn unlock(&self, name: &str, pwd: Option<String>) -> Option<()>;
    fn get_secret_key(&self, name: &str, pwd: Option<String>) -> Option<Ed25519SK>;
    async fn get_wallet(&self, name: &str) -> Option<T>;
    fn lock(&self, name: &str);
    async fn create_wallet(
            &self,
            name: &str,
            key: Ed25519SK,
            pwd: Option<String>,
        ) -> anyhow::Result<()>;
    async fn client() -> ValClient;
}

#[async_trait]
pub trait Melwallet {
    fn address(&self) -> Address;
    async fn get_transaction(
        &self,
        txhash: TxHash,
        fut_snapshot: impl Future<Output = anyhow::Result<ValClientSnapshot>>,
    ) -> Result<Option<Transaction>, RequestErrors>;
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
    kind: Option<TxKind>,
    inputs: Vec<CoinID>,
    outputs: Vec<CoinData>,
    #[serde(default, with = "stdcode::hexvec")]
    covenants: Vec<Vec<u8>>,
    data: Option<String>,
    #[serde(default)]
    nobalance: Vec<Denom>,
    #[serde(default)]
    fee_ballast: usize,
    signing_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PoolInfo {
    result: u128,
    price_impact: f64,
    poolkey: String,
}
#[derive(Serialize, Deserialize)]
pub struct TxBalance(bool, TxKind, BTreeMap<String, i128>);

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum RequestErrors {
    #[error("Wallet could not be found")]
    WalletNotFound,
    #[error("Bad request")]
    BadRequest(String),
    #[error("Invalid Pool Key {0}")]
    PoolKeyError(PoolKey),
    #[error("Invalid Password")]
    InvalidPassword,
    #[error("Invalid Signature")]
    InvalidSignature,
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError<DbError>),
    #[error("Http error {0}")]
    HttpStatusError(http_types::StatusCode),
    #[error("Failed to unlock wallet {0}")]
    FailedUnlock(String),
    #[error("Cannot find transaction {0}")]
    TransactionNotFound(TxHash),
    #[error("Cannot submit faucet transaction on this network")]
    InvalidFaucetTransaction,
    #[error("Lost transaction {0}, no longer pending but not confirmed; probably gave up")]
    LostTransaction(TxHash),
    #[error("Failed to create wallet: {0}")]
    WalletCreationError(String),
}

