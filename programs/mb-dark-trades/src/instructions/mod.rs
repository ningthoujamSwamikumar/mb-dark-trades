pub mod create_permission;
pub mod delegate_pda;
pub mod initialize;
pub mod match_orders;
pub mod place_intent;
pub mod settle_and_undelegate;
pub mod withdraw;

pub use create_permission::*;
pub use delegate_pda::*;
pub use match_orders::*;
pub use place_intent::*;
pub use settle_and_undelegate::*;
pub use withdraw::*;

use anchor_lang::prelude::*;

use crate::{DEPOSIT_ACCOUNT_SEEDS, INTENT_ACCOUNT_SEEDS};

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum AccountType {
    DepositAccount { side: u8, user: Pubkey, id: u64 },
    Intent { side: u8, user: Pubkey, id: u64 },
}

/// Single source of all the PDA account seeds
fn derive_seeds_from_account_type(account_type: &AccountType) -> Vec<Vec<u8>> {
    match account_type {
        AccountType::Intent { side, user, id } => vec![
            INTENT_ACCOUNT_SEEDS.to_vec(),
            vec![*side],
            user.key().to_bytes().to_vec(),
            id.to_le_bytes().to_vec(),
        ],
        AccountType::DepositAccount { side, user, id } => vec![
            DEPOSIT_ACCOUNT_SEEDS.to_vec(),
            vec![*side],
            user.key().to_bytes().to_vec(),
            id.to_le_bytes().to_vec(),
        ],
    }
}
