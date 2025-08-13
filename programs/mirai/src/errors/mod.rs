use anchor_lang::prelude::*;

#[error_code]
pub enum MiraiError {
    // V1 Errors (preserved)
    #[msg("Invalid stream timing: start time must be before end time")]
    InvalidStreamTiming,
    
    #[msg("Stream has not started yet")]
    StreamNotStarted,
    
    #[msg("Stream has already ended")]
    StreamEnded,
    
    #[msg("Insufficient unlocked tokens available for withdrawal")]
    InsufficientUnlockedTokens,
    
    #[msg("Invalid withdrawal amount: must be greater than 0")]
    InvalidWithdrawalAmount,
    
    #[msg("Stream duration must be at least 1 second")]
    InvalidStreamDuration,
    
    #[msg("Total amount must be greater than 0")]
    InvalidTotalAmount,
    
    #[msg("Unauthorized: only the recipient can withdraw from this stream")]
    UnauthorizedWithdrawal,
    
    #[msg("Unauthorized: only the DAO authority can create streams")]
    UnauthorizedStreamCreation,
    
    #[msg("Invalid mint: stream mint must match DAO treasury mint")]
    InvalidMint,
    
    #[msg("Invalid DAO configuration")]
    InvalidDaoConfig,
    
    #[msg("Stream already exists for this recipient")]
    StreamAlreadyExists,
    
    #[msg("Invalid token account")]
    InvalidTokenAccount,

    // V2 Treasury Management Errors
    #[msg("Treasury operations are currently paused")]
    TreasuryPaused,
    
    #[msg("Stream amount exceeds maximum allowed per stream")]
    StreamAmountExceedsLimit,
    
    #[msg("Total allocation would exceed treasury limits")]
    TotalAllocationExceedsLimit,
    
    #[msg("Invalid payment category")]
    InvalidPaymentCategory,
    
    #[msg("Stream description is too long (max 64 characters)")]
    DescriptionTooLong,
    
    #[msg("Stream is not in active status")]
    StreamNotActive,
    
    #[msg("Stream is paused and cannot be modified")]
    StreamPaused,
    
    #[msg("Stream is completed and cannot be modified")]
    StreamCompleted,
    
    #[msg("Stream is cancelled and cannot be modified")]
    StreamCancelled,
    
    #[msg("Unauthorized: only DAO authority can modify treasury settings")]
    UnauthorizedTreasuryModification,
    
    #[msg("Invalid governance settings")]
    InvalidGovernanceSettings,
    
    #[msg("Stream modification not allowed in current state")]
    StreamModificationNotAllowed,
    
    #[msg("Insufficient treasury balance for operation")]
    InsufficientTreasuryBalance,
    
    #[msg("Invalid stream status transition")]
    InvalidStatusTransition,

    // V2 Vesting Errors
    #[msg("Vesting is not in active status")]
    VestingNotActive,
    
    #[msg("Vesting is not paused")]
    VestingNotPaused,
    
    #[msg("Vesting cannot be cancelled in current state")]
    VestingCannotCancel,
    
    #[msg("Invalid vesting timing: start time must be before end time")]
    InvalidVestingTiming,
    
    #[msg("Invalid cliff timing: cliff time must be between start and end time")]
    InvalidCliffTiming,
    
    #[msg("Vesting has not started yet")]
    VestingNotStarted,
    
    #[msg("Vesting has already ended")]
    VestingEnded,
    
    #[msg("Insufficient vested tokens available for claim")]
    InsufficientVestedTokens,
    
    #[msg("Invalid claim amount: must be greater than 0")]
    InvalidClaimAmount,
    
    #[msg("Unauthorized: only the recipient can claim from this vesting")]
    UnauthorizedVestingClaim,
    
    #[msg("Unauthorized: only the DAO authority can create vesting")]
    UnauthorizedVestingCreation,
    
    #[msg("Vesting already exists for this recipient")]
    VestingAlreadyExists,
    
    #[msg("Invalid vesting type")]
    InvalidVestingType,
} 