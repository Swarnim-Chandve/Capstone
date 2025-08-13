#![allow(unexpected_cfgs,deprecated)]
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;


use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::{DaoConfig, Stream, PaymentCategory, Vesting, VestingType};
use crate::errors::MiraiError;

declare_id!("DMiFVyoupSnwoWs2xq8n5X3hwtaaU6vLv9EkdSFoNsSv");

#[program]
pub mod mirai {
    use super::*;

   
    pub fn init_dao(
        ctx: Context<InitDao>,
        treasury_mint: Pubkey,
    ) -> Result<()> {
        instructions::init_dao::handler(ctx, treasury_mint)
    }

    pub fn create_stream(
        ctx: Context<CreateStream>,
        start_time: i64,
        end_time: i64,
        total_amount: u64,
        category: PaymentCategory,
        description: String,
    ) -> Result<()> {
        instructions::create_stream::handler(ctx, start_time, end_time, total_amount, category, description)
    }

    pub fn redeem_stream(
        ctx: Context<RedeemStream>,
        amount: u64,
    ) -> Result<()> {
        instructions::redeem_stream::handler(ctx, amount)
    }

    pub fn create_vesting(
        ctx: Context<CreateVesting>,
        vesting_type: VestingType,
        total_amount: u64,
        start_time: i64,
        end_time: i64,
        cliff_time: i64,
        category: PaymentCategory,
        description: String,
    ) -> Result<()> {
        instructions::create_vesting::handler(ctx, vesting_type, total_amount, start_time, end_time, cliff_time, category, description)
    }

    pub fn claim_vesting(
        ctx: Context<ClaimVesting>,
        amount: u64,
    ) -> Result<()> {
        instructions::claim_vesting::handler(ctx, amount)
    }

}





#[derive(Accounts)]
pub struct InitDao<'info> {

    #[account(
        init,
        payer = authority,
        space= DaoConfig::SIZE,
        seeds = [b"dao_config", authority.key().as_ref()],
         bump
    )]
    pub dao_config: Account<'info, DaoConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub treasury_mint: Account<'info,Mint>,

    pub system_program: Program<'info,System>,

    pub token_program: Program<'info, Token>,

    pub rend: Sysvar<'info, Rent>,
}




#[derive(Accounts)]
#[instruction(start_time: i64, end_time: i64, total_amount: u64, category: PaymentCategory, description: String)]
pub struct CreateStream<'info> {
    #[account(
        mut,
        seeds = [b"dao_config", authority.key().as_ref()],
        bump = dao_config.bump,
        has_one = authority @ MiraiError::UnauthorizedStreamCreation,
        has_one = treasury_mint @ MiraiError::InvalidMint
    )]
    pub dao_config: Account<'info, DaoConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: This is the recipient of the stream
    pub recipient: UncheckedAccount<'info>,
    #[account(
        init,
        payer = authority,
        space = Stream::SIZE,
        seeds = [
            b"stream",
            dao_config.key().as_ref(),
            recipient.key().as_ref()
        ],
        bump
    )]
    pub stream: Account<'info, Stream>,
    #[account(
        init,
        payer = authority,
        associated_token::mint = treasury_mint,
        associated_token::authority = stream
    )]
    pub stream_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = treasury_mint.key() == dao_config.treasury_mint @ MiraiError::InvalidMint
    )]
    pub treasury_mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = authority_ata.mint == treasury_mint.key() @ MiraiError::InvalidMint,
        constraint = authority_ata.owner == authority.key() @ MiraiError::UnauthorizedStreamCreation
    )]
    pub authority_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct RedeemStream<'info> {
    #[account(
        mut,
        seeds = [b"dao_config", dao_config.authority.as_ref()],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    #[account(
        mut,
        seeds = [
            b"stream",
            dao_config.key().as_ref(),
            recipient.key().as_ref()
        ],
        bump = stream.bump,
        has_one = recipient @ MiraiError::UnauthorizedWithdrawal,
        has_one = dao_config @ MiraiError::InvalidDaoConfig
    )]
    pub stream: Account<'info, Stream>,
    #[account(mut)]
    pub recipient: Signer<'info>,
    #[account(
        mut,
        constraint = stream_ata.key() == stream.stream_ata @ MiraiError::InvalidTokenAccount,
        constraint = stream_ata.mint == treasury_mint.key() @ MiraiError::InvalidMint
    )]
    pub stream_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = recipient,
        associated_token::mint = treasury_mint,
        associated_token::authority = recipient
    )]
    pub recipient_ata: Account<'info, TokenAccount>,
    #[account(
        constraint = treasury_mint.key() == dao_config.treasury_mint @ MiraiError::InvalidMint
    )]
    pub treasury_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(vesting_type: VestingType, total_amount: u64, start_time: i64, end_time: i64, cliff_time: i64, category: PaymentCategory, description: String)]
pub struct CreateVesting<'info> {
    #[account(
        mut,
        seeds = [b"dao_config", authority.key().as_ref()],
        bump = dao_config.bump,
        has_one = authority @ MiraiError::UnauthorizedVestingCreation,
        has_one = treasury_mint @ MiraiError::InvalidMint
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: This is the recipient of the vesting
    pub recipient: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = authority,
        space = Vesting::SIZE,
        seeds = [
            b"vesting",
            dao_config.key().as_ref(),
            recipient.key().as_ref()
        ],
        bump
    )]
    pub vesting: Account<'info, Vesting>,
    
    #[account(
        init,
        payer = authority,
        associated_token::mint = treasury_mint,
        associated_token::authority = vesting
    )]
    pub vesting_ata: Account<'info, TokenAccount>,
    
    #[account(
        constraint = treasury_mint.key() == dao_config.treasury_mint @ MiraiError::InvalidMint
    )]
    pub treasury_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        constraint = authority_ata.mint == treasury_mint.key() @ MiraiError::InvalidMint,
        constraint = authority_ata.owner == authority.key() @ MiraiError::UnauthorizedVestingCreation
    )]
    pub authority_ata: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ClaimVesting<'info> {
    #[account(
        mut,
        seeds = [b"dao_config", dao_config.authority.as_ref()],
        bump = dao_config.bump
    )]
    pub dao_config: Account<'info, DaoConfig>,
    
    #[account(
        mut,
        seeds = [
            b"vesting",
            dao_config.key().as_ref(),
            recipient.key().as_ref()
        ],
        bump = vesting.bump,
        has_one = recipient @ MiraiError::UnauthorizedVestingClaim,
        has_one = dao_config @ MiraiError::InvalidDaoConfig
    )]
    pub vesting: Account<'info, Vesting>,
    
    #[account(mut)]
    pub recipient: Signer<'info>,
    
    #[account(
        mut,
        constraint = vesting_ata.key() == vesting.vesting_ata @ MiraiError::InvalidTokenAccount,
        constraint = vesting_ata.mint == treasury_mint.key() @ MiraiError::InvalidMint
    )]
    pub vesting_ata: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = recipient,
        associated_token::mint = treasury_mint,
        associated_token::authority = recipient
    )]
    pub recipient_ata: Account<'info, TokenAccount>,
    
    #[account(
        constraint = treasury_mint.key() == dao_config.treasury_mint @ MiraiError::InvalidMint
    )]
    pub treasury_mint: Account<'info, Mint>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
