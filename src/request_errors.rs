use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::error;


#[derive(Error, Debug, Serialize, Deserialize)]
pub enum SummarizeWalletError{
    #[error(transparent)]
    WalletNotFound(#[from] error::WalletNotFound),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GetSummaryError{
    #[error(transparent)]
    MelnetError(#[from] melnet::MelnetError),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GetPoolError{
    #[error(transparent)]
    PoolKeyError(#[from] error::PoolKeyError),
    #[error(transparent)]
    MelnetError(#[from] melnet::MelnetError),
    #[error(transparent)]
    BadRequest(#[from] error::BadRequest),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GetPoolInfoError{
    #[error(transparent)]
    BadRequest(#[from] error::BadRequest),
    #[error(transparent)]
    MelnetError(#[from] melnet::MelnetError),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CreateWalletError{
    #[error(transparent)]
    WalletCreationError(#[from] error::WalletCreationError),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum DumpCoinsError{
    #[error(transparent)]
    WalletNotFound(#[from] error::WalletNotFound),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum DumpTransactionsError{
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum UnlockWalletError{
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ExportSkFromWalletError{
    
}
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