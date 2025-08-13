use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::errors::MiraiError;
use crate::state::{DaoConfig, GovernanceSettings};
use crate::InitDao;

pub fn handler(ctx: Context<InitDao>, treasury_mint: Pubkey) -> Result<()> {
    let dao_config = &mut ctx.accounts.dao_config;
    let clock = Clock::get()?;
    
    require!(
        ctx.accounts.treasury_mint.key() == treasury_mint,
        MiraiError::InvalidMint
    );
    
    dao_config.authority = ctx.accounts.authority.key();
    dao_config.treasury_mint = treasury_mint;
    dao_config.bump = ctx.bumps.dao_config;
    dao_config.created_at = clock.unix_timestamp;
    
  
    dao_config.total_streams = 0;
    dao_config.total_allocated = 0;
    dao_config.total_paid = 0;
    dao_config.governance_settings = GovernanceSettings {
        is_paused: false,
        max_stream_amount: u64::MAX,
        max_total_allocation: u64::MAX,
        last_updated: clock.unix_timestamp,
    };
    
    msg!("DAO initialized successfully with treasury management");
    msg!("Authority: {}", dao_config.authority);
    msg!("Treasury Mint: {}", dao_config.treasury_mint);
    msg!("Created at: {}", dao_config.created_at);
    msg!("Treasury Status: Active");
    msg!("Max Stream Amount: {}", dao_config.governance_settings.max_stream_amount);
    msg!("Max Total Allocation: {}", dao_config.governance_settings.max_total_allocation);
    
    Ok(())
} 