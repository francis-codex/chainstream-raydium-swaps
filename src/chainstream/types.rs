//! Types that Chainstream emits.

use serde::{Deserialize, Serialize};

/// [`jsonrpsee`] implements Ethereum pub sub model (https://geth.ethereum.org/docs/interacting-with-geth/rpc/pubsub)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthereumPubSubResult<T> {
    pub subscription: u64,
    pub result: T,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    pub pubkey: String,
    pub lamports: i64,
    pub post_balance: u64,
    pub reward_type: Option<i32>,
    pub commission: Option<u32>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Timestamp(String);

pub mod transaction {
    use std::str::FromStr;

    use solana_sdk::signature::Signature;

    use super::Timestamp;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct TransactionWrite {
        pub context: Context,
        pub value: Transaction,
    }

    impl TransactionWrite {
        #[allow(unused)]
        pub fn signature(&self) -> Signature {
            Signature::from_str(self.context.signature.as_str()).unwrap()
        }

        #[allow(unused)]
        pub fn logs(&self) -> &[String] {
            self.value.meta.as_ref().unwrap().log_messages.as_slice()
        }

        #[allow(unused)]
        pub fn meta(&self) -> Meta {
            self.value.meta.as_ref().unwrap().clone()
        }
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Context {
        pub slot_status: String,
        pub node_time: Option<Timestamp>,
        pub is_vote: bool,
        pub signature: String,
        pub index: Option<u64>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Transaction {
        pub block_time: Option<u64>,
        pub meta: Option<Meta>,
        pub slot: u64,
        #[serde(flatten)]
        pub transaction: Option<Body>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Meta {
        pub err: Option<serde_json::Value>,
        pub fee: u64,
        pub inner_instructions: Vec<InnerInstructions>,
        pub loaded_addresses: Option<LoadedAddresses>,
        pub log_messages: Vec<String>,
        pub post_balances: Vec<u64>,
        pub post_token_balances: Vec<TokenBalance>,
        pub pre_balances: Vec<u64>,
        pub pre_token_balances: Vec<TokenBalance>,
        pub rewards: Vec<super::Reward>,
        pub status: Option<serde_json::Value>,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct CompiledInstruction {
        pub program_id_index: u32,
        pub accounts: Vec<u32>,
        pub data: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InnerInstructions {
        pub index: u32,
        pub instructions: Vec<CompiledInstruction>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LoadedAddresses {
        pub writable: Vec<String>,
        pub readonly: Vec<String>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(untagged)]
    pub enum OneofTransactionStatus {
        Ok(bool),
        Err(serde_json::Value),
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TokenBalance {
        pub account_index: u32,
        pub mint: String,
        pub owner: String,
        pub program_id: String,
        pub ui_token_amount: Option<TokenAmount>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TokenAmount {
        pub amount: String,
        pub decimals: u32,
        pub ui_amount: Option<f64>,
        pub ui_amount_string: String,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Body {
        pub message: Option<Message>,
        pub message_hash: String,
        pub signatures: Vec<String>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Message {
        pub account_keys: Vec<String>,
        pub address_table_lookups: Vec<AddressTableLookup>,
        pub header: Option<Header>,
        pub instructions: Vec<CompiledInstruction>,
        pub recent_blockhash: String,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AddressTableLookup {
        pub account_key: String,
        pub writable_indexes: Vec<u32>,
        pub readonly_indexes: Vec<u32>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Header {
        pub num_readonly_signed_accounts: u32,
        pub num_readonly_unsigned_accounts: u32,
        pub num_required_signatures: u32,
    }
}

pub mod block {
    use super::*;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BlockUpdate {
        pub context: Option<Context>,
        pub value: Option<Value>,
    }
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Context {
        pub node_time: Option<Timestamp>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Value {
        pub slot: u64,
        pub blockhash: String,
        pub rewards: Vec<Reward>,
        pub block_time: Option<Timestamp>,
        pub block_height: Option<u64>,
        pub parent_slot: Option<u64>,
        pub parent_blockhash: Option<String>,
        pub executed_transaction_count: Option<u64>,
    }
}

pub mod slot {
    use super::Timestamp;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SlotUpdate {
        pub context: Option<Context>,
        pub value: Option<Value>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Context {
        pub node_time: Option<Timestamp>,
    }
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Value {
        pub slot: u64,
        pub parent: Option<u64>,
        pub status: String,
    }
}

pub mod full_block {
    use super::transaction::TransactionWrite;
    use super::*;

    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Rewards {
        pub rewards: Vec<Reward>,
    }

    #[derive(serde::Serialize, serde::Deserialize, Clone)]
    pub struct FullBlock {
        pub context: Option<Context>,
        pub value: Option<Value>,
    }

    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Context {
        pub node_time: Option<Timestamp>,
    }
    #[derive(Clone, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Value {
        pub slot: u64,
        pub blockhash: String,
        #[serde(skip_deserializing)]
        pub rewards: Option<Rewards>,
        pub block_time: u64,
        pub block_height: u64,
        pub parent_slot: u64,
        pub parent_blockhash: String,
        pub executed_transaction_count: u64,
        pub transactions: Option<Vec<TransactionWrite>>,
        pub transaction_signatures: Option<Vec<String>>,
        #[allow(dead_code)]
        #[serde(skip)]
        pub entries_count: u64,
        #[serde(skip)]
        #[allow(dead_code)]
        pub entries: Vec<()>,
    }
}
