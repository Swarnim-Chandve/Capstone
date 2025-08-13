use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, Transfer},
    associated_token::AssociatedToken,
};

use crate::state::{DaoConfig, Vesting, VestingType, VestingStatus, PaymentCategory};
use crate::errors::MiraiError;
use crate::CreateVesting;

pub fn handler(
    ctx: Context<CreateVesting>,
    vesting_type: VestingType,
    total_amount: u64,
    start_time: i64,
    end_time: i64,
    cliff_time: i64,
    category: PaymentCategory,
    description: String,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate vesting parameters
    require!(total_amount > 0, MiraiError::InvalidTotalAmount);
    require!(start_time < end_time, MiraiError::InvalidVestingTiming);
    require!(start_time > current_time, MiraiError::InvalidVestingTiming);
    
    // Validate cliff timing for cliff vesting
    if matches!(vesting_type, VestingType::Cliff) {
        require!(cliff_time >= start_time && cliff_time <= end_time, MiraiError::InvalidCliffTiming);
    }
    
    // Validate description length
    require!(description.len() <= 64, MiraiError::DescriptionTooLong);
    
    // Store keys for later use
    let dao_config_key = ctx.accounts.dao_config.key();
    let authority_key = ctx.accounts.authority.key();
    let recipient_key = ctx.accounts.recipient.key();
    let treasury_mint_key = ctx.accounts.treasury_mint.key();
    let vesting_ata_key = ctx.accounts.vesting_ata.key();
    
    // Get DAO config and validate treasury is active
    let dao_config = &mut ctx.accounts.dao_config;
    require!(dao_config.is_treasury_active(), MiraiError::TreasuryPaused);
    require!(dao_config.validate_stream_amount(total_amount), MiraiError::StreamAmountExceedsLimit);
    require!(dao_config.validate_total_allocation(total_amount), MiraiError::TotalAllocationExceedsLimit);
    
    // Initialize vesting account
    let vesting = &mut ctx.accounts.vesting;
    vesting.authority = authority_key;
    vesting.recipient = recipient_key;
    vesting.dao_config = dao_config_key;
    vesting.treasury_mint = treasury_mint_key;
    vesting.vesting_ata = vesting_ata_key;
    vesting.vesting_type = vesting_type.clone();
    vesting.total_amount = total_amount;
    vesting.claimed_amount = 0;
    vesting.start_time = start_time;
    vesting.end_time = end_time;
    vesting.cliff_time = cliff_time;
    vesting.status = VestingStatus::Active;
    vesting.category = category.clone();
    vesting.description = description;
    vesting.bump = ctx.bumps.vesting;
    vesting.created_at = current_time;
    
    // Update DAO config statistics
    dao_config.add_stream(total_amount);
    
    // Transfer tokens from authority to vesting account
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.authority_ata.to_account_info(),
            to: ctx.accounts.vesting_ata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    );
    
    anchor_spl::token::transfer(transfer_ctx, total_amount)?;
    
    msg!("Vesting created successfully");
    msg!("Recipient: {}", ctx.accounts.recipient.key());
    msg!("Amount: {}", total_amount);
    msg!("Type: {:?}", vesting_type);
    msg!("Category: {:?}", category);
    msg!("Start: {}, End: {}", start_time, end_time);
    
    Ok(())
} 