#![allow(unused_imports)]

use chainstream_raydium_trade_pair::{
    chainstream::{
        client::ChainStreamClient,
        methods::{CommitmentLevel, Method},
    },
    raydium::{anchor_events::RaydiumCLMMEvent, parse::parse_raydium_anchor_events},
};

const RAYDIUM_CLMM_PROGRAM: &'static str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("SYNDICA_TOKEN")
        .expect("SYNDICA_TOKEN env var not set, use `export SYNDICA_TOKEN=<your_token>`");

    let method = Method::new_transaction_subscription()
        .one_of_account_keys(&[RAYDIUM_CLMM_PROGRAM])
        .commitment_level(CommitmentLevel::Confirmed);

    let client = ChainStreamClient::new(&token).await?;

    let mut subscription = client.subscribe(method).await?;

    while let Some(Ok(transaction)) = subscription.next().await {
        let meta = transaction.meta();
        if let Ok(anchor_events) = parse_raydium_anchor_events(meta) {
            if let Some(RaydiumCLMMEvent::Swap(swap_event)) = anchor_events.get(0) {
                if swap_event.zero_for_one {
                    println!(
                        "{} --> {}",
                        swap_event.token_account_0, swap_event.token_account_1
                    );
                } else {
                    println!(
                        "{} <-- {}",
                        swap_event.token_account_0, swap_event.token_account_1
                    );
                }
            }
        }
    }

    Ok(())
}
