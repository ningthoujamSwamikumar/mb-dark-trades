use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{DarkConfig, ANCHOR_DISCRIMINATOR, DARK_CONFIG_SEEDS};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + DarkConfig::INIT_SPACE,
        seeds = [DARK_CONFIG_SEEDS],
        bump,
        has_one = admin,
    )]
    pub config: Account<'info, DarkConfig>,

    pub mint_a: Account<'info, Mint>,

    pub mint_b: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_a,
        associated_token::authority = config,
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_b,
        associated_token::authority = config,
    )]
    pub vault_b: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.config.set_inner(DarkConfig {
        admin: ctx.accounts.admin.key(),
        bump: ctx.bumps.config,
    });

    Ok(())
}
