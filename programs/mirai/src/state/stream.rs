use anchor_lang::prelude::*;
use crate::state::config::PaymentCategory;

/// Status of a stream
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum StreamStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
}

/// Stream account representing a linear token stream
#[account]
pub struct Stream {
    /// DAO configuration this stream belongs to
    pub dao_config: Pubkey,
    /// Recipient of the stream
    pub recipient: Pubkey,
    /// Authority that created the stream
    pub authority: Pubkey,
    /// SPL token mint being streamed
    pub mint: Pubkey,
    /// Payment category for this stream
    pub category: PaymentCategory,
    /// Description of the stream
    pub description: String,
    /// Total amount to be streamed
    pub total_amount: u64,
    /// Amount already withdrawn
    pub withdrawn_amount: u64,
    /// Start time of the stream 
    pub start_time: i64,
    /// End time of the stream
    pub end_time: i64,
    /// Associated token account for the stream
    pub stream_ata: Pubkey,
    /// Current status of the stream
    pub status: StreamStatus,
    /// Bump seed for the PDA
    pub bump: u8,
    /// Timestamp when the stream was created
    pub created_at: i64,
}

impl Stream {
    /// Size of the account in bytes
    pub const SIZE: usize = 8 + // discriminator
        32 +
        32 + 
        32 + 
        32 + 
        1 +  
        64 + 
        8 +  
        8 +  
        8 +  
        8 + 
        32 + 
        1 +  
        1 + 
        8;   

    /// Calculate the amount of tokens that can be withdrawn at the current time
    pub fn get_withdrawable_amount(&self, current_time: i64) -> u64 {
        // If stream is paused or cancelled, no withdrawals allowed
        if self.status == StreamStatus::Paused || self.status == StreamStatus::Cancelled {
            return 0;
        }

        if current_time < self.start_time {
            return 0;
        }

        if current_time >= self.end_time {
            return self.total_amount.saturating_sub(self.withdrawn_amount);
        }

        // Linear interpolation: (current_time - start_time) / (end_time - start_time) * total_amount
        let time_elapsed = current_time.saturating_sub(self.start_time);
        let total_duration = self.end_time.saturating_sub(self.start_time);
        
        if total_duration == 0 {
            return 0;
        }

        let unlocked_amount = (self.total_amount as u128)
            .checked_mul(time_elapsed as u128)
            .unwrap_or(0)
            .checked_div(total_duration as u128)
            .unwrap_or(0) as u64;

        unlocked_amount.saturating_sub(self.withdrawn_amount)
    }

    /// Check if the stream is active (between start and end time)
    pub fn is_active(&self, current_time: i64) -> bool {
        self.status == StreamStatus::Active && 
        current_time >= self.start_time && 
        current_time < self.end_time
    }

    /// Check if the stream has ended
    pub fn has_ended(&self, current_time: i64) -> bool {
        current_time >= self.end_time
    }

    /// Check if the stream can be modified
    pub fn can_modify(&self) -> bool {
        self.status == StreamStatus::Active
    }

    /// Pause the stream
    pub fn pause(&mut self) {
        if self.status == StreamStatus::Active {
            self.status = StreamStatus::Paused;
        }
    }

    /// Resume the stream
    pub fn resume(&mut self) {
        if self.status == StreamStatus::Paused {
            self.status = StreamStatus::Active;
        }
    }

    /// Complete the stream
    pub fn complete(&mut self) {
        self.status = StreamStatus::Completed;
    }

    /// Cancel the stream
    pub fn cancel(&mut self) {
        self.status = StreamStatus::Cancelled;
    }

    /// Get the remaining amount in the stream
    pub fn get_remaining_amount(&self) -> u64 {
        self.total_amount.saturating_sub(self.withdrawn_amount)
    }

    /// Get the progress percentage of the stream
    pub fn get_progress_percentage(&self, current_time: i64) -> u8 {
        if current_time < self.start_time {
            return 0;
        }

        if current_time >= self.end_time {
            return 100;
        }

        let time_elapsed = current_time.saturating_sub(self.start_time);
        let total_duration = self.end_time.saturating_sub(self.start_time);
        
        if total_duration == 0 {
            return 0;
        }

        let progress = (time_elapsed as u128)
            .checked_mul(100)
            .unwrap_or(0)
            .checked_div(total_duration as u128)
            .unwrap_or(0) as u8;

        progress.min(100)
    }
} 