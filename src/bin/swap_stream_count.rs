use std::collections::HashMap;

use chainstream_raydium_trade_pair::{
    chainstream::{
        client::ChainStreamClient, methods::Method, types::transaction::TransactionWrite,
    },
    raydium::{anchor_events::RaydiumCLMMEvent, parse::parse_raydium_anchor_events},
};

const RAYDIUM_CLMM_PROGRAM: &'static str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("SYNDICA_TOKEN")
        .expect("SYNDICA_TOKEN env var not set, use `export SYNDICA_TOKEN=<your_token>`");

    let method = Method::new_transaction_subscription()
        .one_of_account_keys(vec![RAYDIUM_CLMM_PROGRAM.into()])
        .build();

    let client = ChainStreamClient::new("wss://chainstream.api.syndica.io", &token).await?;

    let mut subscription = client.subscribe::<TransactionWrite>(method).await?;

    let mut pair_occurances: HashMap<String, u64> = std::collections::HashMap::new();

    let mut iterval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = iterval.tick() => {
                println!("Pairs: {:#?}", pair_occurances);
            }
            maybe_tx = subscription.next() => {
                handle_transaction(maybe_tx, &mut pair_occurances);
            }
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    Ok(())
}

fn handle_transaction(
    maybe_tx: Option<Result<TransactionWrite, serde_json::Error>>,
    pair_occurances: &mut HashMap<String, u64>,
) {
    match maybe_tx {
        Some(Ok(transaction)) => {
            match parse_raydium_anchor_events(&RAYDIUM_CLMM_PROGRAM, transaction.meta().clone()) {
                Ok(event) => {
                    if let Some(RaydiumCLMMEvent::Swap(swap_event)) = event.get(0) {
                        let pair = format!(
                            "{:?} -> {:?}",
                            swap_event.token_account_0, swap_event.token_account_1
                        );
                        pair_occurances
                            .entry(pair)
                            .and_modify(|e| *e += 1)
                            .or_insert(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing program event: {:?}", e);
                }
            }
        }
        Some(Err(e)) => {
            eprintln!("Error parsing transaction: {:?}", e);
        }
        None => {
            eprintln!("Transaction stream ended");
        }
    }
}
