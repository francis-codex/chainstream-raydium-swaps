//! Anchor events for emitted by the Raydium program. These events are helpful for tracking on-chain
//! state changes and can be used to trigger off-chain workflows.
//!
//! The events are defined as Rust structs with the `#[event]` attribute.
//!
//! For more information on Anchor events,
//! see the [Anchor documentation](https://project-serum.github.io/anchor/anchor_lang/events.html).
//!
//! These structures were lifted from the Raydium program source code,
//! which can be found [here](https://github.com/raydium-io/raydium-clmm/blob/master/programs/amm/src/states)
use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

// Number of rewards Token
const REWARD_NUM: usize = 3;

#[allow(unused)]
#[derive(Debug)]
pub enum RaydiumCLMMEvent {
    ConfigChange(ConfigChangeEvent),
    Swap(SwapEvent),
    PoolCreated(PoolCreatedEvent),
    CollectProtocolFee(CollectProtocolFeeEvent),
    LiquidityChange(LiquidityChangeEvent),
    CreatePersonalPosition(CreatePersonalPositionEvent),
    IncreaseLiquidity(IncreaseLiquidityEvent),
    DecreaseLiquidity(DecreaseLiquidityEvent),
    LiquidityCalculate(LiquidityCalculateEvent),
    CollectPersonalFee(CollectPersonalFeeEvent),
    UpdateRewardInfos(UpdateRewardInfosEvent),
    Unknown(String),
}

#[event]
#[derive(Debug)]
pub struct ConfigChangeEvent {
    pub index: u16,
    #[index]
    pub owner: Pubkey,
    pub protocol_fee_rate: u32,
    pub trade_fee_rate: u32,
    pub tick_spacing: u16,
    pub fund_fee_rate: u32,
    pub fund_owner: Pubkey,
}

/// Emitted when a pool is created and initialized with a starting price
///
#[event]
#[derive(Debug)]
pub struct PoolCreatedEvent {
    /// The first token of the pool by address sort order
    #[index]
    pub token_mint_0: Pubkey,

    /// The second token of the pool by address sort order
    #[index]
    pub token_mint_1: Pubkey,

    /// The minimum number of ticks between initialized ticks
    pub tick_spacing: u16,

    /// The address of the created pool
    pub pool_state: Pubkey,

    /// The initial sqrt price of the pool, as a Q64.64
    pub sqrt_price_x64: u128,

    /// The initial tick of the pool, i.e. log base 1.0001 of the starting price of the pool
    pub tick: i32,

    /// Vault of token_0
    pub token_vault_0: Pubkey,
    /// Vault of token_1
    pub token_vault_1: Pubkey,
}

/// Emitted when the collected protocol fees are withdrawn by the factory owner
#[event]
#[derive(Debug)]
pub struct CollectProtocolFeeEvent {
    /// The pool whose protocol fee is collected
    #[index]
    pub pool_state: Pubkey,

    /// The address that receives the collected token_0 protocol fees
    pub recipient_token_account_0: Pubkey,

    /// The address that receives the collected token_1 protocol fees
    pub recipient_token_account_1: Pubkey,

    /// The amount of token_0 protocol fees that is withdrawn
    pub amount_0: u64,

    /// The amount of token_0 protocol fees that is withdrawn
    pub amount_1: u64,
}

/// Emitted by when a swap is performed for a pool
#[event]
#[derive(Debug)]
pub struct SwapEvent {
    /// The pool for which token_0 and token_1 were swapped
    #[index]
    pub pool_state: Pubkey,

    /// The address that initiated the swap call, and that received the callback
    #[index]
    pub sender: Pubkey,

    /// The payer token account in zero for one swaps, or the recipient token account
    /// in one for zero swaps
    #[index]
    pub token_account_0: Pubkey,

    /// The payer token account in one for zero swaps, or the recipient token account
    /// in zero for one swaps
    #[index]
    pub token_account_1: Pubkey,

    /// The real delta amount of the token_0 of the pool or user
    pub amount_0: u64,

    /// The transfer fee charged by the withheld_amount of the token_0
    pub transfer_fee_0: u64,

    /// The real delta of the token_1 of the pool or user
    pub amount_1: u64,

    /// The transfer fee charged by the withheld_amount of the token_1
    pub transfer_fee_1: u64,

    /// if true, amount_0 is negtive and amount_1 is positive
    pub zero_for_one: bool,

    /// The sqrt(price) of the pool after the swap, as a Q64.64
    pub sqrt_price_x64: u128,

    /// The liquidity of the pool after the swap
    pub liquidity: u128,

    /// The log base 1.0001 of price of the pool after the swap
    pub tick: i32,
}

/// Emitted pool liquidity change when increase and decrease liquidity
#[event]
#[derive(Debug)]
pub struct LiquidityChangeEvent {
    /// The pool for swap
    #[index]
    pub pool_state: Pubkey,

