use serde::{Deserialize, Serialize};
use thiserror::Error;

use themelio_structs::{PoolKey, TxHash};



// ### EXTERNAL ERRORS ###
trait ExternalError {}
#[derive(Error, Debug, Deserialize, Serialize)]
#[error(transparent)]
pub struct MelnetError(#[from] pub melnet::MelnetError);
impl ExternalError for MelnetError{}

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
pub enum MelwalletdError<T: std::error::Error, J: ExternalError> {
    #[error(transparent)]
    External(#[from] J),

    #[error(transparent)]
    Internal(T),
}

pub type StateError<T> = MelwalletdError<T, MelnetError>;

 