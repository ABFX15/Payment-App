use anchor_lang::prelude::*;

#[account]
pub struct Payment {
    pub payer: Pubkey,
    pub merchant: Pubkey,
    pub amount: u64,
    pub mint: Pubkey,
    pub reference: Pubkey,
    pub timestamp: i64,
    pub claimed: bool,
}

impl Payment {
    pub const LEN: usize = 32 + 32 + 8 + 32 + 32 + 8 + 1;
}

#[account]
pub struct Merchant {
    pub owner: Pubkey,
    pub name: String,
    pub description: String,
    pub website: String,
}

impl Merchant {
    pub const LEN: usize = 32 + 4 + 64 + 4 + 256 + 4 + 256; // 32 + String overhead + max lengths
}