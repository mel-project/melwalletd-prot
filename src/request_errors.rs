use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::error;


#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CreateWalletError{
    #[error(transparent)]
    SecretKeyError(#[from] error::SecretKeyError),
    #[error(transparent)]
    WalletCreationError(#[from] error::WalletCreationError),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum PrepareTxError{
    #[error(transparent)]
    InvalidSignature(#[from] error::InvalidSignature),
    #[error(transparent)]
    FailedUnlock(#[from] error::FailedUnlock),
    
}