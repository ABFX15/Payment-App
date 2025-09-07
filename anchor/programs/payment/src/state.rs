use anchor_lang::prelude::*;

#[account]
pub struct Payment {
    pub payer: Pubkey,
    pub merchant: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub reference: Pubkey,
    pub timestamp: i64,
    pub claimed: bool, 
}

#[account]
pub struct Merchant {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub website: String,
}