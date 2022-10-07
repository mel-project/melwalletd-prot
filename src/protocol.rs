use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::error::{self, MelnetError, ValClientError, WalletNotFound, InvalidPassword, StateError, PoolKeyError};
use crate::request_errors::{
    self, CreateWalletError, DumpCoinsError, DumpTransactionsError, ExportSkFromWalletError,
    GetPoolError, GetPoolInfoError, GetSummaryError, GetTxBalanceError, GetTxError, PoolError,
    PrepareTxError, SendFaucetError, SendTxError, SummarizeWalletError, UnlockWalletError,
};
use crate::types::{Melwallet, MelwalletdHelpers, WalletSummary};
use crate::walletdata::{AnnCoinID, TransactionStatus};
use async_trait::async_trait;
use base32::Alphabet;
use nanorpc::nanorpc_derive;
use std::fmt::Debug;
use themelio_structs::{
    BlockHeight, CoinData, CoinID, CoinValue, Denom, NetID, Transaction, TxHash, TxKind,
};
use themelio_structs::{Header, PoolKey, PoolState};
use tmelcrypt::{Ed25519SK, HashVal, Hashable};

use melnet::{self, MelnetError};
use serde::*;

use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareTxArgs {
    #[serde(default)]
    kind: Option<TxKind>,
    inputs: Vec<CoinID>,
    outputs: Vec<CoinData>,
    #[serde(default, with = "stdcode::hexvec")]
    covenants: Vec<Vec<u8>>,
    data: Option<String>,
    #[serde(default)]
    nobalance: Vec<Denom>,
    #[serde(default)]
    fee_ballast: usize,
    signing_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PoolInfo {
    result: u128,
    price_impact: f64,
    poolkey: String,
}
#[derive(Serialize, Deserialize)]
pub struct TxBalance(bool, TxKind, BTreeMap<String, i128>);

// #[derive(Serialize, Deserialize, Error, Debug)]
// pub enum RequestError<T: std::error::Error> {
//     FatalError(String),
// }

#[nanorpc_derive]
#[async_trait]
pub trait MelwalletdProtocol: Send + Sync {
    async fn summarize_wallet(
        &self,
        wallet_name: String,
    ) -> Result<WalletSummary, error::WalletNotFound>;

    /// Attempts to get network information
    async fn get_summary(&self) -> Result<Header, error::MelnetError>;

    ///Attempts to get a poolstate
    async fn get_pool(&self, pool_key: PoolKey) -> Result<PoolState, StateError<PoolKeyError>>;
    async fn simulate_pool_swap(
        &self,
        to: Denom,
        from: Denom,
        value: u128,
    ) -> Result<PoolInfo, StateError<PoolKeyError>>;
    async fn create_wallet(
        &self,
        wallet_name: String,
        password: Option<String>,
        secret: Option<String>,
    ) -> Result<(), CreateWalletError>;
    async fn dump_coins(
        &self,
        wallet_name: String,
    ) -> Result<Vec<(CoinID, CoinData)>, DumpCoinsError>;
    async fn dump_transactions(
        &self,
        wallet_name: String,
    ) -> Result<Vec<(TxHash, Option<BlockHeight>)>, DumpTransactionsError>;
    async fn lock_wallet(&self, wallet_name: String);
    async fn unlock_wallet(
        &self,
        wallet_name: String,
        password: Option<String>,
    ) -> Result<(), UnlockWalletError>;
    async fn export_sk_from_wallet(
        &self,
        wallet_name: String,
        password: Option<String>,
    ) -> Result<String, ExportSkFromWalletError>;
    async fn prepare_tx(
        &self,
        wallet_name: String,
        request: PrepareTxArgs,
    ) -> Result<Transaction, PrepareTxError>;
    async fn send_tx(&self, wallet_name: String, tx: Transaction) -> Result<TxHash, SendTxError>;
    async fn get_tx_balance(
        &self,
        wallet_name: String,
        txhash: HashVal,
    ) -> Result<TxBalance, GetTxBalanceError>;
    async fn get_tx(
        &self,
        wallet_name: String,
        txhash: HashVal,
    ) -> Result<TransactionStatus, GetTxError>;
    async fn send_faucet(&self, wallet_name: String) -> Result<TxHash, SendFaucetError>;
}

#[derive(Clone)]
pub struct MelwalletdRpcImpl<T: Melwallet, State: MelwalletdHelpers<T>> {
    pub state: Arc<State>,
    _phantom: PhantomData<T>,
}
impl<T: Melwallet + Send + Sync, State: MelwalletdHelpers<T> + Send + Sync>
     MelwalletdRpcImpl<T, State>
{
    async fn summarize_wallet(&self, wallet_name: String) -> Result<WalletSummary, WalletNotFound> {
        let state = self.state.clone();
        let wallet_list = state.list_wallets().await;
        wallet_list
            .get(&wallet_name)
            .cloned()
            .ok_or(error::WalletNotFound.into())
    }

    async fn get_summary(&self) -> Result<Header, error::MelnetError> {
        let state = self.state.clone();
        let client = state.client().clone();
        let snap = client.snapshot().await?;
        Ok(snap.current_header())
    }

    /// get a pool by poolkey,
    /// can fail by:
    ///     providing an invalid poolkey like MEL/MEL
    ///     inability to create snapshot
    /// returns None if pool doesn't exist
    /// ErrorEnum => PoolError; PoolKeyError *melnet::MelnetError BadRequest
    async fn get_pool(&self, pool_key: PoolKey) -> Result<Option<PoolState>, StateError<PoolKeyError>> {
        let pool_key = pool_key
            .to_canonical()
            .ok_or_else(|| error::PoolKeyError(pool_key))?;

        let state = self.state.clone();
        let client = state.client().clone();
        let pool = client.snapshot().await?.get_pool(pool_key).await?;
        Ok(pool)
    }

    /// simulate swapping one asset for another
    /// can fail :
    ///     bad pool key
    ///     failed snapshot
    /// None if pool doesn't exist
    async fn simulate_pool_swap(
        &self,
        to: Denom,
        from: Denom,
        value: u128,
    ) -> Result<Option<PoolInfo>, PoolError> {
        let pool_key = PoolKey {
            left: to,
            right: from,
        };
        let pool_key = pool_key
            .to_canonical()
            .ok_or_else(|| error::PoolKeyError(pool_key))?;

        let state = self.state.clone();
        let client = state.client().clone();

        let maybe_pool_state = client.snapshot().await?.get_pool(pool_key).await?;

        if maybe_pool_state.is_none() {
            return Ok(None);
        }

        let pool_state = maybe_pool_state.unwrap();

        let left_to_right = pool_key.left == from;

        let r = if left_to_right {
            let old_price = pool_state.lefts as f64 / pool_state.rights as f64;
            let mut new_pool_state = pool_state;
            let (_, new) = new_pool_state.swap_many(value, 0);
            let new_price = new_pool_state.lefts as f64 / new_pool_state.rights as f64;
            PoolInfo {
                result: new,
                price_impact: (new_price / old_price - 1.0),
                poolkey: hex::encode(pool_key.to_bytes()),
            }
        } else {
            let old_price = pool_state.rights as f64 / pool_state.lefts as f64;
            let mut new_pool_state = pool_state;
            let (new, _) = new_pool_state.swap_many(0, value);
            let new_price = new_pool_state.rights as f64 / new_pool_state.lefts as f64;
            PoolInfo {
                result: new,
                price_impact: (new_price / old_price - 1.0),
                poolkey: hex::encode(pool_key.to_bytes()),
            }
        };
        Ok(Some(r))
    }
    /// ErrorEnum => CreateWalletError; SecretKeyError WalletCreationError
    async fn create_wallet(
        &self,
        wallet_name: String,
        password: Option<String>,
        secret: Option<String>,
    ) -> Result<(), CreateWalletError> {
        let state = self.state.clone();
        let sk = if let Some(secret) = secret {
            // We must reconstruct the secret key using the ed25519-dalek library
            let secret = base32::decode(Alphabet::Crockford, &secret).ok_or(
                error::SecretKeyError("Failed to decode secret key".to_owned()),
            )?;
            let secret = ed25519_dalek::SecretKey::from_bytes(&secret)
                .map_err(|_| error::SecretKeyError("Failed to create secret key".to_owned()))?;
            let public: ed25519_dalek::PublicKey = (&secret).into();
            let mut vv = [0u8; 64];
            vv[0..32].copy_from_slice(&secret.to_bytes());
            vv[32..].copy_from_slice(&public.to_bytes());
            Ed25519SK(vv)
        } else {
            tmelcrypt::ed25519_keygen().1
        };
        match state.create_wallet(&wallet_name, sk, password).await {
            Ok(_) => Ok(()),
            Err(e) => Err(error::WalletCreationError(e).into()), // bikeshed this more
        }
    }

    /// ErrorEnum => DumpCoinsError; WalletNotFound
    async fn dump_coins(
        &self,
        wallet_name: String,
    ) -> Result<Vec<(CoinID, CoinData)>, DumpCoinsError> {
        let state = self.state.clone();
        let wallet = state
            .get_wallet(&wallet_name)
            .await
            .ok_or(error::WalletNotFound)?;
        let coins = wallet.get_coin_mapping(true, false).await;
        let coin_vec = &coins.into_iter().collect::<Vec<_>>();
        Ok(coin_vec.to_owned())
    }
    /// ErrorEnum => DumpTransactionsError;
    async fn dump_transactions(
        &self,
        wallet_name: String,
    ) -> Result<Vec<(TxHash, Option<BlockHeight>)>, DumpTransactionsError> {
        let state = self.state.clone();
        let wallet = state
            .get_wallet(&wallet_name)
            .await
            .ok_or(error::WalletNotFound)?;
        let transactions = wallet.get_transaction_history().await;
        Ok(transactions)
    }

    async fn lock_wallet(&self, wallet_name: String) {
        let state = self.state.clone();
        state.lock(&wallet_name);
    }

    async fn unlock_wallet(
        &self,
        wallet_name: String,
        password: Option<String>,
    ) -> Result<(), InvalidPassword> {
        let state = self.state.clone();
        state
            .unlock(&wallet_name, password)
            .ok_or(error::InvalidPassword)?;
        Ok(())
    }

    /// ErrorEnum => ExportSkFromWalletError; InvalidPassword
    async fn export_sk_from_wallet(
        &self,
        wallet_name: String,
        password: Option<String>,
    ) -> Result<Option<String>, ExportSkFromWalletError> {
        let state = self.state.clone();
        let maybe_secret = state
            .get_secret_key(&wallet_name, password)?;

        if maybe_secret.is_none() { return Ok(None) }
        let encoded: String = base32::encode(Alphabet::Crockford, &secret.0[..32]).into();
        Ok(encoded)
    }

    /// ErrorEnum => PrepareTxError;
    async fn prepare_tx(
        &self,
        wallet_name: String,
        request: PrepareTxArgs,
    ) -> Result<Transaction, PrepareTxError> {
        let state = self.state.clone();
        let signing_key: Arc<dyn Signer> = if let Some(signing_key) = request.signing_key.as_ref() {
            Arc::new(
                signing_key
                    .parse::<Ed25519SK>()
                    .map_err(|_| error::InvalidSignature)?,
            )
        } else {
            state
                .get_signer(&wallet_name)
                .ok_or(error::FailedUnlock(wallet_name.clone()))?
        };
        let wallet = state
            .get_wallet(&wallet_name)
            .await
            .ok_or(error::WalletNotFound)?;

        // calculate fees
        let client = state.client().clone();
        let snapshot = client.snapshot().await?;
        let fee_multiplier = snapshot.current_header().fee_multiplier;
        let kind = request.kind;
        let data = match request.data.as_ref() {
            Some(v) => Some(hex::decode(v).map_err(|_| error::BadRequest("".to_owned()))?),
            None => None,
        };
        let prepared_tx = wallet
            .prepare(
                request.inputs.clone(),
                request.outputs.clone(),
                fee_multiplier,
                |mut tx: Transaction| {
                    if let Some(kind) = kind {
                        tx.kind = kind
                    }
                    if let Some(data) = data.clone() {
                        tx.data = data
                    }
                    tx.covenants.extend_from_slice(&request.covenants);
                    for i in 0..tx.inputs.len() {
                        tx = signing_key.sign_tx(tx, i)?;
                    }
                    Ok(tx)
                },
                request.nobalance.clone(),
                request.fee_ballast,
                state.client().snapshot().await?,
            )
            .await
            .map_err(|_| error::BadRequest("".to_owned()))?;

        Ok(prepared_tx)
    }

    /// ErrorEnum => SendTxError;
    async fn send_tx(&self, wallet_name: String, tx: Transaction) -> Result<TxHash, SendTxError> {
        let state = self.state.clone();
        let wallet = state
            .get_wallet(&wallet_name)
            .await
            .ok_or(error::BadRequest("".to_owned()))?;
        let snapshot = state.client().snapshot().await?;

        // we send it off ourselves
        snapshot.get_raw().send_tx(tx.clone()).await?;
        // we mark the TX as sent in this thread.
        wallet
            .commit_sent(
                tx.clone(),
                snapshot.current_header().height + BlockHeight(10),
            )
            .await
            .map_err(|_| error::BadRequest("".to_owned()))?;
        log::info!("sent transaction with hash {}", tx.hash_nosigs());
        let r = &tx.hash_nosigs();
        Ok(r.to_owned())
    }
    /// ErrorEnum => GetTxBalanceError;
    async fn get_tx_balance(
        &self,
        wallet_name: String,
        txhash: HashVal,
    ) -> Result<TxBalance, GetTxBalanceError> {
        let state = self.state.clone();
        let wallet = state
            .get_wallet(&wallet_name)
            .await
            .ok_or(error::WalletNotFound)?;
        let raw = wallet
            .get_transaction(txhash.into(), async {
                Ok(state.client().snapshot().await?)
            })
            .await?
            .ok_or(error::TransactionNotFound(txhash.into()))?;

        // Is this self-originated? We check the covenants
        let self_originated = raw.covenants.iter().any(|c| c.hash() == wallet.address().0);
        // Total balance out
        let mut balance: BTreeMap<String, i128> = BTreeMap::new();
        // Add all outputs to balance

        if self_originated {
            *balance
                .entry(hex::encode(Denom::Mel.to_bytes()))
                .or_default() -= raw.fee.0 as i128;
        }
        for (idx, output) in raw.outputs.iter().enumerate() {
            let coinid = raw.output_coinid(idx as u8);
            let denom_key = hex::encode(output.denom.to_bytes());
            // first we *deduct* any balance if this self-originated
            if self_originated {
                *balance.entry(denom_key).or_default() -= output.value.0 as i128;
            }
            // then, if we find this value in our coins, we add it back. this turns out to take care of swap tx well
            if let Some(ours) = wallet.get_one_coin(coinid).await {
                let denom_key = hex::encode(ours.denom.to_bytes());
                if ours.covhash == wallet.address() {
                    *balance.entry(denom_key).or_default() += ours.value.0 as i128;
                }
            }
        }
        let r = TxBalance(self_originated, raw.kind, balance);

        Ok(r)
    }

    /// ErrorEnum => GetTxError;
    async fn get_tx(
        &self,
        wallet_name: String,
        txhash: HashVal,
    ) -> Result<TransactionStatus, GetTxError> {
        let state = self.state.clone();
        let wallet = state
            .get_wallet(&wallet_name)
            .await
            .ok_or(error::WalletNotFound)?;

        let raw = wallet
            .get_cached_transaction(txhash.into())
            .await
            .ok_or(error::TransactionNotFound(txhash.into()))?;
        let mut confirmed_height = None;
        for idx in 0..raw.outputs.len() {
            if let Some(cdh) = wallet
                .get_coin_confirmation(raw.output_coinid(idx as u8))
                .await
            {
                confirmed_height = Some(cdh.height);
            }
        }
        let outputs = raw
            .outputs
            .iter()
            .enumerate()
            .map(|(i, cd)| {
                let coin_id = raw.output_coinid(i as u8).to_string();
                let is_change = cd.covhash == wallet.address();
                let coin_data = cd.clone();
                AnnCoinID {
                    coin_data,
                    is_change,
                    coin_id,
                }
            })
            .collect();

        if confirmed_height.is_none() {
            // Must be pending
            if !wallet.is_pending(txhash.into()).await {
                return Err(error::LostTransaction(txhash.into()));
            }
        }
        Ok(TransactionStatus {
            raw,
            confirmed_height,
            outputs,
        })
    }

    /// ErrorEnum => SendFaucetError;
    async fn send_faucet(&self, wallet_name: String) -> Result<TxHash, SendFaucetError> {
        let state = self.state.clone();
        let network = state.network;
        let wallet = state
            .get_wallet(&wallet_name)
            .await
            .ok_or(error::WalletNotFound)?;

        // TODO: protect other networks where faucet transaction applicability is unknown
        if network == NetID::Mainnet {
            return Err(error::InvalidFaucetTransaction);
        }
        let tx = Transaction {
            kind: TxKind::Faucet,
            inputs: vec![],
            outputs: vec![CoinData {
                covhash: wallet.address(),
                value: CoinValue::from_millions(1001u64),
                denom: Denom::Mel,
                additional_data: vec![],
            }],
            data: (0..32).map(|_| fastrand::u8(0..=255)).collect(),
            fee: CoinValue::from_millions(1001u64),
            covenants: vec![],
            sigs: vec![],
        };
        // we mark the TX as sent in this thread
        let txhash = tx.hash_nosigs();
        wallet
            .commit_sent(tx, BlockHeight(10000000000))
            .await
            .map_err(|_| error::BadRequest("Failed to submit faucet transaction".to_owned()))?;
        Ok(txhash)
    }
}
