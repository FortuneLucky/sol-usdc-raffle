use anchor_lang::prelude::*;

#[error_code]
pub enum RaffleError {
    #[msg("Amount should be bigger than 0")]
    InvalidAmount,
    #[msg("Raffle isn't finished")]
    NotFinishRaffle,
    #[msg("No VRF request yet!")]
    NoVRFRequest,
    #[msg("Raffle was already finished")]
    AlreadyFinished,
    #[msg("Over max count")]
    OverMaxCount,
    #[msg("Raffle had finished or not exist")]
    OutOfRaffle,
    #[msg("Alreay set winner")]
    AlreadySetWinner,
    #[msg("Error in set winner")]
    SetWinnerError,
    #[msg("Raffle already started")]
    StartedRaffle,
    #[msg("Too many ticket")]
    TooManyTicket,
    #[msg("Not winner")]
    NotWinner,
    #[msg("Error in claim prize, Already claimed")]
    ClaimPrizeError,
    #[msg("Paytoken mint address is wrong!")]
    PayTokenMintAddressError,
    #[msg("Referal should not be buyer!")]
    ReferralError,
    #[msg("Referal associated token account not provided!")]
    ReferralAtaError,
    #[msg("Invalid Admin provided!")]
    InvalidAdmin,
    #[msg("Invalid Account fee Sol Amount!")]
    InvalidAccountFeeAmount,
    #[msg("Still processing VRF!")]
    StillProcessing,
    #[msg("Account is not initialized!")]
    UninitializedAccount
}