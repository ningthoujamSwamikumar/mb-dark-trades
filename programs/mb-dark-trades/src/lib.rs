pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("65GJ4WBAz7oF5TdPH8tJpsLkmq2AwT92pQAyi5decD1U");

#[program]
pub mod mb_dark_trades {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.handler(amount, ctx.bumps.user_account)
    }
}
