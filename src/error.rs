use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use themelio_structs::{PoolKey, TxHash};

use crate::types::Melwallet;

// ### EXTERNAL ERRORS ###
trait ExternalError {}
#[derive(Error, Debug, Serialize, Deserialize)]
#[error("Melnet error")]
pub struct MelnetError(String);
impl ExternalError for MelnetError {}
impl From<melnet::MelnetError> for MelnetError {
    fn from(err: melnet::MelnetError) -> Self {
        MelnetError(err.to_string())
    }
}

// ### INTERNAL ERRORS ###

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Wallet could not be found")]
pub struct WalletNotFound;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Pool Key {0}")]
pub struct PoolKeyError(pub PoolKey);

// WALLET ERRORS

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Failed to create wallet: {0}")]
pub struct WalletCreationError(#[from] pub anyhow::Error);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("{0}")]
pub struct SecretKeyError(pub String);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Password")]
pub struct InvalidPassword;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Signature")]
pub struct InvalidSignature;

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

#[derive(Error, Serialize, Deserialize)]
pub enum MelwalletdError<T: std::error::Error, J: ExternalError + std::error::Error> {
    #[error(transparent)]
    Exo(J),

    #[error(transparent)]
    Endo(#[from] T),
}
// pub type Exo = MelwalletdError::Exo;
pub type StateError<T> = MelwalletdError<T, MelnetError>;

pub fn to_exo<K: ExternalError + std::error::Error, T: std::error::Error, J: Into<K>>(
    err: J,
) -> MelwalletdError<T, K> {
    MelwalletdError::Exo(err.into())
}
