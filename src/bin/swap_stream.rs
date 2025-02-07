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

    while let Some(Ok(transaction)) = subscription.next().await {
        if let Ok(event) =
            parse_raydium_anchor_events(&RAYDIUM_CLMM_PROGRAM, transaction.meta().clone())
        {
            let signature = transaction.signature();
            if let Some(RaydiumCLMMEvent::Swap(swap_event)) = event.get(0) {
                println!(
                    "{:?} -> {:?} -- sig: {}",
                    swap_event.token_account_0, swap_event.token_account_1, signature
                );
            }
        }
    }

    Ok(())
}
