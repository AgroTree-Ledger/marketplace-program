use anchor_lang::prelude::*;

#[error_code]
pub enum MarketErrorCode {
    Unauthorized,
    MathError,
    InvalidFee,
    InvalidAssetId,
    InvalidPrice,
    InvalidCompressedListingSeller,
    InvalidCompressedListingMerkleTree,
    InvalidCompressedListingAssetId,
}
