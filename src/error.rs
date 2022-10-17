use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::error::Error as StdError;
use themelio_structs::{CoinValue, Denom, PoolKey, TxHash};

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

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum TransactionError {
    #[error("Cannot find transaction {0}")]
    NotFound(TxHash),

    #[error("Cannot submit faucet transaction on this network")]
    InvalidFaucet,

    #[error("Lost transaction {0}, no longer pending but not confirmed; probably gave up")]
    Lost(TxHash),

    #[error("Failed to send transaction: {0}")]
    SendFailed(String),
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
