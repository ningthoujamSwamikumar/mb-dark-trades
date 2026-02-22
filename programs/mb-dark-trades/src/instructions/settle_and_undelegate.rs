use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use ephemeral_rollups_sdk::{anchor::commit, ephem::commit_and_undelegate_accounts};

use crate::{error::DarkTradeErrors, DepositAccount, IntentAccount, MATCHER_PUBKEY};

#[commit]
#[derive(Accounts)]
pub struct SettleAndUndelegate<'info> {
    #[account(mut, address = MATCHER_PUBKEY)]
    pub matcher: Signer<'info>,

    #[account(
        mut,
        constraint = intent.is_matched == true @ DarkTradeErrors::OrderNotComplete
    )]
    pub intent: Account<'info, IntentAccount>,

    #[account(mut)]
    pub deposit: Account<'info, DepositAccount>,

    #[account(mut)]
    pub usdc_vault: Account<'info, TokenAccount>,

    /// CHECK: The MagicBlock Delegation Program
    pub delegation_program: UncheckedAccount<'info>,
}

impl SettleAndUndelegate<'_> {
    pub fn handler(&self) -> Result<()> {
        commit_and_undelegate_accounts(
            &self.matcher.to_account_info(),
            vec![
                &self.intent.to_account_info(),
                &self.deposit.to_account_info(),
                &self.usdc_vault.to_account_info(),
            ],
            &self.magic_context,
            &self.magic_program,
        )?;

        msg!("Order {} settled and returned to L1", self.intent.key());
        Ok(())
    }
}
