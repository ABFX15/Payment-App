use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
pub use state::*;
pub use errors::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod payment {
    use super::*;

    pub fn send_payment(ctx: Context<SendPayment>, amount: u64, reference: Pubkey, is_token_payment: bool) -> Result<()> {
        ctx.accounts.process(amount, reference, is_token_payment)?;
        Ok(())
    }

    pub fn refund_payment(ctx: Context<RefundPayment>, amount: u64, reference: Pubkey, is_token_payment: bool) -> Result<()> {
        let mut refund_ctx = ctx.accounts;
        refund_ctx.process(amount, reference, is_token_payment)
    }
}
