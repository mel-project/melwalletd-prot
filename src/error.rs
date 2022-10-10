use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::error::Error as StdError;
use themelio_structs::{PoolKey, TxHash};

// ### EXTERNAL ERRORS ###
pub trait ExternalError {}
#[derive(Error, Debug, Serialize, Deserialize)]
#[error("Melnet error")]
pub struct MelnetError(pub String);
impl ExternalError for MelnetError {}
impl From<melnet::MelnetError> for MelnetError {
    fn from(err: melnet::MelnetError) -> Self {
        MelnetError(err.to_string())
    }
}

// ### INTERNAL ERRORS ###

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Pool Key {0}")]
pub struct PoolKeyError(pub PoolKey);

// WALLET ERRORS

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Failed to create wallet: {0}")]
pub struct WalletCreationError(pub String);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("{0}")]
pub struct SecretKeyError(pub String);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Password")]
pub struct InvalidPassword;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Signature")]
pub struct InvalidSignature;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Signature")]
pub struct FailedUnlock;

//TRANSACTION ERRORS

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Cannot find transaction {0}")]
pub struct TransactionNotFound(pub TxHash);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Cannot submit faucet transaction on this network")]
pub struct InvalidFaucetTransaction;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Lost transaction {0}, no longer pending but not confirmed; probably gave up")]
pub struct LostTransaction(pub TxHash);

// #### COMPOUND ERRORS ####

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum NeverError {
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CreateWalletError {
    #[error(transparent)]
    SecretKeyError(#[from] SecretKeyError),
    #[error(transparent)]
    WalletCreationError(#[from] WalletCreationError),
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum PrepareTxError {
    #[error(transparent)]
    InvalidSignature(#[from] InvalidSignature),
    #[error(transparent)]
    FailedUnlock(#[from] FailedUnlock),
}

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum TransactionError {
    #[error("Cannot find transaction {0}")]
    NotFound(TxHash),

    #[error("Cannot submit faucet transaction on this network")]
    InvalidFaucet,

    #[error("Lost transaction {0}, no longer pending but not confirmed; probably gave up")]
    Lost(TxHash),
}

// ### Useful monads ###

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum NeedWallet<T: StdError> {
    #[error("Couldn't find wallet: {0}")]
    NotFound(String),
    #[error(transparent)]
    Other(#[from] T),
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ProtocolError<T: std::error::Error, J: ExternalError + StdError> {
    #[error("{0}")]
    BadRequest(String),

    #[error(transparent)]
    Exo(J),

    #[error(transparent)]
    Endo(#[from] T),
}

#[derive(Error, Serialize, Deserialize)]
pub enum MelwalletdError<T: std::error::Error> {
    #[error("{0}")]
    BadRequest(String),

    #[error(transparent)]
    Error(#[from] T),
}

// pub type Exo = ProtocolError::Exo;
pub type StateError<T> = ProtocolError<T, MelnetError>;

pub fn to_exo<K: ExternalError + std::error::Error, T: std::error::Error, J: Into<K>>(
    err: J,
) -> ProtocolError<T, K> {
    ProtocolError::Exo(err.into())
}

pub fn default_melnet_error() -> melnet::MelnetError {
    melnet::MelnetError::Custom("Default error".into())
}
