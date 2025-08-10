use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum PaymentCategory {
    Contributors,
    Grants,
    Operations,
    Marketing,
    Development,
    Other,
}









#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct GovernanceSettings{
    pub is_paused: bool,

    pub max_stream_amount: u64,

    pub_max_total_allocation: u64,

    pub last_updated: i64,


}



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