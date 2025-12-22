//! Parse Anchor events from instruction data.
//!
//! This code was mostly adapted from raydium-io's raydium-clmm repository:
//! https://github.com/raydium-io/raydium-clmm/blob/master/client/src/instructions/events_instructions_parse.rs

use std::sync::LazyLock;

use anchor_lang::{prelude::*, Discriminator};
use anyhow::anyhow;
use base64::engine::general_purpose;
use base64::Engine;
use regex::Regex;
use tracing::debug;

use crate::chainstream::types::transaction::Meta;

use super::anchor_events::*;

const PROGRAM_LOG: &str = "Program log: ";
const PROGRAM_DATA: &str = "Program data: ";

/// Raydium CLMM program address on Solana mainnet
pub const RAYDIUM_CLMM_PROGRAM: &str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";

/// Pre-compiled regex for parsing program invoke logs
static INVOKE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^Program (.*) invoke.*$").expect("Invalid invoke regex"));

/// Pre-compiled regex for parsing program success logs
static SUCCESS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^Program (.*) success*$").expect("Invalid success regex"));

/// Top-level event parser. Returns a list of parsed events (if any).
pub fn parse_raydium_anchor_events(meta: Meta) -> Result<Vec<RaydiumCLMMEvent>> {
    let mut parsed_events = Vec::new();
    let mut logs = &meta.log_messages[..];

    if logs.is_empty() {
        debug!("Transaction log is empty, no events to parse");
        return Ok(parsed_events);
    }

    if let Ok(mut execution) = Execution::new(&mut logs) {
        for l in logs {
            let (new_program, did_pop, maybe_event) =
                if !execution.is_empty() && RAYDIUM_CLMM_PROGRAM == execution.program() {
                    handle_program_log(RAYDIUM_CLMM_PROGRAM, l, true)?
                } else {
                    let (program, did_pop) = handle_system_log(RAYDIUM_CLMM_PROGRAM, l);
                    (program, did_pop, None)
                };

            if let Some(evt) = maybe_event {
                parsed_events.push(evt);
            }

            if let Some(new_program) = new_program {
                execution.push(new_program);
            }

            if did_pop {
                execution.pop();
            }
        }
    }

    Ok(parsed_events)
}

/// Minimal call stack simulation for tracking CPI context
struct Execution {
    stack: Vec<String>,
}

impl Execution {
    pub fn new(logs: &mut &[String]) -> anyhow::Result<Self> {
        let l = &logs[0];
        *logs = &logs[1..];

        let c = INVOKE_REGEX
            .captures(l)
            .ok_or_else(|| anyhow!("log parse error: {l}"))?;

        let program = c
            .get(1)
            .ok_or_else(|| anyhow!("log parse error: {l}"))?
            .as_str()
            .to_string();

        Ok(Self {
            stack: vec![program],
        })
    }

