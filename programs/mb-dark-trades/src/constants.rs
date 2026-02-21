use anchor_lang::prelude::*;

#[constant]
pub const DARK_CONFIG_SEEDS: &[u8] = b"dark_trades_config";

#[constant]
pub const USER_ACCOUNT_SEEDS: &[u8] = b"dark_trades_users";

pub const ANCHOR_DISCRIMINATOR: usize = 8;
