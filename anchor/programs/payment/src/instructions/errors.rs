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