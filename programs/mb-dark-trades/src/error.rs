use anchor_lang::prelude::*;

#[error_code]
pub enum DarkTradeErrors {
    #[msg("Custom error message")]
    CustomError,
    AlreadyMatched,
    MintMismatch,
    NoMatchFound,
    IntentExpired,
    ArithmeticError,
    InsufficientFunds,
    OrderNotComplete
}
