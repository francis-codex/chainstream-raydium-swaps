//! Provides the necessary types required to build Chainstream RPC requests.
use jsonrpsee::core::params::{self, ObjectParams};
use serde::{Deserialize, Serialize};
use serde_json;
use thiserror;

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

#[derive(Debug, Clone, thiserror::Error)]
pub enum RpcError {
    #[error("Unsupported method")]
    UnsupportedMethod,
    #[error("Params error: {0}")]
    ParamsError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "transactionsSubscribe")]
    TransactionSubscribe(TransactionMethodBuilder),
    #[serde(rename = "blocksSubscribe")]
    BlockSubscribe(BlockMethodBuilder),
    #[serde(rename = "slotUpdatesSubscribe")]
    SlotSubscribe(SlotMethodBuilder),
}

impl Method {
    pub fn new_transaction_subscription() -> TransactionMethodBuilder {
        TransactionMethodBuilder::default()
    }

    pub fn new_block_subscription() -> BlockMethodBuilder {
        BlockMethodBuilder::default()
    }

    pub fn new_slot_subscription() -> SlotMethodBuilder {
        SlotMethodBuilder::default()
    }

    pub fn params(&self) -> Result<params::ObjectParams, RpcError> {
        match self {
            Method::TransactionSubscribe(builder) => builder.build_params(),
            Method::BlockSubscribe(builder) => builder.build_params(),
            Method::SlotSubscribe(builder) => builder.build_params(),
        }
    }

    pub fn subscribe_method(&self) -> &'static str {
        match self {
            Method::TransactionSubscribe(_) => "transactionsSubscribe",
            Method::BlockSubscribe(_) => "blocksSubscribe",
            Method::SlotSubscribe(_) => "slotUpdatesSubscribe",
        }
    }

    pub fn unsubscribe_method(&self) -> &'static str {
        match self {
            Method::TransactionSubscribe(_) => "transactionsUnsubscribe",
            Method::BlockSubscribe(_) => "blocksUnsubscribe",
            Method::SlotSubscribe(_) => "slotUpdatesUnsubscribe",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransactionMethodBuilder {
    pub network: Network,
    pub verified: bool,
    pub filter: TransactionFilter,
}

impl TransactionMethodBuilder {
    pub fn filter(self, filter: TransactionFilter) -> Self {
        Self { filter, ..self }
    }

    pub fn network(self, network: Network) -> Self {
        Self { network, ..self }
    }

    pub fn verified(self, verified: bool) -> Self {
        Self { verified, ..self }
    }

    pub fn exclude_votes(self, exclude_votes: bool) -> Self {
        let filter = TransactionFilter {
            exclude_votes: Some(exclude_votes),
            ..self.filter
        };
        Self { filter, ..self }
    }

    pub fn all_account_keys(self, account_keys: Vec<String>) -> Self {
        let filter = TransactionFilter {
            account_keys: Some(PubKeySelector {
                all: Some(account_keys),
                ..self.filter.account_keys.unwrap_or_default()
            }),
            ..self.filter
        };
        Self { filter, ..self }
    }

    pub fn one_of_account_keys(self, account_keys: Vec<String>) -> Self {
        let filter = TransactionFilter {
            account_keys: Some(PubKeySelector {
                one_of: Some(account_keys),
                ..self.filter.account_keys.unwrap_or_default()
            }),
            ..self.filter
        };
        Self { filter, ..self }
    }

    pub fn exclude_account_keys(self, account_keys: Vec<String>) -> Self {
        let filter = TransactionFilter {
            account_keys: Some(PubKeySelector {
                exclude: Some(account_keys),
                ..self.filter.account_keys.unwrap_or_default()
            }),
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

    pub fn build(self) -> Method {
        Method::TransactionSubscribe(self)
    }
}

impl Default for TransactionMethodBuilder {
    fn default() -> Self {
        Self {
            network: Network::SolanaMainnet,
            verified: false,
            filter: TransactionFilter {
                exclude_votes: None,
                account_keys: None,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransactionFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_votes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_keys: Option<PubKeySelector>,
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
pub struct BlockMethodBuilder {
    pub network: Network,
    pub verified: bool,
}

impl BlockMethodBuilder {
    pub fn network(self, network: Network) -> Self {
        Self { network, ..self }
    }

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

    pub fn build(self) -> Method {
        Method::BlockSubscribe(self)
    }
}

impl Default for BlockMethodBuilder {
    fn default() -> Self {
        Self {
            network: Network::SolanaMainnet,
            verified: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SlotMethodBuilder {
    pub network: Network,
    pub verified: bool,
}

impl SlotMethodBuilder {
    pub fn network(self, network: Network) -> Self {
        Self { network, ..self }
    }

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

    pub fn build(self) -> Method {
        Method::SlotSubscribe(self)
    }
}

impl Default for SlotMethodBuilder {
    fn default() -> Self {
        Self {
            network: Network::SolanaMainnet,
            verified: false,
        }
    }
}
