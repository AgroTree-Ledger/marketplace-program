pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EFw2geYiUUZup7tWqXFn5voceBPPPMttQvkLZ42ZD3WD");

#[program]
pub mod agrotree_marketplace {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u16) -> Result<()> {
        ctx.accounts.handler(fee, &ctx.bumps)
    }

    pub fn collect_fee<'info>(ctx: Context<CollectFee>) -> Result<()> {
        ctx.accounts.handler()?;
        Ok(())
    }

    pub fn listing_cnft<'info>(
        ctx: Context<'_, '_, '_, 'info, ListingCnft<'info>>,
        args: ListingCnftArgs,
    ) -> Result<()> {
        let remaining_accounts: Vec<(&AccountInfo<'info>, bool, bool)> = ctx
            .remaining_accounts
            .iter()
            .map(|account| (account, account.is_writable, account.is_signer)) // Do not dereference here
            .collect();

        ctx.accounts.handler(args, remaining_accounts, &ctx.bumps)?;

        Ok(())
    }

    pub fn unlist_cnft<'info>(
        ctx: Context<'_, '_, '_, 'info, UnlistCnft<'info>>,
        args: UnlistCnftArgs,
    ) -> Result<()> {
        let remaining_accounts: Vec<(&AccountInfo<'info>, bool, bool)> = ctx
            .remaining_accounts
            .iter()
            .map(|account| (account, account.is_writable, account.is_signer)) // Do not dereference here
            .collect();

        ctx.accounts.handler(args, remaining_accounts)?;

        Ok(())
    }

    pub fn buy_cnft<'info>(
        ctx: Context<'_, '_, '_, 'info, BuyCnft<'info>>,
        args: BuyCnftArgs,
    ) -> Result<()> {
        let remaining_accounts: Vec<(&AccountInfo<'info>, bool, bool)> = ctx
            .remaining_accounts
            .iter()
            .map(|account| (account, account.is_writable, account.is_signer)) // Do not dereference here
            .collect();

        ctx.accounts.handler(args, remaining_accounts)?;

        Ok(())
    }
}
