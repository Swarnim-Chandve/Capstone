#![allow(unexpected_cfgs,deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;


use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::{DaoConfig, Stream, PaymentCategory, Vesting, VestingType};
use crate::error::MiraiError;

declare_id!("DMiFVyoupSnwoWs2xq8n5X3hwtaaU6vLv9EkdSFoNsSv");

#[program]
pub mod mirai {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
