use anchor_lang::prelude::*;



#[account]
pub struct DaoConfig {
    pub authority: Pubkey,

    pub treasury_mint: Pubkey,

    pub total_streams: u32,

    pub total_allocated: u64,

    pub total_paid: u64,

    pub governance_settings: GovernanceSettings,

    pub bump: u8,

    pub created_at: i64,




}