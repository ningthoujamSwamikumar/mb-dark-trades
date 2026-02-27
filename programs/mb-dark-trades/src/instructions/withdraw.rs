use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{DEPOSIT_ACCOUNT_SEEDS, DepositAccount, INTENT_ACCOUNT_SEEDS, IntentAccount, error::DarkTradeErrors};

#[derive(Accounts)]
#[instruction(id: u64, intent_side: u8)]
pub struct Withdraw<'info> {
    // owner
    #[account(mut)]
    pub owner: Signer<'info>,
    // receiver sol account
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
    // receiver usdc account
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = receiver,
        associated_token::token_program = token_program
    )]
    pub receiver_usdc: Account<'info, TokenAccount>,
    // usdc mint
    pub usdc_mint: Account<'info, Mint>,
    // deposit account
    #[account(
        mut,
        seeds = [DEPOSIT_ACCOUNT_SEEDS, &[intent_side], owner.key().as_ref(), &id.to_le_bytes()],
        bump = deposit.bump,
        close = owner
    )]
    pub deposit: Account<'info, DepositAccount>,
    // usdc vault
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = deposit,
        associated_token::token_program = token_program,
        close = owner
    )]
    pub usdc_vault: Account<'info, TokenAccount>,

    #[account(
        mut, 
        seeds = [INTENT_ACCOUNT_SEEDS, &[intent_side], owner.key().as_ref(), &id.to_le_bytes()],
        bump,
        close = owner,
    )]
    pub intent: Account<'info, IntentAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl Withdraw<'_> {
    pub fn handler(&self, id: u64, intent_side: u8) -> Result<()> {
        // ensure intent is either matched or expired
        let now = Clock::get()?.unix_timestamp;
        require!(self.intent.expiry > now || self.intent.is_matched, DarkTradeErrors::OrderNotComplete);

        // 1. Prepare seeds for the PDA signer
        let owner_key = self.owner.key();
        let id_bytes = id.to_le_bytes();
        let seeds = &[
            DEPOSIT_ACCOUNT_SEEDS,
            &[intent_side],
            owner_key.as_ref(),
            &id_bytes,
            &[self.deposit.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        // 2. Transfer USDC from vault to receiver
        let cpi_accounts = anchor_spl::token::Transfer {
            from: self.usdc_vault.to_account_info(),
            to: self.receiver_usdc.to_account_info(),
            authority: self.deposit.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        anchor_spl::token::transfer(cpi_ctx, self.usdc_vault.amount)?;

        // 3. Transfer SOL from deposit PDA to receiver
        // We do this by moving the lamports.
        // Note: Since 'close = owner' is in the macro, the remaining lamports
        // will automatically go to the 'owner' (Signer).
        // If you want them to go to 'receiver', we manually transfer them here:
        let amount_to_transfer = self.deposit.get_lamports();
        **self.deposit.to_account_info().try_borrow_mut_lamports()? -= amount_to_transfer;
        **self.receiver.to_account_info().try_borrow_mut_lamports()? += amount_to_transfer;

        Ok(())
    }
}
