use anchor_lang::prelude::*;

#[constant]
pub const DARK_CONFIG_SEEDS: &[u8] = b"dark_trades_config";

#[constant]
pub const DEPOSIT_ACCOUNT_SEEDS: &[u8] = b"dark_trades_deposits";

#[constant]
pub const INTENT_ACCOUNT_SEEDS: &[u8] = b"dark_trades_intents";

#[constant]
pub const MATCHER_PUBKEY: Pubkey =
    Pubkey::from_str_const("Bgsbppow2TFYGs71xohv3aNdDbQGoXGZ7ez1VBLc8kp9");

pub const ANCHOR_DISCRIMINATOR: usize = 8;
