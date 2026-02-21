use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DarkConfig {
    pub admin: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub mint: Pubkey,
    pub bump: u8,
    pub amount: u64,
    pub is_initialized: bool,
}
