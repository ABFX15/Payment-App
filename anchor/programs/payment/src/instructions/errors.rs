use anchor_lang::prelude::*;

#[error_code]
pub enum PaymentError {
    #[msg("Invalid payment amount")]
    InvalidAmount,
    #[msg("Payment not found")]
    PaymentNotFound,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}

#[error_code]
pub enum RefundError {
    #[msg("Refund amount exceeds original payment")]
    RefundExceedsPayment,
    #[msg("Refund already processed")]
    RefundAlreadyProcessed,
    #[msg("Invalid refund reason")]
    InvalidRefundReason,
    #[msg("Payment already claimed")]
    PaymentAlreadyClaimed,
}