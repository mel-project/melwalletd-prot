use crate::error::{
    self, InvalidPassword, NeedWallet, NeverError, PoolKeyError, StateError, TransactionError,
};

use crate::request_errors::{CreateWalletError, PrepareTxError};
use crate::types::{PoolInfo, PrepareTxArgs, TxBalance, WalletSummary};
use crate::walletdata::TransactionStatus;
use async_trait::async_trait;
use nanorpc::nanorpc_derive;
use std::fmt::Debug;
use themelio_structs::{BlockHeight, CoinData, CoinID, Denom, Transaction, TxHash};
use themelio_structs::{Header, PoolKey, PoolState};
use tmelcrypt::HashVal;

#[nanorpc_derive]
#[async_trait]
pub trait MelwalletdProtocol: Send + Sync {
    async fn summarize_wallet(
        &self,
        wallet_name: String,
    ) -> Result<WalletSummary, NeedWallet<NeverError>>;
    async fn get_summary(&self) -> Result<Header, error::NetworkError>;
    async fn get_pool(
        &self,
        pool_key: PoolKey,
    ) -> Result<Option<PoolState>, StateError<PoolKeyError>>;
    async fn simulate_pool_swap(
        &self,
        to: Denom,
        from: Denom,
        value: u128,
    ) -> Result<Option<PoolInfo>, StateError<PoolKeyError>>;
    async fn create_wallet(
        &self,
        wallet_name: String,
        password: Option<String>,
        secret: Option<String>,
    ) -> Result<(), CreateWalletError>;
    async fn dump_coins(
        &self,
        wallet_name: String,
    ) -> Result<Vec<(CoinID, CoinData)>, NeedWallet<NeverError>>;
    async fn dump_transactions(
        &self,
        wallet_name: String,
    ) -> Result<Vec<(TxHash, Option<BlockHeight>)>, NeedWallet<NeverError>>;
    async fn lock_wallet(&self, wallet_name: String);
    async fn unlock_wallet(
        &self,
        wallet_name: String,
        password: Option<String>,
    ) -> Result<(), InvalidPassword>;
    async fn export_sk_from_wallet(
        &self,
        wallet_name: String,
        password: Option<String>,
    ) -> Result<Option<String>, InvalidPassword>;
    async fn prepare_tx(
        &self,
        wallet_name: String,
        request: PrepareTxArgs,
    ) -> Result<Transaction, StateError<NeedWallet<PrepareTxError>>>;
    async fn send_tx(
        &self,
        wallet_name: String,
        tx: Transaction,
    ) -> Result<TxHash, StateError<NeedWallet<NeverError>>>;
    async fn get_tx_balance(
        &self,
        wallet_name: String,
        txhash: HashVal,
    ) -> Result<TxBalance, StateError<NeedWallet<TransactionError>>>;
    async fn get_tx(
        &self,
        wallet_name: String,
        txhash: HashVal,
    ) -> Result<TransactionStatus, StateError<NeedWallet<TransactionError>>>;
    async fn send_faucet(
        &self,
        wallet_name: String,
    ) -> Result<TxHash, StateError<NeedWallet<TransactionError>>>;
}
