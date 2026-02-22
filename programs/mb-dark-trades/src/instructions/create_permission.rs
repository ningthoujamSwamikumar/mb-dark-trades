use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{
    access_control::{
        instructions::CreatePermissionCpiBuilder,
        structs::{Member, MembersArgs},
    },
    consts::PERMISSION_PROGRAM_ID,
};

use crate::{instructions::derive_seeds_from_account_type, AccountType};

#[derive(Accounts)]
pub struct CreatePermission<'info> {
    /// CHECK: Validated via permission program CPI
    pub permissioned_account: UncheckedAccount<'info>,

    /// CHECK: Checked by permission program
    #[account(mut)]
    pub permission: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: PERMISSION PROGRAM
    #[account(
        address = PERMISSION_PROGRAM_ID
    )]
    pub permission_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl CreatePermission<'_> {
    /// Creates a permission based on account type input.
    /// Derives the bump from the account type and seeds, then calls the permission program.
    pub fn handler(&self, account_type: AccountType, members: Option<Vec<Member>>) -> Result<()> {
        let seed_data = derive_seeds_from_account_type(&account_type);
        let mut seed_refs: Vec<&[u8]> = seed_data.iter().map(|s| s.as_slice()).collect();

        let (_, bump) = Pubkey::find_program_address(&seed_refs, &crate::ID);
        let bump_slice = &[bump];
        seed_refs.push(bump_slice);

        CreatePermissionCpiBuilder::new(&self.permission_program)
            .permissioned_account(&self.permissioned_account.to_account_info())
            .permission(&self.permission)
            .payer(&self.payer)
            .system_program(&self.system_program)
            .args(MembersArgs { members })
            .invoke_signed(&[seed_refs.as_slice()])?;

        Ok(())
    }
}
