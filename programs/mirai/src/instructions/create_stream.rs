use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, Transfer},
    associated_token::AssociatedToken,
};
use crate::errors::MiraiError;
use crate::state::{DaoConfig, Stream, PaymentCategory, StreamStatus};
use crate::CreateStream;

pub fn handler(
    ctx: Context<CreateStream>,
    start_time: i64,
    end_time: i64,
    total_amount: u64,
    category: PaymentCategory,
    description: String,
) -> Result<()> {
    let dao_config_key = ctx.accounts.dao_config.key();
    let dao_config = &mut ctx.accounts.dao_config;
    let stream = &mut ctx.accounts.stream;
    let clock = Clock::get()?;
    
  
    require!(total_amount > 0, MiraiError::InvalidTotalAmount);
    require!(start_time < end_time, MiraiError::InvalidStreamTiming);
    require!(
        end_time.saturating_sub(start_time) > 0,
        MiraiError::InvalidStreamDuration
    );
    
   
    require!(dao_config.is_treasury_active(), MiraiError::TreasuryPaused);
    require!(dao_config.validate_stream_amount(total_amount), MiraiError::StreamAmountExceedsLimit);
    require!(dao_config.validate_total_allocation(total_amount), MiraiError::TotalAllocationExceedsLimit);
    require!(description.len() <= 64, MiraiError::DescriptionTooLong);
    
  
    stream.dao_config = dao_config_key;
    stream.recipient = ctx.accounts.recipient.key();
    stream.authority = ctx.accounts.authority.key();
    stream.mint = ctx.accounts.treasury_mint.key();
    stream.total_amount = total_amount;
    stream.withdrawn_amount = 0;
    stream.start_time = start_time;
    stream.end_time = end_time;
    stream.stream_ata = ctx.accounts.stream_ata.key();
    stream.bump = ctx.bumps.stream;
    stream.created_at = clock.unix_timestamp;
    
    
    stream.category = category;
    stream.description = description;
    stream.status = StreamStatus::Active;
    
    // Update treasury statistics
    dao_config.add_stream(total_amount);
    
    // Transfer tokens
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.authority_ata.to_account_info(),
            to: ctx.accounts.stream_ata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    );
    anchor_spl::token::transfer(transfer_ctx, total_amount)?;
    
    msg!("Stream created successfully with treasury management");
    msg!("Recipient: {}", stream.recipient);
    msg!("Category: {:?}", stream.category);
    msg!("Description: {}", stream.description);
    msg!("Total Amount: {}", stream.total_amount);
    msg!("Start Time: {}", stream.start_time);
    msg!("End Time: {}", stream.end_time);
    msg!("Duration: {} seconds", end_time - start_time);
    msg!("Treasury Total Streams: {}", dao_config.total_streams);
    msg!("Treasury Total Allocated: {}", dao_config.total_allocated);
    
    Ok(())
} 