    pub fn program(&self) -> &str {
        self.stack
            .last()
            .expect("Execution stack should not be empty")
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push(&mut self, new_program: String) {
        self.stack.push(new_program);
    }

    pub fn pop(&mut self) {
        debug_assert!(!self.stack.is_empty(), "Cannot pop from empty stack");
        self.stack.pop();
    }
}

/// Attempt to parse a log line from the "current" program context.
/// Returns (optional_new_program, did_pop, parsed_event).
fn handle_program_log(
    self_program_str: &str,
    l: &str,
    with_prefix: bool,
) -> Result<(Option<String>, bool, Option<RaydiumCLMMEvent>)> {
    let log = if with_prefix {
        l.strip_prefix(PROGRAM_LOG)
            .or_else(|| l.strip_prefix(PROGRAM_DATA))
    } else {
        Some(l)
    };

    if let Some(log) = log {
        if l.starts_with("Program log:") {
            return Ok((None, false, None));
        }

        let borsh_bytes = match general_purpose::STANDARD.decode(log) {
            Ok(b) => b,
            Err(_) => {
                debug!(log = %log, "Could not base64 decode log");
                return Ok((None, false, None));
            }
        };

        if borsh_bytes.len() < 8 {
            debug!(log = %log, "Event data too short (< 8 bytes)");
            return Ok((None, false, None));
        }

        let mut disc = [0u8; 8];
        disc.copy_from_slice(&borsh_bytes[..8]);
        let mut slice: &[u8] = &borsh_bytes[8..];

        // Match recognized event discriminators
        let event = match disc {
            ConfigChangeEvent::DISCRIMINATOR => {
                let e = decode_event::<ConfigChangeEvent>(&mut slice)?;
                RaydiumCLMMEvent::ConfigChange(e)
            }
            SwapEvent::DISCRIMINATOR => {
                let e = decode_event::<SwapEvent>(&mut slice)?;
                RaydiumCLMMEvent::Swap(e)
            }
            PoolCreatedEvent::DISCRIMINATOR => {
                let e = decode_event::<PoolCreatedEvent>(&mut slice)?;
                RaydiumCLMMEvent::PoolCreated(e)
            }
            CollectProtocolFeeEvent::DISCRIMINATOR => {
                let e = decode_event::<CollectProtocolFeeEvent>(&mut slice)?;
                RaydiumCLMMEvent::CollectProtocolFee(e)
            }
            LiquidityChangeEvent::DISCRIMINATOR => {
                let e = decode_event::<LiquidityChangeEvent>(&mut slice)?;
                RaydiumCLMMEvent::LiquidityChange(e)
            }
            CreatePersonalPositionEvent::DISCRIMINATOR => {
                let e = decode_event::<CreatePersonalPositionEvent>(&mut slice)?;
                RaydiumCLMMEvent::CreatePersonalPosition(e)
            }
            IncreaseLiquidityEvent::DISCRIMINATOR => {
                let e = decode_event::<IncreaseLiquidityEvent>(&mut slice)?;
                RaydiumCLMMEvent::IncreaseLiquidity(e)
            }
            DecreaseLiquidityEvent::DISCRIMINATOR => {
                let e = decode_event::<DecreaseLiquidityEvent>(&mut slice)?;
                RaydiumCLMMEvent::DecreaseLiquidity(e)
            }
            LiquidityCalculateEvent::DISCRIMINATOR => {
                let e = decode_event::<LiquidityCalculateEvent>(&mut slice)?;
                RaydiumCLMMEvent::LiquidityCalculate(e)
            }
            CollectPersonalFeeEvent::DISCRIMINATOR => {
                let e = decode_event::<CollectPersonalFeeEvent>(&mut slice)?;
                RaydiumCLMMEvent::CollectPersonalFee(e)
            }
            UpdateRewardInfosEvent::DISCRIMINATOR => {
                let e = decode_event::<UpdateRewardInfosEvent>(&mut slice)?;
                RaydiumCLMMEvent::UpdateRewardInfos(e)
            }
            _ => RaydiumCLMMEvent::Unknown(l.to_string()),
        };

        Ok((None, false, Some(event)))
    } else {
        // If there's no recognized prefix, treat as a system log
        let (program, did_pop) = handle_system_log(self_program_str, l);
        Ok((program, did_pop, None))
    }
}

/// Handle system logs, i.e., lines that indicate a program "invoke" or "success".
/// Returns (optional_new_program, did_pop).
fn handle_system_log(this_program_str: &str, log: &str) -> (Option<String>, bool) {
    if log.starts_with(&format!("Program {this_program_str} invoke")) {
        (Some(this_program_str.to_string()), false)
    } else if log.contains("invoke") {
        (Some("cpi".to_string()), false)
    } else if SUCCESS_REGEX.is_match(log) {
        (None, true)
    } else {
        (None, false)
    }
}

/// Generic borsh decoding of an Event
fn decode_event<T: anchor_lang::Event + AnchorDeserialize>(slice: &mut &[u8]) -> Result<T> {
    Ok(T::deserialize(slice)?)
}
