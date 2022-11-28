use crate::types::{
    CreateWalletError, NeedWallet, NetworkError, PrepareTxArgs, PrepareTxError,
    SwapInfo, TransactionStatus, TxBalance, WalletAccessError, WalletSummary,
};

use async_trait::async_trait;
use nanorpc::nanorpc_derive;
use std::fmt::Debug;
use stdcode::SerializeAsString;
use themelio_structs::{
 BlockHeight, CoinData, CoinID, Denom, Header, Transaction, TxHash,
};
use themelio_structs::{PoolKey, PoolState};
use tmelcrypt::HashVal;


#[nanorpc_derive]
#[async_trait]
/// A [macro@nanorpc_derive] trait that describes the RPC protocol exposed by a `melwalletd` daemon.
///
/// **Note**: since the trait uses [macro@async_trait], the function signatures shown in the documentation are a little funky. The "human readable" versions are merely async methods; for example [MelwalletdProtocol::wallet_summary] should be implemented like
/// ```ignore
/// #[async_trait]
/// impl MelwalletdProtocol for YourBusinessLogic {
///     //...
///     async fn wallet_summary(&self, wallet_name: String)
///         -> Result<WalletSummary, WalletAccessError> {
///         todo!()
///     }
/// }
/// ```
pub trait MelwalletdProtocol: Send + Sync {
    /// Returns a list of wallet names.
    async fn list_wallets(&self) -> Vec<String>;

    /// Returns a summary of the overall state of the wallet. See [WalletSummary] for what that entails.
    async fn wallet_summary(
        &self,
        wallet_name: String,
    ) -> Result<WalletSummary, WalletAccessError>;

    /// Returns the latest blockchain header.
    async fn latest_header(&self) -> Result<Header, NetworkError>;

    /// Obtains up-to-date information about a particular melswap pool, identified by its [PoolKey]. Returns `None` if no such pool exists.
    async fn melswap_info(
        &self,
        pool_key: SerializeAsString<PoolKey>,
    ) -> Result<Option<PoolState>, NetworkError>;

    /// Simulates a swap between the two given [Denom]s, returning a [SwapInfo] that contains detailed information about the swap (such as price, slippage, etc)
    async fn simulate_swap(
        &self,
        to: SerializeAsString<Denom>,
        from: SerializeAsString<Denom>,
        value: u128,
    ) -> Result<Option<SwapInfo>, NetworkError>;

    /// Creates a wallet. If `secret` is provided, this must be a base32-encoded ed25519 private key.
    async fn create_wallet(
        &self,
        wallet_name: String,
        password: String,
        secret: Option<String>,
    ) -> Result<(), CreateWalletError>;

    /// Dump all the coins of a given wallet.
    async fn dump_coins(
        &self,
        wallet_name: String,
    ) -> Result<Vec<(CoinID, CoinData)>, WalletAccessError>;

    /// Dumps the transactions history of the given wallet.
    async fn dump_transactions(
        &self,
        wallet_name: String,
    ) -> Result<Vec<(TxHash, Option<BlockHeight>)>, WalletAccessError>;

    /// Locks the wallet.
    async fn lock_wallet(&self, wallet_name: String) -> Result<(), WalletAccessError>;

    /// Unlocks the given wallet. If the password is incorrect, will return [WalletAccessError::Locked].
    async fn unlock_wallet(
        &self,
        wallet_name: String,
        password: String,
    ) -> Result<(), WalletAccessError>;

    /// Exports the secret key, in the standard base32 format, from the given wallet. The password must be correct; if it is not, will return [WalletAccessError::Locked].
    async fn export_sk(
        &self,
        wallet_name: String,
        password: String,
    ) -> Result<String, WalletAccessError>;

    /// Prepares a transaction according to a template (see [PrepareTxArgs]). Returns a transaction that is ready for inspection.
    ///
    /// This method does not change the internal state of the wallet or send any transactions. Once you're sure you want to send the transaction returned, simply pass it to [MelwalletdProtocol::send_tx].
    async fn prepare_tx(
        &self,
        wallet_name: String,
        request: PrepareTxArgs,
    ) -> Result<Transaction, NeedWallet<PrepareTxError>>;

    /// Sends a transaction to the network, returning its hash. Note that the absence of an error does *not* mean that the transaction will certainly go through!
    async fn send_tx(
        &self,
        wallet_name: String,
        tx: Transaction,
    ) -> Result<TxHash, NeedWallet<NetworkError>>;

    /// Returns the "balance" (see [TxBalance]) of a transaction --- how much it increased or decreased the balance of the wallet. If such a transaction doesn't exist in the given wallet, returns `Ok(None)`.
    async fn tx_balance(
        &self,
        wallet_name: String,
        txhash: HashVal,
    ) -> Result<Option<TxBalance>, WalletAccessError>;

    /// Returns the status ([TransactionStatus]) of a transaction, which includes its full contents as well as where, if anywhere, was it confirmed. If no such transaction can be found, or if the wallet has given up on the transaction already, returns `Ok(None)`.
    async fn tx_status(
        &self,
        wallet_name: String,
        txhash: HashVal,
    ) -> Result<Option<TransactionStatus>, WalletAccessError>;

    /// A convenience method for sending 1000 MEL of a faucet transaction to the wallet itself.
    async fn send_faucet(&self, wallet_name: String) -> Result<TxHash, NeedWallet<NetworkError>>;
}
