#![allow(unused)]
//! Provides the necessary types required to build Chainstream RPC requests.
use jsonrpsee::core::params::{self, ObjectParams};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;
use thiserror;

use super::types::transaction::TransactionWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    SolanaMainnet,
    SolanaTestnet,
}

impl Network {
    pub fn as_str(&self) -> &str {
        match self {
            Network::SolanaMainnet => "solana-mainnet",
            Network::SolanaTestnet => "solana-testnet",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitmentLevel {
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "finalized")]
    Finalized,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum RpcError {
    #[error("Params error: {0}")]
    ParamsError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "transactionsSubscribe")]
    TransactionSubscribe(TransactionMethod),
    #[serde(rename = "blocksSubscribe")]
    BlockSubscribe(BlockMethod),
    #[serde(rename = "slotUpdatesSubscribe")]
    SlotSubscribe(SlotMethod),
}

pub trait SubscriptionMethod {
    type Output: DeserializeOwned;
    fn subscribe_method(&self) -> &'static str;
    fn unsubscribe_method(&self) -> &'static str;
    fn params(&self) -> Result<ObjectParams, RpcError>;
}

impl Method {
    pub fn new_transaction_subscription() -> TransactionMethod {
        TransactionMethod::default()
    }

    #[allow(unused)]
    pub fn new_block_subscription() -> BlockMethod {
        BlockMethod::default()
    }

    #[allow(unused)]
    pub fn new_slot_subscription() -> SlotMethod {
        SlotMethod::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransactionMethod {
    pub network: Network,
    pub verified: bool,
    pub filter: TransactionFilter,
}

impl TransactionMethod {
    #[allow(unused)]
    pub fn filter(self, filter: TransactionFilter) -> Self {
        Self { filter, ..self }
    }

    #[allow(unused)]
    pub fn network(self, network: Network) -> Self {
        Self { network, ..self }
    }

    #[allow(unused)]
    pub fn verified(self, verified: bool) -> Self {
        Self { verified, ..self }
    }

    #[allow(unused)]
    pub fn exclude_votes(self, exclude_votes: bool) -> Self {
        let filter = TransactionFilter {
            exclude_votes: Some(exclude_votes),
            ..self.filter
        };
        Self { filter, ..self }
    }

    #[allow(unused)]
    pub fn all_account_keys<T: ToString>(self, account_keys: &[T]) -> Self {
        let filter = TransactionFilter {
            account_keys: Some(PubKeySelector {
                all: Some(account_keys.iter().map(|k| k.to_string()).collect()),
                ..self.filter.account_keys.unwrap_or_default()
            }),
            ..self.filter
        };
        Self { filter, ..self }
    }

    #[allow(unused)]
    pub fn one_of_account_keys<T: ToString>(self, account_keys: &[T]) -> Self {
        let filter = TransactionFilter {
            account_keys: Some(PubKeySelector {
                one_of: Some(account_keys.iter().map(|k| k.to_string()).collect()),
                ..self.filter.account_keys.unwrap_or_default()
            }),
            ..self.filter
        };
        Self { filter, ..self }
    }

    #[allow(unused)]
    pub fn exclude_account_keys<T: ToString>(self, account_keys: &[T]) -> Self {
        let filter = TransactionFilter {
            account_keys: Some(PubKeySelector {
                exclude: Some(account_keys.iter().map(|k| k.to_string()).collect()),
                ..self.filter.account_keys.unwrap_or_default()
            }),
            ..self.filter
        };
        Self { filter, ..self }
    }

    #[allow(unused)]
    pub fn commitment_level(self, commitment: CommitmentLevel) -> Self {
        let filter = TransactionFilter {
            commitment: Some(commitment),
            ..self.filter
        };
        Self { filter, ..self }
    }

    pub fn build_params(&self) -> Result<ObjectParams, RpcError> {
        let mut params = params::ObjectParams::new();
        params
            .insert("network", self.network.as_str())
            .map_err(|e| RpcError::ParamsError(e.to_string()))?;
        params
            .insert("verified", self.verified)
            .map_err(|e| RpcError::ParamsError(e.to_string()))?;
        params
            .insert("filter", serde_json::to_value(&self.filter).unwrap())
            .map_err(|e| RpcError::ParamsError(e.to_string()))?;

        Ok(params)
    }

    #[allow(unused)]
    pub fn build(self) -> Method {
        Method::TransactionSubscribe(self)
    }
}

impl Default for TransactionMethod {
    fn default() -> Self {
        Self {
            network: Network::SolanaMainnet,
            verified: false,
            filter: TransactionFilter {
                exclude_votes: None,
                account_keys: None,
                commitment: None,
            },
        }
    }
}

impl SubscriptionMethod for TransactionMethod {
    type Output = TransactionWrite;

    fn subscribe_method(&self) -> &'static str {
        "transactionsSubscribe"
    }

    fn unsubscribe_method(&self) -> &'static str {
        "transactionsUnsubscribe"
    }

