use anchor_lang::prelude::*;

#[error_code]
pub enum PaymentError {
    #[msg("Invalid payment amount")]
    InvalidAmount,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Unauthorized")]
    Unauthorized,
}