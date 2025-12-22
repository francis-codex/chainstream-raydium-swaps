# ChainStream API <--> Raydium Swaps

A Rust library and example application for streaming real-time token swap events from Raydium's Concentrated Liquidity Market Maker (CLMM) program on Solana using Syndica's ChainStream API.

## Features

- **Real-time Streaming**: WebSocket-based connection to Syndica's ChainStream API for live transaction data
- **Anchor Event Parsing**: Decode and parse all Raydium CLMM events including swaps, liquidity changes, and more
- **Type-Safe API**: Comprehensive type definitions for all Raydium CLMM events
- **Configurable Filters**: Filter transactions by account keys, commitment level, and more
- **Structured Logging**: Built-in `tracing` support for production-ready logging

## Prerequisites

1. **Rust**: Install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).

2. **ChainStream API Access**: Sign up for a [Syndica account](https://app.syndica.io/signup) and follow the [ChainStream API documentation](https://docs.syndica.io/platform/chainstream-api) to get your API token.

## Running the Example

Set your Syndica API token as an environment variable:

```bash
export SYNDICA_TOKEN=<your-syndica-token>
```

Run the example:

```bash
cargo run
```

Or run the complete example binary:

```bash
cargo run --bin complete-example
```

The application will start streaming Raydium CLMM swap events and display the token flow direction. Press `Ctrl+C` to stop.

### Configuring Log Level

Control the log verbosity with the `RUST_LOG` environment variable:

```bash
# Show info level logs (default)
RUST_LOG=info cargo run

# Show debug logs for detailed event parsing
RUST_LOG=debug cargo run

# Show only errors
RUST_LOG=error cargo run
```

## Library Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
chainstream-raydium-trade-pair = { path = "." }
```

Example usage:

```rust
use chainstream_raydium_trade_pair::{
    chainstream::{client::ChainStreamClient, methods::{Method, CommitmentLevel}},
    raydium::{parse::{parse_raydium_anchor_events, RAYDIUM_CLMM_PROGRAM}, anchor_events::RaydiumCLMMEvent},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = ChainStreamClient::new(&std::env::var("SYNDICA_TOKEN")?).await?;

    let method = Method::new_transaction_subscription()
        .one_of_account_keys(&[RAYDIUM_CLMM_PROGRAM])
        .commitment_level(CommitmentLevel::Confirmed);

    let mut subscription = client.subscribe(method).await?;

    while let Some(Ok(tx)) = subscription.next().await {
        for event in parse_raydium_anchor_events(tx.meta())? {
            match event {
                RaydiumCLMMEvent::Swap(swap) => println!("Swap: {:?}", swap),
                RaydiumCLMMEvent::PoolCreated(pool) => println!("Pool created: {:?}", pool),
                _ => {}
            }
        }
    }
    Ok(())
}
```

## Supported Events

The library parses all Raydium CLMM events:

| Event | Description |
|-------|-------------|
| `Swap` | Token swap between two assets |
| `PoolCreated` | New liquidity pool creation |
| `LiquidityChange` | Pool liquidity modifications |
| `IncreaseLiquidity` | Liquidity provider deposits |
| `DecreaseLiquidity` | Liquidity provider withdrawals |
| `CreatePersonalPosition` | New LP position creation |
| `CollectProtocolFee` | Protocol fee collection |
| `CollectPersonalFee` | LP fee collection |
| `ConfigChange` | Pool configuration updates |
| `UpdateRewardInfos` | Reward information updates |

## Architecture

```
src/
├── lib.rs                    # Library entry point
├── main.rs                   # Default binary
├── chainstream/              # ChainStream API client
│   ├── client.rs             # WebSocket client
│   ├── methods.rs            # Subscription methods
│   └── types.rs              # Response types
├── raydium/                  # Raydium event parsing
│   ├── anchor_events.rs      # Event type definitions
│   └── parse.rs              # Event decoder
└── bin/
    └── complete_example.rs   # Complete example binary
```

## License

See the LICENSE file for details.