    fn params(&self) -> Result<ObjectParams, RpcError> {
        self.build_params()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransactionFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_votes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_keys: Option<PubKeySelector>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment: Option<CommitmentLevel>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PubKeySelector {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    one_of: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BlockMethod {
    pub network: Network,
    pub verified: bool,
}

impl BlockMethod {
    #[allow(unused)]
    pub fn network(self, network: Network) -> Self {
        Self { network, ..self }
    }

    #[allow(unused)]
    pub fn verified(self, verified: bool) -> Self {
        Self { verified, ..self }
    }

    pub fn build_params(&self) -> Result<ObjectParams, RpcError> {
        let mut params = params::ObjectParams::new();
        params
            .insert("network", self.network.as_str())
            .map_err(|e| RpcError::ParamsError(e.to_string()))?;
        params
            .insert("verified", self.verified)
            .map_err(|e| RpcError::ParamsError(e.to_string()))?;

        Ok(params)
    }

    #[allow(unused)]
    pub fn build(self) -> Method {
        Method::BlockSubscribe(self)
    }
}

impl Default for BlockMethod {
    fn default() -> Self {
        Self {
            network: Network::SolanaMainnet,
            verified: false,
        }
    }
}

impl SubscriptionMethod for BlockMethod {
    type Output = serde_json::Value;

    fn subscribe_method(&self) -> &'static str {
        "blocksSubscribe"
    }

    fn unsubscribe_method(&self) -> &'static str {
        "blocksUnsubscribe"
    }

    fn params(&self) -> Result<ObjectParams, RpcError> {
        self.build_params()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SlotMethod {
    pub network: Network,
    pub verified: bool,
}

impl SlotMethod {
    #[allow(unused)]
    pub fn network(self, network: Network) -> Self {
        Self { network, ..self }
    }

    #[allow(unused)]
    pub fn verified(self, verified: bool) -> Self {
        Self { verified, ..self }
    }

    pub fn build_params(&self) -> Result<ObjectParams, RpcError> {
        let mut params = params::ObjectParams::new();
        params
            .insert("network", self.network.as_str())
            .map_err(|e| RpcError::ParamsError(e.to_string()))?;
        params
            .insert("verified", self.verified)
            .map_err(|e| RpcError::ParamsError(e.to_string()))?;

        Ok(params)
    }

    #[allow(unused)]
    pub fn build(self) -> Method {
        Method::SlotSubscribe(self)
    }
}

impl Default for SlotMethod {
    fn default() -> Self {
        Self {
            network: Network::SolanaMainnet,
            verified: false,
        }
    }
}

impl SubscriptionMethod for SlotMethod {
    type Output = serde_json::Value;

    fn subscribe_method(&self) -> &'static str {
        "slotUpdatesSubscribe"
    }

    fn unsubscribe_method(&self) -> &'static str {
        "slotUpdatesUnsubscribe"
    }

    fn params(&self) -> Result<ObjectParams, RpcError> {
        self.build_params()
    }
}
