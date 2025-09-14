use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::instructions::errors::*;
use crate::state::Payment;

#[derive(Accounts)]
#[instruction(reference: Pubkey)]
pub struct ReceivePayment<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: merchant receives the payment
    #[account(mut)]
    pub merchant: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"payment", payer.key().as_ref(), reference.as_ref()],
        bump,
        has_one = payer,
        has_one = merchant,
    )]
    pub payment: Account<'info, Payment>,
    // Optional token accounts when payment was via token
    #[account(mut)]
    pub merchant_token_account: Option<Account<'info, TokenAccount>>,
    pub token_program: Option<Program<'info, Token>>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> ReceivePayment<'info> {
    pub fn process(&mut self, reference: Pubkey, is_token_payment: bool) -> Result<()> {
        // Ensure the payment is not already claimed
        if self.payment.claimed {
            return Err(PaymentError::Unauthorized.into());
        }

        // For this project design, actual fund transfers are done at `send` time.
        // `receive` simply marks the payment as claimed by the merchant.
        self.payment.claimed = true;
        self.payment.timestamp = self.clock.unix_timestamp;

        Ok(())
    }
}

pub fn receive_payment(ctx: Context<ReceivePayment>, reference: Pubkey, is_token_payment: bool) -> Result<()> {
    ctx.accounts.process(reference, is_token_payment)
}
