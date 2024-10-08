use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{MarketConfig, MARKET_CONFIG_SEED, MARKET_FEEVAULT_SEED};

#[derive(Accounts)]
pub struct CollectFee<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [MARKET_CONFIG_SEED],
        bump = market_config.bump,
        has_one = authority
    )]
    pub market_config: Account<'info, MarketConfig>,
    #[account(
        mut,
        seeds = [MARKET_FEEVAULT_SEED, market_config.key().as_ref()],
        bump = market_config.fee_vault_bump
    )]
    pub market_fee_vault: SystemAccount<'info>,
    #[account(mut)]
    pub destination: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CollectFee<'info> {
    pub fn handler(&mut self) -> Result<()> {
        let seeds = &[
            MARKET_FEEVAULT_SEED,
            self.market_config.to_account_info().key.as_ref(),
            &[self.market_config.fee_vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let lamports = self.market_fee_vault.lamports();

        transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.market_fee_vault.to_account_info(),
                    to: self.destination.to_account_info(),
                },
                signer_seeds,
            ),
            lamports,
        )?;
        Ok(())
    }
}