    /// The tick of the pool
    pub tick: i32,

    /// The tick lower of position
    pub tick_lower: i32,

    /// The tick lower of position
    pub tick_upper: i32,

    /// The liquidity of the pool before liquidity change
    pub liquidity_before: u128,

    /// The liquidity of the pool after liquidity change
    pub liquidity_after: u128,
}

/// Emitted when create a new position
#[event]
#[derive(Debug)]
pub struct CreatePersonalPositionEvent {
    /// The pool for which liquidity was added
    #[index]
    pub pool_state: Pubkey,

    /// The address that create the position
    pub minter: Pubkey,

    /// The owner of the position and recipient of any minted liquidity
    pub nft_owner: Pubkey,

    /// The lower tick of the position
    #[index]
    pub tick_lower_index: i32,

    /// The upper tick of the position
    #[index]
    pub tick_upper_index: i32,

    /// The amount of liquidity minted to the position range
    pub liquidity: u128,

    /// The amount of token_0 was deposit for the liquidity
    pub deposit_amount_0: u64,

    /// The amount of token_1 was deposit for the liquidity
    pub deposit_amount_1: u64,

    /// The token transfer fee for deposit_amount_0
    pub deposit_amount_0_transfer_fee: u64,

    /// The token transfer fee for deposit_amount_1
    pub deposit_amount_1_transfer_fee: u64,
}

/// Emitted when liquidity is increased.
#[event]
#[derive(Debug)]
pub struct IncreaseLiquidityEvent {
    /// The ID of the token for which liquidity was increased
    #[index]
    pub position_nft_mint: Pubkey,

    /// The amount by which liquidity for the NFT position was increased
    pub liquidity: u128,

    /// The amount of token_0 that was paid for the increase in liquidity
    pub amount_0: u64,

    /// The amount of token_1 that was paid for the increase in liquidity
    pub amount_1: u64,

    /// The token transfer fee for amount_0
    pub amount_0_transfer_fee: u64,

    /// The token transfer fee for amount_1
    pub amount_1_transfer_fee: u64,
}

/// Emitted when liquidity is decreased.
#[event]
#[derive(Debug)]
pub struct DecreaseLiquidityEvent {
    /// The ID of the token for which liquidity was decreased
    pub position_nft_mint: Pubkey,
    /// The amount by which liquidity for the position was decreased
    pub liquidity: u128,
    /// The amount of token_0 that was paid for the decrease in liquidity
    pub decrease_amount_0: u64,
    /// The amount of token_1 that was paid for the decrease in liquidity
    pub decrease_amount_1: u64,
    // The amount of token_0 fee
    pub fee_amount_0: u64,
    /// The amount of token_1 fee
    pub fee_amount_1: u64,
    /// The amount of rewards
    pub reward_amounts: [u64; REWARD_NUM],
    /// The amount of token_0 transfer fee
    pub transfer_fee_0: u64,
    /// The amount of token_1 transfer fee
    pub transfer_fee_1: u64,
}

/// Emitted when liquidity decreased or increase.
#[event]
#[derive(Debug)]
pub struct LiquidityCalculateEvent {
    /// The pool liquidity before decrease or increase
    pub pool_liquidity: u128,
    /// The pool price when decrease or increase in liquidity
    pub pool_sqrt_price_x64: u128,
    /// The pool tick when decrease or increase in liquidity
    pub pool_tick: i32,
    /// The amount of token_0 that was calculated for the decrease or increase in liquidity
    pub calc_amount_0: u64,
    /// The amount of token_1 that was calculated for the decrease or increase in liquidity
    pub calc_amount_1: u64,
    // The amount of token_0 fee
    pub trade_fee_owed_0: u64,
    /// The amount of token_1 fee
    pub trade_fee_owed_1: u64,
    /// The amount of token_0 transfer fee without trade_fee_amount_0
    pub transfer_fee_0: u64,
    /// The amount of token_1 transfer fee without trade_fee_amount_0
    pub transfer_fee_1: u64,
}

/// Emitted when tokens are collected for a position
#[event]
#[derive(Debug)]
pub struct CollectPersonalFeeEvent {
    /// The ID of the token for which underlying tokens were collected
    #[index]
    pub position_nft_mint: Pubkey,

    /// The token account that received the collected token_0 tokens
    pub recipient_token_account_0: Pubkey,

    /// The token account that received the collected token_1 tokens
    pub recipient_token_account_1: Pubkey,

    /// The amount of token_0 owed to the position that was collected
    pub amount_0: u64,

    /// The amount of token_1 owed to the position that was collected
    pub amount_1: u64,
}

/// Emitted when Reward are updated for a pool
#[event]
#[derive(Debug)]
pub struct UpdateRewardInfosEvent {
    /// Reward info
    pub reward_growth_global_x64: [u128; REWARD_NUM],
}
