//! ChainStream API client for streaming Raydium CLMM events from Solana.
//!
//! This crate provides a WebSocket client for connecting to Syndica's ChainStream API
//! and parsing Raydium CLMM (Concentrated Liquidity Market Maker) events.
//!
//! # Example
//!
//! ```no_run
//! use chainstream_raydium_trade_pair::{
//!     chainstream::{client::ChainStreamClient, methods::{Method, CommitmentLevel}},
//!     raydium::{parse::{parse_raydium_anchor_events, RAYDIUM_CLMM_PROGRAM}, anchor_events::RaydiumCLMMEvent},
//! };
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let token = std::env::var("SYNDICA_TOKEN")?;
//!     let client = ChainStreamClient::new(&token).await?;
//!
//!     let method = Method::new_transaction_subscription()
//!         .one_of_account_keys(&[RAYDIUM_CLMM_PROGRAM])
//!         .commitment_level(CommitmentLevel::Confirmed);
//!
//!     let mut subscription = client.subscribe(method).await?;
//!
//!     while let Some(Ok(tx)) = subscription.next().await {
//!         let events = parse_raydium_anchor_events(tx.meta())?;
//!         for event in events {
//!             if let RaydiumCLMMEvent::Swap(swap) = event {
//!                 println!("Swap detected: {:?}", swap);
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! ```

pub mod chainstream;
pub mod raydium;
