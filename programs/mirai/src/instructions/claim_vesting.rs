use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, Transfer},
    associated_token::AssociatedToken,
};

use crate::state::{DaoConfig, Vesting, VestingStatus};
use crate::errors::MiraiError;
use crate::ClaimVesting;

pub fn handler(
    ctx: Context<ClaimVesting>,
    amount: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate claim amount
    require!(amount > 0, MiraiError::InvalidClaimAmount);
    
    // Store values before mutable borrow
    let dao_config_key = ctx.accounts.dao_config.key();
    let recipient_key = ctx.accounts.recipient.key();
    let vesting_bump = ctx.accounts.vesting.bump;
    let vesting_account_info = ctx.accounts.vesting.to_account_info();
    
    // Get vesting account and validate
    let vesting = &mut ctx.accounts.vesting;
    require!(vesting.status == VestingStatus::Active, MiraiError::VestingNotActive);
    require!(current_time >= vesting.start_time, MiraiError::VestingNotStarted);
    
    // Calculate claimable amount
    let claimable_amount = vesting.get_claimable_amount(current_time);
    require!(amount <= claimable_amount, MiraiError::InsufficientVestedTokens);
    
    // Update vesting account
    vesting.claimed_amount += amount;
    
    // Check if vesting is completed
    let is_completed = vesting.claimed_amount >= vesting.total_amount;
    if is_completed {
        vesting.status = VestingStatus::Completed;
    }
    
    // Update DAO config statistics
    let dao_config = &mut ctx.accounts.dao_config;
    dao_config.add_payment(amount);
    
  
    let seeds = &[
        b"vesting",
        dao_config_key.as_ref(),
        recipient_key.as_ref(),
        &[vesting_bump],
    ];
    let signer_seeds = &[&seeds[..]];
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vesting_ata.to_account_info(),
            to: ctx.accounts.recipient_ata.to_account_info(),
            authority: vesting_account_info,
        },
        signer_seeds,
    );
    
    anchor_spl::token::transfer(transfer_ctx, amount)?;
    
    msg!("Vesting claimed successfully");
    msg!("Recipient: {}", ctx.accounts.recipient.key());
    msg!("Amount: {}", amount);
    msg!("Total claimed: {}", vesting.claimed_amount);
    msg!("Remaining: {}", vesting.total_amount - vesting.claimed_amount);
    
    if vesting.status == VestingStatus::Completed {
        msg!("Vesting completed!");
    }
    
    Ok(())
} 