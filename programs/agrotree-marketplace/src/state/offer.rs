use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub maker: Pubkey,
    pub seller: Pubkey,
    pub merkle_tree: Pubkey,
    pub asset_id: Pubkey,
    pub price: u64,
    pub bump: u8,
}
