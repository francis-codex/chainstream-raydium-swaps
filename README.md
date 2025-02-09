# ChainStream API <--> Raydium Swaps (Example)

This example demonstrates how to use the Syndica's ChainStream API to obtain a stream of token swaps performed by Raydium's continous liquidity-pool AMM program. It's primarily intended as a demonstration of the ChainStream API, but can also be used as a starting point for building more complex applications and backened services that require real-time data from the Solana blockchain.

Ideally, you should have a basic understanding of the ChainStream API before diving into this example. If you haven't already, you can read the [ChainStream API documentation](https://docs.syndica.io/platform/chainstream-api) to get started.

## Prerequisites

Before running this example, you need to have the following:

Install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).

Access to the ChainStream API. You can get access by signing up for a Syndica account and creating a ChainStream API key. You can sign up for a Syndica account [here](https://app.syndica.io/signup). After which you can follow the guide [here](https://docs.syndica.io/platform/chainstream-api) to enable access to the ChainStream API.

## Running the Example

To run the example, clone the repository and navigate into the directory:

The example will start streaming in transactions from ChainStream API. You can stop the example by pressing `Ctrl+C`.

NOTE: You need to set the `SYNDICA_TOKEN` environment variable to your Syndica API token before running the example. You can get your API token from the Syndica dashboard.

```bash
export SYNDICA_TOKEN=<your-syndica-token>
cargo run
```

The default program that runs is empty since this example is intended to be built as a follow-along guide. To execute the complete example, you can run the following command:

```bash
export SYNDICA_TOKEN=<your-syndica-token>
cargo run --bin complete-example
```