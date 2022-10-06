use serde::{Deserialize, Serialize};
use thiserror::Error;

use themelio_structs::{PoolKey, TxHash};

#[derive(Error, Debug, Deserialize, Serialize)]
#[error(transparent)]
pub struct ValClientError(#[from] pub melnet::MelnetError);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Wallet could not be found")]
pub struct WalletNotFound;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Bad request")]
pub struct BadRequest(pub String);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Pool Key {0}")]
pub struct PoolKeyError(pub PoolKey);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Password")]
pub struct InvalidPassword;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Signature")]
pub struct InvalidSignature;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error(transparent)]
pub struct DatabaseError(pub String);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Http error {0}")]
pub struct HttpStatusError(pub http_types::StatusCode);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Failed to unlock wallet {0}")]
pub struct FailedUnlock(pub String);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Cannot find transaction {0}")]
pub struct TransactionNotFound(pub TxHash);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Cannot submit faucet transaction on this network")]
pub struct InvalidFaucetTransaction;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Lost transaction {0}, no longer pending but not confirmed; probably gave up")]
pub struct LostTransaction(pub TxHash);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Failed to create wallet: {0}")]
pub struct WalletCreationError(pub String);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error(transparent)]
pub struct MelnetError(#[from] pub melnet::MelnetError);

/// rpc method errors

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GetPoolError {
    #[error(transparent)]
    PoolKeyError(#[from] PoolKeyError),
}
