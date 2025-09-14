use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use crate::state::Payment;
use crate::instructions::errors::*;

#[derive(Accounts)]
#[instruction(amount: u64, reference: Pubkey)]
pub struct RefundPayment<'info> {
    /// The original payer who will receive the refund
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Merchant who holds funds and must sign for refunds
    #[account(mut)]
    pub merchant: Signer<'info>,

    /// The payment account PDA
    #[account(
        mut,
        seeds = [b"payment", payer.key().as_ref(), reference.as_ref()],
        bump,
        has_one = payer,
        has_one = merchant,
    )]
    pub payment: Account<'info, Payment>,

    // For token refunds, token accounts and token program
    #[account(mut)]
    pub merchant_token_account: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub payer_token_account: Option<Account<'info, TokenAccount>>,
    pub token_program: Option<Program<'info, Token>>,

    pub system_program: Program<'info, System>,
}

impl<'info> RefundPayment<'info> {
    pub fn process(&mut self, amount: u64, reference: Pubkey, is_token_payment: bool) -> Result<()> {
        // validations
        if self.payment.claimed {
            return Err(RefundError::PaymentAlreadyClaimed.into());
        }

        if amount == 0 || amount > self.payment.amount {
            return Err(RefundError::RefundExceedsPayment.into());
        }

        if self.payment.refunded {
            return Err(RefundError::RefundAlreadyProcessed.into());
        }

        if is_token_payment {
            // ensure token fields present
            let merchant_token_acct = self.merchant_token_account.as_ref().ok_or(RefundError::InvalidRefundReason)?;
            let payer_token_acct = self.payer_token_account.as_ref().ok_or(RefundError::InvalidRefundReason)?;
            let token_program = self.token_program.as_ref().ok_or(RefundError::InvalidRefundReason)?;

            // perform token transfer from merchant -> payer using merchant as authority
            let cpi_accounts = SplTransfer {
                from: merchant_token_acct.to_account_info(),
                to: payer_token_acct.to_account_info(),
                authority: self.merchant.to_account_info(),
            };

            token::transfer(
                CpiContext::new(token_program.to_account_info(), cpi_accounts),
                amount,
            )?;
        } else {
            // Transfer SOL from merchant to payer. Merchant must be signer.
            **self.merchant.to_account_info().try_borrow_mut_lamports()? -= amount;
            **self.payer.to_account_info().try_borrow_mut_lamports()? += amount;
        }

        // mark refunded and record amount (could decrement remaining amount)
        self.payment.refunded = true;

        Ok(())
    }
}

pub fn refund_payment(ctx: Context<RefundPayment>, amount: u64, reference: Pubkey, is_token_payment: bool) -> Result<()> {
    ctx.accounts.process(amount, reference, is_token_payment)
}