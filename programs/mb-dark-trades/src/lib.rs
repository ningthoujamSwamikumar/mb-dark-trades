pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

use ephemeral_rollups_sdk::access_control::structs::Member;
use ephemeral_rollups_sdk::anchor::ephemeral;

declare_id!("65GJ4WBAz7oF5TdPH8tJpsLkmq2AwT92pQAyi5decD1U");

#[ephemeral]
#[program]
pub mod mb_dark_trades {
    use super::*;

    pub fn place_intent(
        ctx: Context<PlaceIntent>,
        id: u64,
        intent_side: u8,
        deposit_amount: u64,
        quantity: u64,
        limit_price: u64,
        expiry: i64,
    ) -> Result<()> {
        ctx.accounts.handler(
            id,
            intent_side,
            deposit_amount,
            quantity,
            limit_price,
            expiry,
            ctx.bumps,
        )
    }

    pub fn create_permission(
        ctx: Context<CreatePermission>,
        account_type: AccountType,
        members: Option<Vec<Member>>,
    ) -> Result<()> {
        ctx.accounts.handler(account_type, members)
    }

    pub fn delegate_pda(ctx: Context<DelegatePda>, account_type: AccountType) -> Result<()> {
        ctx.accounts.handler(account_type)
    }

    pub fn match_intent(ctx: Context<MatchOrders>) -> Result<()> {
        ctx.accounts.handler()
    }

    pub fn settle_and_undelegate(ctx: Context<SettleAndUndelegate>) -> Result<()> {
        ctx.accounts.handler()
    }

    pub fn withdraw(ctx: Context<Withdraw>, id: u64, intent_side: u8) -> Result<()> {
        ctx.accounts.handler(id, intent_side)
    }
}
