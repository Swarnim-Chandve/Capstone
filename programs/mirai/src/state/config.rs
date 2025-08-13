use anchor_lang::prelude::*;

/// Payment category for organizing treasury streams
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum PaymentCategory {
    Contributors,
    Grants,
    Operations,
    Marketing,
    Development,
    Other,
}

/// Governance settings for the DAO treasury
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GovernanceSettings {
    /// Whether treasury operations are paused
    pub is_paused: bool,
    /// Maximum amount that can be allocated per stream
    pub max_stream_amount: u64,
    /// Maximum total allocation across all streams
    pub max_total_allocation: u64,
    /// Timestamp when settings were last updated
    pub last_updated: i64,
}

/// DAO configuration account that stores treasury information and authority
#[account]
pub struct DaoConfig {
    /// Authority that can create streams and manage the DAO
    pub authority: Pubkey,
    /// SPL token mint used for treasury operations
    pub treasury_mint: Pubkey,
    /// Total number of active streams
    pub total_streams: u32,
    /// Total amount allocated across all streams
    pub total_allocated: u64,
    /// Total amount paid out across all streams
    pub total_paid: u64,
    /// Treasury governance settings
    pub governance_settings: GovernanceSettings,
    /// Bump seed for the PDA
    pub bump: u8,
    /// Timestamp when the DAO was created
    pub created_at: i64,
}

impl DaoConfig {
    /// Size of the account in bytes
    pub const SIZE: usize = 8 + // discriminator
        32 + 
        32 + 
        4 + 
        8 +  
        8 + 
        1 +  
        8 +  
        8 +  
        8 +  
        1 +  
        8;   

    /// Initialize governance settings with default values
    pub fn init_governance_settings() -> GovernanceSettings {
        GovernanceSettings {
            is_paused: false,
            max_stream_amount: u64::MAX,
            max_total_allocation: u64::MAX,
            last_updated: 0,
        }
    }

    /// Check if treasury operations are allowed
    pub fn is_treasury_active(&self) -> bool {
        !self.governance_settings.is_paused
    }

    /// Validate stream amount against limits
    pub fn validate_stream_amount(&self, amount: u64) -> bool {
        amount <= self.governance_settings.max_stream_amount
    }

    /// Validate total allocation against limits
    pub fn validate_total_allocation(&self, new_allocation: u64) -> bool {
        self.total_allocated.saturating_add(new_allocation) <= self.governance_settings.max_total_allocation
    }

    /// Update treasury statistics when creating a stream
    pub fn add_stream(&mut self, amount: u64) {
        self.total_streams = self.total_streams.saturating_add(1);
        self.total_allocated = self.total_allocated.saturating_add(amount);
    }

    /// Update treasury statistics when redeeming from a stream
    pub fn add_payment(&mut self, amount: u64) {
        self.total_paid = self.total_paid.saturating_add(amount);
    }
} 