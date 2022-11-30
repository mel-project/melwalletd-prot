use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use serde_with::serde_as;
use stdcode::SerializeAsString;
use themelio_structs::{
    Address, BlockHeight, CoinData, CoinID, CoinValue, Denom, NetID, Transaction, TxKind, PoolKey,
};
use thiserror::Error;

use std::error::Error as StdError;

#[derive(Error, Debug, Serialize, Deserialize)]
/// Indicates a problem with accessing the wallet
pub enum WalletAccessError {
    #[error("wallet not found")]
    NotFound,
    #[error("wallet needs to be unlocked")]
    Locked,
    #[error("other problem accessing wallet: {0}")]
    Other(String),
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CreateWalletError {
    #[error("error parsing secret key: {0}")]
    SecretKey(String),
    #[error("wallet already exists")]
    WalletExists,
    #[error("other error creating wallet: {0}")]
    Other(String),
}

#[derive(Error, Debug, Serialize, Deserialize)]
/// The error type returned by [crate::MelwalletdProtocol::prepare_tx].
pub enum PrepareTxError {
    #[error("not enough money ({0} more of {1} needed)")]
    InsufficientFunds(CoinValue, Denom),

    #[error("cannot access external input coin {0}")]
    BadExternalInput(CoinID),

    #[error("could not access network: {0}")]
    Network(NetworkError),
}

#[derive(Error, Debug, Serialize, Deserialize)]
/// The error type returned by [crate::MelwalletdProtocol::send_tx].
pub enum SendTxError {
    #[error("could not access wallet: {0}")]
    Wallet(WalletAccessError),

    #[error("could not access network: {0}")]
    Network(NetworkError),
}

/// Either a wallet error, or some other kind of error. The JSON representation is ["untagged"](https://serde.rs/enum-representations.html#untagged).
#[derive(Error, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NeedWallet<T: StdError> {
    #[error("wallet access error: {0}")]
    Wallet(WalletAccessError),
    #[error(transparent)]
    Other(#[from] T),
}

/// A network-caused, possibly transient state-access error.
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum NetworkError {
    #[error("transient network error: {0}")]
    Transient(String),
    #[error("fatal network error: {0}")]
    Fatal(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// A JSON-serializable summary of the entire wallet, generally corresponding to whatever is displayed on the "front page" of the wallet.
pub struct WalletSummary {
    /// Total MEL balance. JSON-serialized as micromels.
    pub total_micromel: CoinValue,
    /// Detailed balance. Keys are the standard (Display/FromStr) string representation of a [Denom].
    pub detailed_balance: BTreeMap<SerializeAsString<Denom>, CoinValue>,
    /// SYM that is inaccessible due to being staked.
    pub staked_microsym: CoinValue,
    #[serde(with = "stdcode::asstr")]
    /// Network ID (mainnet, testnet, etc). JSON-serialized as the corresponding integer.
    pub network: NetID,
    /// Address of this wallet. JSON-serialized as the standard `t.....` address format.
    #[serde(with = "stdcode::asstr")]
    pub address: Address,
    /// Whether this wallet is locked. Locked wallets generally cannot be interacted with.
    pub locked: bool,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
/// Arguments passed to [crate::MelwalletdProtocol::prepare_tx]. Configures what sort of transaction to construct.
pub struct PrepareTxArgs {
    #[serde(default = "txkind_normal")]
    #[serde(with = "stdcode::asstr")]
    /// "Kind" of the transaction. Optional in JSON, defaulting to [TxKind::Normal].
    pub kind: TxKind,
    /// **Additional** inputs of the transaction. Normally, this field can be left as an empty vector, in which case UTXOs locked by the wallet's own address are picked automatically.
    ///
    /// Use this field to specify "out of wallet" coins from dapps, multisig vaults, and such, which do not have their `covhash` field equal to the [Address] of the wallet, yet the wallet is able to spend, possibly in combination with other fields of [PrepareTxArgs]. For example, a multisig coin would not have the [Address] of any single-key wallet, and spending it must require explicitly specifying its [CoinID] and explicitly passing unlock arguments.
    ///
    /// Optional in JSON, in which case it defaults to an empty list.
    #[serde(default)]
    pub inputs: Vec<CoinID>,
    /// **Required** outputs of the transaction. This generally specifies the "recipients" of the transaction. Note that this only specifies the first outputs of the transaction; more outputs may be created as "change" outputs.
    pub outputs: Vec<CoinData>,
    /// **Additional** covenants that must be included in the transaction. This is needed when spending out-of-wallet coins. Optional in JSON, defaulting to an empty list.
    #[serde(default, with = "stdcode::hexvec")]
    pub covenants: Vec<Vec<u8>>,
    /// The "data" field of the transaction. Optional and hex-encoded in JSON, defaulting to an empty string.
    #[serde(default, with = "stdcode::hex")]
    pub data: Vec<u8>,
    #[serde(default)]
    /// The "data" field of the transaction. Optional and hex-encoded in JSON, defaulting to an empty string.
    pub nobalance: Vec<Denom>,
    #[serde(default)]
    /// Pretend like the transaction has this many more bytes when calculating the correct fee level. Useful in niche situations where you want to intentionally pay more fees than necessary.
    pub fee_ballast: usize,
}

fn txkind_normal() -> TxKind {
    TxKind::Normal
}

#[derive(Serialize, Deserialize)]
/// Returned from [crate::MelwalletdProtocol::simulate_swap], encoding information about a simulated Melswap swap.
pub struct SwapInfo {
    /// How many units of the "other" token will the swap obtain
    pub result: u128,
    /// Impact to the price, as a fraction
    pub slippage: u128,
    #[serde(with = "stdcode::asstr")]
    pub poolkey: PoolKey,
}
#[derive(Serialize, Deserialize)]
/// A tuple including:
/// - whether the transaction is spent
/// - the kind of the transaction
/// - a mapping between string-represented [Denom]s and how much the balance of that [Denom] changed in this transaction.
pub struct TxBalance(
    pub bool,
    #[serde(with = "stdcode::asstr")] 
    pub TxKind,
    pub BTreeMap<SerializeAsString<Denom>, i128>,
);

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
