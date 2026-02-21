use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DarkConfig {
    pub admin: Pubkey,
    pub bump: u8,
}