use anchor_lang::prelude::*;

use crate::PaymentCategory;


#[derive(AnchorDeserialize, AnchorSerialize, Clone,PartialEq, Eq, Debug)]
pub enum VestingType{
    Linear,
    Cliff,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum  VestingStatus {
    Active,
    Paused,
    Completed,
    Cancelled
}


#[account]
pub struct Vesting{
    pub authority: Pubkey,

    pub recipient: Pubkey,

    pub dao_config: Pubkey,

    pub treasury_mint: Pubkey,

    pub vesting_ata: Pubkey,

    pub vesting_type: VestingType,

    pub total_amount: u64,

    pub claimed_amount: u64,

    pub start_time: i64,

    pub end_time: i64,

    pub cliff_time: i64,

    pub status: VestingStatus,

    pub category: PaymentCategory,

    pub description: String,

    pub bump: u8,

    pub created_at: i64,

}