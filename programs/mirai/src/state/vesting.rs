use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum VestingType {
    Linear,
    Cliff,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum VestingStatus {
    Active,
    Paused,
    Completed,
    Cancelled,
}

#[account]
pub struct Vesting {
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
    pub cliff_time: i64,             // Cliff time (for cliff vesting)
    pub status: VestingStatus,       
    pub category: PaymentCategory,   // Payment category
    pub description: String,        
    pub bump: u8,                    
    pub created_at: i64,           
}

impl Vesting {
    pub const SIZE: usize = 32 + 32 + 32 + 32 + 32 + 1 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 64 + 1 + 8;

    pub fn get_claimable_amount(&self, current_time: i64) -> u64 {
        if self.status != VestingStatus::Active {
            return 0;
        }

        if current_time < self.start_time {
            return 0;
        }

        match self.vesting_type {
            VestingType::Linear => self.get_linear_claimable(current_time),
            VestingType::Cliff => self.get_cliff_claimable(current_time),
        }
    }

    fn get_linear_claimable(&self, current_time: i64) -> u64 {
        if current_time >= self.end_time {
            return self.total_amount - self.claimed_amount;
        }

        let elapsed = current_time - self.start_time;
        let total_duration = self.end_time - self.start_time;
        
        if total_duration <= 0 {
            return 0;
        }

        let vested_amount = (self.total_amount as f64 * elapsed as f64 / total_duration as f64) as u64;
        let claimable = if vested_amount > self.claimed_amount {
            vested_amount - self.claimed_amount
        } else {
            0
        };

        claimable
    }

    fn get_cliff_claimable(&self, current_time: i64) -> u64 {
        if current_time < self.cliff_time {
            return 0;
        }

        if current_time >= self.end_time {
            return self.total_amount - self.claimed_amount;
        }

        let elapsed = current_time - self.cliff_time;
        let total_duration = self.end_time - self.cliff_time;
        
        if total_duration <= 0 {
            return 0;
        }

        let vested_amount = (self.total_amount as f64 * elapsed as f64 / total_duration as f64) as u64;
        let claimable = if vested_amount > self.claimed_amount {
            vested_amount - self.claimed_amount
        } else {
            0
        };

        claimable
    }

    pub fn is_active(&self) -> bool {
        self.status == VestingStatus::Active
    }

    pub fn can_claim(&self, current_time: i64) -> bool {
        self.is_active() && self.get_claimable_amount(current_time) > 0
    }

    pub fn get_progress_percentage(&self, current_time: i64) -> f64 {
        if self.total_amount == 0 {
            return 0.0;
        }

        let claimable = self.get_claimable_amount(current_time);
        let total_vested = claimable + self.claimed_amount;
        
        (total_vested as f64 / self.total_amount as f64) * 100.0
    }

    pub fn pause(&mut self) -> Result<()> {
        require!(self.status == VestingStatus::Active, MiraiError::VestingNotActive);
        self.status = VestingStatus::Paused;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        require!(self.status == VestingStatus::Paused, MiraiError::VestingNotPaused);
        self.status = VestingStatus::Active;
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<()> {
        require!(self.status == VestingStatus::Active || self.status == VestingStatus::Paused, MiraiError::VestingCannotCancel);
        self.status = VestingStatus::Cancelled;
        Ok(())
    }
}


use crate::state::config::PaymentCategory;
use crate::errors::MiraiError; 