use anchor_lang::prelude::*;

use crate::error::MarketErrorCode;

#[account]
#[derive(InitSpace)]
pub struct MarketConfig {
    pub authority: Pubkey,
    pub fee_vault: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub fee_vault_bump: u8,
}

impl MarketConfig {
    pub fn check_fee(&self, fee: u16) -> Result<()> {
        require!(fee.ge(&0) && fee.le(&10_000), MarketErrorCode::InvalidFee);
        Ok(())
    }
}
