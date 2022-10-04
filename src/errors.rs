use serde::{Deserialize, Serialize};
use themelio_structs::{PoolKey, TxHash};
use thiserror::Error;



#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Wallet could not be found")]
pub struct WalletNotFound;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Bad request")]
pub struct BadRequest(String);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Pool Key {0}")]
pub struct PoolKeyError(PoolKey);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Password")]
pub struct InvalidPassword;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Invalid Signature")]
pub struct InvalidSignature;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error(transparent)]
pub struct DatabaseError(String);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Http error {0}")]
pub struct HttpStatusError(http_types::StatusCode);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Failed to unlock wallet {0}")]
pub struct FailedUnlock(String);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Cannot find transaction {0}")]
pub struct TransactionNotFound(TxHash);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Cannot submit faucet transaction on this network")]
pub struct InvalidFaucetTransaction;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Lost transaction {0}, no longer pending but not confirmed; probably gave up")]
pub struct LostTransaction(TxHash);

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Failed to create wallet: {0}")]
pub struct WalletCreationError(String);

