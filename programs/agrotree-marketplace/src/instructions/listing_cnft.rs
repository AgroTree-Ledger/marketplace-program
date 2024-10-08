use crate::{
    error::MarketErrorCode, CnftArgs, CompressedListing, ListingCnftArgs, COMPRESSED_LISTING_SEED,
};
use anchor_lang::prelude::*;
use mpl_bubblegum::{
    instructions::TransferCpiBuilder,
    programs::{MPL_BUBBLEGUM_ID, SPL_ACCOUNT_COMPRESSION_ID, SPL_NOOP_ID},
    utils::get_asset_id,
};

#[derive(Accounts)]
#[instruction(args: ListingCnftArgs)]
pub struct ListingCnft<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        init,
        payer = seller,
        space = 8 + CompressedListing::INIT_SPACE,
        seeds = [COMPRESSED_LISTING_SEED, seller.key.as_ref() , merkle_tree.key().as_ref(), args.asset_id.as_ref()],
        bump
    )]
    pub compressed_listing: Box<Account<'info, CompressedListing>>,
    /// CHECK: will used by mpl_bubblegum program
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = mpl_bubblegum_program.key()
    )]
    pub tree_config: UncheckedAccount<'info>,
    /// CHECK: Zero initialized account
    #[account(mut)]
    pub merkle_tree: AccountInfo<'info>,
    pub leaf_owner: SystemAccount<'info>,
    pub leaf_delegate: SystemAccount<'info>,
    /// CHECK: Safe. Bubblegum program.
    #[account(address = MPL_BUBBLEGUM_ID)]
    pub mpl_bubblegum_program: UncheckedAccount<'info>,
    /// CHECK: Safe. Compression program.
    #[account(address = SPL_ACCOUNT_COMPRESSION_ID)]
    pub spl_compression_program: UncheckedAccount<'info>,
    /// CHECK: Safe. Log wrapper program.
    #[account(address = SPL_NOOP_ID)]
    pub log_wrapper_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ListingCnft<'info> {
    pub fn handler(
        &mut self,
        args: ListingCnftArgs,
        remaining_accounts: Vec<(&AccountInfo<'info>, bool, bool)>,
        bumps: &ListingCnftBumps,
    ) -> Result<()> {
        let asset_id = get_asset_id(self.merkle_tree.key, args.cnft_args.nonce);

        require!(args.asset_id == asset_id, MarketErrorCode::InvalidAssetId);
        require!(args.price.gt(&0), MarketErrorCode::InvalidPrice);
        self.compressed_listing.set_inner(CompressedListing {
            seller: self.seller.key(),
            merkle_tree: self.merkle_tree.key(),
            price: args.price,
            asset_id: args.asset_id,
            bump: bumps.compressed_listing,
        });

        self.transfer_to_vault(
            CnftArgs {
                root: args.cnft_args.root,
                data_hash: args.cnft_args.data_hash,
                creator_hash: args.cnft_args.creator_hash,
                nonce: args.cnft_args.nonce,
                index: args.cnft_args.index,
            },
            remaining_accounts,
        )?;
        Ok(())
    }

    fn transfer_to_vault(
        &self,
        args: CnftArgs,
        remaining_accounts: Vec<(&AccountInfo<'info>, bool, bool)>,
    ) -> Result<()> {
        let merkle_tree = self.merkle_tree.to_account_info();
        let compressed_listing = self.compressed_listing.to_account_info();

        TransferCpiBuilder::new(&self.mpl_bubblegum_program)
            .tree_config(&self.tree_config)
            .leaf_owner(&self.seller, false)
            .leaf_delegate(&self.seller.to_account_info(), true)
            .new_leaf_owner(&compressed_listing)
            .merkle_tree(&merkle_tree)
            .log_wrapper(&self.log_wrapper_program)
            .system_program(&self.system_program)
            .compression_program(&self.spl_compression_program)
            .root(args.root)
            .data_hash(args.data_hash)
            .creator_hash(args.creator_hash)
            .nonce(u64::from(args.nonce))
            .index(args.index)
            .add_remaining_accounts(&remaining_accounts[..])
            .invoke()?;

        Ok(())
    }
}
