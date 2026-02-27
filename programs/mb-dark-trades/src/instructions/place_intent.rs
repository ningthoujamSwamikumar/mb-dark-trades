use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::{
    DepositAccount, IntentAccount, ANCHOR_DISCRIMINATOR, DEPOSIT_ACCOUNT_SEEDS,
    INTENT_ACCOUNT_SEEDS,
};

#[derive(Accounts)]
#[instruction(id: u64, intent_side: u8)]
pub struct PlaceIntent<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_usdc_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + DepositAccount::INIT_SPACE,
        seeds = [DEPOSIT_ACCOUNT_SEEDS, &[intent_side] , user.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub deposit: Account<'info, DepositAccount>,

    #[account(
        init,
        payer = user,
        associated_token::mint = usdc_mint,
        associated_token::authority = deposit,
        associated_token::token_program = token_program,
    )]
    pub deposit_usdc_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + IntentAccount::INIT_SPACE,
        seeds = [INTENT_ACCOUNT_SEEDS, &[intent_side], user.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub intent_account: Account<'info, IntentAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl PlaceIntent<'_> {
    pub fn handler(
        &mut self,
        id: u64,
        side: u8,
        deposit_amount: u64,
        trade_quantity: u64,
        limit_price: u64,
        expiry: i64,
        bumps: PlaceIntentBumps,
    ) -> Result<()> {
        self.deposit.set_inner(DepositAccount {
            id,
            bump: bumps.deposit,
            amount: trade_quantity,
        });

        self.intent_account.set_inner(IntentAccount {
            id,
            owner: self.user.key(),
            side,
            quantity: trade_quantity,
            limit_price,
            expiry,
            is_matched: false,
        });

        // if the intent is to buy, deposit usdc
        if side == 0 {
            transfer_checked(
                CpiContext::new(
                    self.token_program.to_account_info(),
                    TransferChecked {
                        authority: self.user.to_account_info(),
                        from: self.user_usdc_ata.to_account_info(),
                        mint: self.usdc_mint.to_account_info(),
                        to: self.deposit_usdc_vault.to_account_info(),
                    },
                ),
                deposit_amount, // user has to be responsible to make enough deposits for the trade
                self.usdc_mint.decimals,
            )?;
        } else {
            // if the intent is to sell, deposit sol
            transfer(
                CpiContext::new(
                    self.system_program.to_account_info(),
                    Transfer {
                        from: self.user.to_account_info(),
                        to: self.deposit.to_account_info(),
                    },
                ),
                deposit_amount,
            )?;
        }

        Ok(())
    }
}
