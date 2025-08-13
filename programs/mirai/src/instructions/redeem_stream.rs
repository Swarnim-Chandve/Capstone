use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, Transfer},
    associated_token::AssociatedToken,
};
use crate::errors::MiraiError;
use crate::state::{DaoConfig, Stream, StreamStatus};
use crate::RedeemStream;

pub fn handler(ctx: Context<RedeemStream>, amount: u64) -> Result<()> {
    let dao_config_key = ctx.accounts.dao_config.key();
    let dao_config = &mut ctx.accounts.dao_config;
    let stream_bump = ctx.accounts.stream.bump;
    let stream_to_account_info = ctx.accounts.stream.to_account_info();
    let recipient_key = ctx.accounts.recipient.key();
    let stream = &mut ctx.accounts.stream;
    let clock = Clock::get()?;
    
    
    require!(amount > 0, MiraiError::InvalidWithdrawalAmount);
    require!(
        clock.unix_timestamp >= stream.start_time,
        MiraiError::StreamNotStarted
    );
    
  
    require!(stream.status == StreamStatus::Active, MiraiError::StreamNotActive);
    
    let available_amount = stream.get_withdrawable_amount(clock.unix_timestamp);
    require!(
        amount <= available_amount,
        MiraiError::InsufficientUnlockedTokens
    );
    
    stream.withdrawn_amount = stream.withdrawn_amount.checked_add(amount)
        .ok_or(MiraiError::InsufficientUnlockedTokens)?;
    
    // Update treasury statistics
    dao_config.add_payment(amount);
    
    // Check if stream is completed
    if stream.withdrawn_amount >= stream.total_amount {
        stream.status = StreamStatus::Completed;
    }
    
    let seeds = &[
        b"stream",
        dao_config_key.as_ref(),
        recipient_key.as_ref(),
        &[stream_bump],
    ];
    let signer_seeds = &[&seeds[..]];
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.stream_ata.to_account_info(),
            to: ctx.accounts.recipient_ata.to_account_info(),
            authority: stream_to_account_info,
        },
        signer_seeds,
    );
    anchor_spl::token::transfer(transfer_ctx, amount)?;
    
    msg!("Stream redeemed successfully with treasury tracking");
    msg!("Amount withdrawn: {}", amount);
    msg!("Total withdrawn: {}", stream.withdrawn_amount);
    msg!("Remaining: {}", stream.get_remaining_amount());
    msg!("Stream Status: {:?}", stream.status);
    msg!("Treasury Total Paid: {}", dao_config.total_paid);
    
    if stream.status == StreamStatus::Completed {
        msg!("Stream completed - all tokens withdrawn");
    }
    
    Ok(())
} 