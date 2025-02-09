#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod chainstream;
mod raydium;

use raydium::anchor_events;

use crate::{
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

    Ok(())
}
