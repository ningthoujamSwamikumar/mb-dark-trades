use anchor_lang::prelude::{program_pack::Pack, *};
use anchor_spl::token::{spl_token::state::Account as RawTokenAccount, Mint, Token, TokenAccount};

use crate::{
    error::DarkTradeErrors, DepositAccount, IntentAccount, DEPOSIT_ACCOUNT_SEEDS, MATCHER_PUBKEY,
    USDC_MINT,
};

#[derive(Accounts)]
#[instruction(sell_id: u64, buy_id: u64)]
pub struct MatchOrders<'info> {
    /// The Matcher Bot must sign to trigger the TEE execution
    #[account(signer, address = MATCHER_PUBKEY)]
    pub matcher: Signer<'info>,

    #[account(address = USDC_MINT)]
    pub usdc_mint: Account<'info, Mint>,

    pub seller: SystemAccount<'info>,

    pub buyer: SystemAccount<'info>,

    #[account(mut)]
    pub seller_intent: Account<'info, IntentAccount>,

    #[account(mut)]
    pub buyer_intent: Account<'info, IntentAccount>,

    /// this is the pda which stores sol deposited, as well as the deposit information
    #[account(
        mut,
        seeds = [DEPOSIT_ACCOUNT_SEEDS, &[1], seller.key().as_ref(), &sell_id.to_le_bytes()],
        bump = seller_sol_deposit.bump,
    )]
    pub seller_sol_deposit: Account<'info, DepositAccount>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = seller_sol_deposit,
        associated_token::token_program = token_program,
    )]
    pub seller_usdc_vault: Account<'info, TokenAccount>,

    /// this is be the sol receiver pda account
    #[account(
        mut,
        seeds = [DEPOSIT_ACCOUNT_SEEDS, &[0], buyer.key().as_ref(), &buy_id.to_le_bytes()],
        bump = buyer_usdc_deposit.bump
    )]
    pub buyer_usdc_deposit: Account<'info, DepositAccount>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = buyer_usdc_deposit,
        associated_token::token_program = token_program,
    )]
    pub buyer_usdc_vault: Account<'info, TokenAccount>,

    // may need to close the deposit pda accounts
    /// CHECK: The Pyth Lazer Price Feed PDA (derived via seeds)
    pub pyth_price_feed: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

impl MatchOrders<'_> {
    pub fn handler(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let seller_intent = &mut self.seller_intent;
        let buyer_intent = &mut self.buyer_intent;

        // 1. Validation Checks
        require!(
            !seller_intent.is_matched && !buyer_intent.is_matched,
            DarkTradeErrors::AlreadyMatched
        );
        require!(
            now < seller_intent.expiry && now < buyer_intent.expiry,
            DarkTradeErrors::IntentExpired
        );

        // Fetch the price from "Pyth Lazer Price"
        // We assume the Price Feed PDA is passed correctly by the matcher
        let price_data = self.pyth_price_feed.try_borrow_data()?;
        let current_price = i64::from_le_bytes(price_data[73..81].try_into().unwrap()) as u64;

        // Match Verification (Midpoint Crossing)
        // we assume party 'a' is the buyer, and party 'b' is the seller
        if seller_intent.limit_price <= current_price && buyer_intent.limit_price >= current_price {
            // Calculate the trade quantity (keeping it simple for now)
            let trade_quantity = std::cmp::min(seller_intent.quantity, buyer_intent.quantity);
            // trade value in usdc (6 decimals), for trade quantity in lamports (1 sol = 9 decimals)
            let trade_value = trade_quantity
                .checked_mul(current_price)
                .ok_or(DarkTradeErrors::ArithmeticError)?
                .checked_div(1_000_000_000)
                .ok_or(DarkTradeErrors::ArithmeticError)?;

            // Execute the "Dark Swap"

            // remove from sol from seller pda, and add sol to buyer pda
            let seller_account_info = self.seller_sol_deposit.to_account_info();
            let buyer_account_info = self.buyer_usdc_deposit.to_account_info();
            **seller_account_info.try_borrow_mut_lamports()? -= trade_quantity;
            **buyer_account_info.try_borrow_mut_lamports()? += trade_quantity;

            // remove usdc from buyer usdc vault, and add to seller usdc vault
            let buyer_usdc_account_info = self.buyer_usdc_vault.to_account_info();
            let seller_usdc_account_info = self.seller_usdc_vault.to_account_info();

            // remove usdc from buyer
            {
                let mut data = buyer_usdc_account_info.try_borrow_mut_data()?;
                let mut account = RawTokenAccount::unpack(&data[..])?;

                account.amount = account
                    .amount
                    .checked_sub(trade_value)
                    .ok_or(DarkTradeErrors::InsufficientFunds)?;

                RawTokenAccount::pack(account, &mut data)?;
            }

            // add usdc to seller
            {
                let mut data = seller_usdc_account_info.try_borrow_mut_data()?;
                let mut account = RawTokenAccount::unpack(&data[..])?;

                account.amount = account
                    .amount
                    .checked_add(trade_value)
                    .ok_or(DarkTradeErrors::InsufficientFunds)?;

                RawTokenAccount::pack(account, &mut data)?;
            }

            // update intents
            seller_intent.quantity -= trade_quantity;
            buyer_intent.quantity -= trade_quantity;

            // Mark as matched
            if seller_intent.quantity == 0 {
                seller_intent.is_matched = true;
            }
            if buyer_intent.quantity == 0 {
                buyer_intent.is_matched = true;
            }

            msg!(
                "Dark Match! Quantity: {} @ Price: {}",
                trade_quantity,
                current_price
            );
        } else {
            return Err(DarkTradeErrors::NoMatchFound.into());
        }

        Ok(())
    }
}
