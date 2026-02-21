use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, TransferChecked, transfer_checked};

use crate::{ANCHOR_DISCRIMINATOR, DARK_CONFIG_SEEDS, DarkConfig, USER_ACCOUNT_SEEDS, UserAccount};

#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(mut)]
    pub depositor: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [DARK_CONFIG_SEEDS],
        bump = config.bump,
    )]
    pub config: Account<'info, DarkConfig>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = config, 
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = config,
    )]
    pub depositor_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = ANCHOR_DISCRIMINATOR + UserAccount::INIT_SPACE,
        seeds = [USER_ACCOUNT_SEEDS, mint.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl Deposit<'_> {
    pub fn handler(&mut self, amount: u64, bump: u8)->Result<()>{
        
        if !self.user_account.is_initialized {
            self.user_account.set_inner(UserAccount { mint: self.mint.key(), bump, amount, is_initialized: true });
        }else {
            self.user_account.amount += amount;
            self.user_account.bump = bump;
        };

        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(), 
                TransferChecked {
                            from: self.depositor_ata.to_account_info(),
                            authority: self.depositor.to_account_info(),
                            mint: self.mint.to_account_info(),
                            to: self.vault.to_account_info(),
                }
            ), 
            amount, 
            self.mint.decimals
        )
    }
}
