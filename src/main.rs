mod chainstream;
mod raydium;

use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::{
    chainstream::{
        client::ChainStreamClient,
        methods::{CommitmentLevel, Method},
    },
    raydium::{
        anchor_events::RaydiumCLMMEvent,
        parse::{parse_raydium_anchor_events, RAYDIUM_CLMM_PROGRAM},
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber with environment filter
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let token = std::env::var("SYNDICA_TOKEN")
        .expect("SYNDICA_TOKEN env var not set, use `export SYNDICA_TOKEN=<your_token>`");

    let method = Method::new_transaction_subscription()
        .one_of_account_keys(&[RAYDIUM_CLMM_PROGRAM])
        .commitment_level(CommitmentLevel::Confirmed);

    let client = ChainStreamClient::new(&token).await?;
    let mut subscription = client.subscribe(method).await?;

    info!("Listening for Raydium CLMM swap events...");

    while let Some(Ok(transaction)) = subscription.next().await {
        let meta = transaction.meta();
        if let Ok(anchor_events) = parse_raydium_anchor_events(meta) {
            if let Some(RaydiumCLMMEvent::Swap(swap_event)) = anchor_events.first() {
                if swap_event.zero_for_one {
                    info!(
                        "{} --> {}",
                        swap_event.token_account_0, swap_event.token_account_1
                    );
                } else {
                    info!(
                        "{} <-- {}",
                        swap_event.token_account_0, swap_event.token_account_1
                    );
                }
            }
        }
    }

    Ok(())
}
