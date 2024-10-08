use anchor_lang::prelude::*;

use crate::error::MarketErrorCode;

#[account]
#[derive(InitSpace)]
pub struct CompressedListing {
    pub seller: Pubkey,
    pub merkle_tree: Pubkey,
    pub asset_id: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl CompressedListing {
    pub fn calculate_fee(&self, fee_rate: u16) -> Result<(u64, u64)> {
        let fee = self
            .price
            .checked_mul(fee_rate as u64)
            .ok_or(MarketErrorCode::MathError)?
            .checked_div(10_000)
            .ok_or(MarketErrorCode::MathError)?;

        let lamports = self
            .price
            .checked_sub(fee)
            .ok_or(MarketErrorCode::MathError)?;

        Ok((lamports, fee))
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct ListingCnftArgs {
    pub asset_id: Pubkey,
    pub price: u64,
    pub cnft_args: CnftArgs,
    // pub root: [u8; 32],
    // pub data_hash: [u8; 32],
    // pub creator_hash: [u8; 32],
    // pub nonce: u64,
    // pub index: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct UnlistCnftArgs {
    pub asset_id: Pubkey,
    pub cnft_args: CnftArgs,
    // pub root: [u8; 32],
    // pub data_hash: [u8; 32],
    // pub creator_hash: [u8; 32],
    // pub nonce: u64,
    // pub index: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct BuyCnftArgs {
    pub asset_id: Pubkey,
    pub cnft_args: CnftArgs,
    // pub root: [u8; 32],
    // pub data_hash: [u8; 32],
    // pub creator_hash: [u8; 32],
    // pub nonce: u64,
    // pub index: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub struct CnftArgs {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}
