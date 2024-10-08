use anchor_lang::prelude::*;

use crate::{MarketConfig, MARKET_CONFIG_SEED, MARKET_FEEVAULT_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + MarketConfig::INIT_SPACE,
        seeds = [MARKET_CONFIG_SEED],
        bump
    )]
    pub market_config: Account<'info, MarketConfig>,
    #[account(
        seeds = [MARKET_FEEVAULT_SEED, market_config.key().as_ref()],
        bump
    )]
    pub market_fee_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn handler(&mut self, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        self.market_config.check_fee(fee)?;

        self.market_config.set_inner(MarketConfig {
            authority: self.authority.key(),
            fee_vault: self.market_fee_vault.key(),
            bump: bumps.market_config,
            fee_vault_bump: bumps.market_fee_vault,
            fee,
        });

        Ok(())
    }
}
