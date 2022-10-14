use serde::{Deserialize, Serialize};
use themelio_nodeprot::{NodeRpcError, ValClientError};
use thiserror::Error;

use std::{error::Error as StdError, fmt::Display};
use themelio_structs::{PoolKey, TxHash};
// ### EXTERNAL ERRORS ###
#[derive(Error, Debug, Serialize, Deserialize)]
#[error("{0}")]
pub struct NetworkError(pub String);


pub fn to_network<T: Display>(e: T) -> NetworkError {
    NetworkError(e.to_string())
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
pub enum NeverError {}
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

    #[error("Failed to send transaction: {0}")]
    SendFailed(String),
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
pub enum ProtocolError<T: StdError, K: StdError> {
    #[error("{0}")]
    BadRequest(String),

    #[error(transparent)]
    Exo(K),

    #[error(transparent)]
    Endo(T),
}



impl <T: StdError> From<ValClientError> for ProtocolError<T,NetworkError> {
    fn from(e: ValClientError) -> Self {
        ProtocolError::Exo(to_network(e))
    }
}

impl <T: Display + StdError, J: StdError> From<NodeRpcError<T>> for ProtocolError<J, NetworkError> {
    fn from(e: NodeRpcError<T>) -> Self {
        ProtocolError::Exo(to_network(e))
    }
}

// pub type Exo = ProtocolError::Exo;
pub type StateError<T> = ProtocolError<T, NetworkError>;

pub fn to_exo<T: StdError, K: StdError>(err: K) -> ProtocolError<T, K> {
    ProtocolError::Exo(err)
}

pub fn to_endo<T: StdError, K: StdError>(err: T) -> ProtocolError<T, K> {
    ProtocolError::Endo(err)
}

pub fn to_network_exo<T: StdError, K: Display>(err: K) -> ProtocolError<T, NetworkError> {
    ProtocolError::Exo(to_network(err.to_string()))
}