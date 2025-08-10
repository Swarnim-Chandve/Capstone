use anchor_lang::prelude::*;
use crate::state::config::PaymentCategory;


#[derive(AnchorDeserialize,AnchorSerialize, Clone, PartialEq, Eq, Debug)]
pub enum  StreamStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
}


#[account]
pub struct Stream{
    pub dao_config: Pubkey,

    pub recipient: Pubkey,

    pub authority: Pubkey,

    pub mint: Pubkey,

    pub category: PaymentCategory,

    pub description: String,

    pub total_amount: u64,

    pub withdrawn_amount: u64,

    pub start_time: i64,


    pub end_time: i64,

    pub stream_ata: Pubkey,

    pub status: StreamStatus,

    pub bump: u8,

    pub created_at: i64






}