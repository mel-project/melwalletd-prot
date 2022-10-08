use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::error;


#[derive(Error, Debug, Serialize, Deserialize)]
pub enum PrepareTxError{
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SendTxError{
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GetTxBalanceError{
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GetTxError{
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SendFaucetError{
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum PoolError{
    #[error(transparent)]
    PoolKeyError(#[from] error::PoolKeyError),
    #[error(transparent)]
    MelnetError(#[from] melnet::MelnetError),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CreateWalletError{
    #[error(transparent)]
    SecretKeyError(#[from] error::SecretKeyError),
    #[error(transparent)]
    WalletCreationError(#[from] error::WalletCreationError),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum DumpCoinsError{
    #[error(transparent)]
    WalletNotFound(#[from] error::WalletNotFound),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ExportSkFromWalletError{
    #[error(transparent)]
    InvalidPassword(#[from] error::InvalidPassword),
    
}