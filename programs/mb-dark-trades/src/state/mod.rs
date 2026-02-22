use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DarkConfig {
    pub admin: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct DepositAccount {
    pub id: u64,
    pub bump: u8,
    pub amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct IntentAccount {
    pub id: u64,
    pub owner: Pubkey,
    pub side: u8, // 0: Buy, 1: Sell
    pub quantity: u64,
    pub limit_price: u64,
    pub expiry: i64,
    pub is_matched: bool,
}
