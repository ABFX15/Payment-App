use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer as SplTransfer};
use crate::instructions::errors::*;
use crate::state::Payment; 


#[derive(Accounts)]
#[instruction(amount: u64, reference: Pubkey)]
pub struct SendPayment<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This is the merchant receiving the payment
    #[account(mut)]
    pub merchant: AccountInfo<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + Payment::LEN,
        seeds = [b"payment", payer.key().as_ref(), reference.as_ref()],
        bump,
    )]
    pub payment: Account<'info, Payment>,
    // Optional token accounts for stablecoin payments
    #[account(mut)]
    pub payer_token_account: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub merchant_token_account: Option<Account<'info, TokenAccount>>,
    // Optional mint for token payments
    pub mint: Option<Account<'info, Mint>>,
    pub token_program: Option<Program<'info, Token>>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> SendPayment<'info> {
    pub fn validate(&self, amount: u64, is_token_payment: bool) -> Result<()> {
        if amount == 0 {
            return Err(PaymentError::InvalidAmount.into());
        }
        
        if is_token_payment {
            // Validate token payment requirements
            if self.payer_token_account.is_none() || self.merchant_token_account.is_none() {
                return Err(PaymentError::Unauthorized.into());
            }
            
            let payer_token_balance = self.payer_token_account.as_ref().unwrap().amount;
            if payer_token_balance < amount {
                return Err(PaymentError::InsufficientFunds.into());
            }
        } else {
            // Validate SOL payment requirements
            let payer_balance = self.payer.lamports();
            if payer_balance < amount {
                return Err(PaymentError::InsufficientFunds.into());
            }
        }
        
        Ok(())
    }

    pub fn process(&mut self, amount: u64, reference: Pubkey, is_token_payment: bool) -> Result<()> {
        self.validate(amount, is_token_payment)?;
        
        if is_token_payment {
            // Transfer tokens
            token::transfer(
                CpiContext::new(
                    self.token_program.as_ref().unwrap().to_account_info(),
                    SplTransfer {
                        from: self.payer_token_account.as_ref().unwrap().to_account_info(),
                        to: self.merchant_token_account.as_ref().unwrap().to_account_info(),
                        authority: self.payer.to_account_info(),
                    },
                ),
                amount,
            )?;
            
            // Set mint for token payment
            self.payment.mint = self.mint.as_ref().unwrap().key();
        } else {
            // Transfer SOL
            system_program::transfer(
                CpiContext::new(
                    self.system_program.to_account_info(),
                    system_program::Transfer {
                        from: self.payer.to_account_info(),
                        to: self.merchant.to_account_info(),
                    },
                ),
                amount,
            )?;
            
            // Set default mint for SOL payment
            self.payment.mint = Pubkey::default();
        }
        
        // Update payment state
        self.payment.payer = self.payer.key();
        self.payment.merchant = self.merchant.key();
        self.payment.amount = amount;
        self.payment.reference = reference;
        self.payment.timestamp = self.clock.unix_timestamp;
        self.payment.claimed = false;
        
        Ok(())
    }
}

pub fn handler(ctx: Context<SendPayment>, amount: u64, reference: Pubkey, is_token_payment: bool) -> Result<()> {
    ctx.accounts.process(amount, reference, is_token_payment)?;
    Ok(())
}