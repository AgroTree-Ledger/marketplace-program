use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{
    error::MarketErrorCode, BuyCnftArgs, CnftArgs, CompressedListing, MarketConfig, COMPRESSED_LISTING_SEED, MARKET_CONFIG_SEED, MARKET_FEEVAULT_SEED
};
use mpl_bubblegum::{
    instructions::TransferCpiBuilder,
    programs::{MPL_BUBBLEGUM_ID, SPL_ACCOUNT_COMPRESSION_ID, SPL_NOOP_ID},
    utils::get_asset_id,
};

#[derive(Accounts)]
#[instruction(args: BuyCnftArgs)]
pub struct BuyCnft<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
      mut,
      address = compressed_listing.seller,
    )]
    pub seller: SystemAccount<'info>,
    #[account(
      seeds = [MARKET_CONFIG_SEED],
      bump = market_config.bump,
    )]
    pub market_config: Box<Account<'info, MarketConfig>>,
    #[account(
      mut, 
      seeds = [MARKET_FEEVAULT_SEED, market_config.key().as_ref()],
      bump = market_config.fee_vault_bump
    )]
    pub market_fee_vault: SystemAccount<'info>,
    #[account(
      mut,
      seeds = [COMPRESSED_LISTING_SEED, seller.key().as_ref(), merkle_tree.key().as_ref(), args.asset_id.as_ref()],
      bump = compressed_listing.bump,
      constraint = compressed_listing.asset_id.eq(&args.asset_id) @MarketErrorCode::InvalidCompressedListingAssetId,
      close = seller
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
    #[account(
      mut,
      address = compressed_listing.merkle_tree,
    )]
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

impl<'info> BuyCnft<'info> {
    pub fn handler(&mut self, args: BuyCnftArgs, remaining_accounts: Vec<(&AccountInfo<'info>, bool, bool)>) -> Result<()> {

      let asset_id = get_asset_id(self.merkle_tree.key, args.cnft_args.nonce);

      require!(args.asset_id == asset_id, MarketErrorCode::InvalidAssetId);

      let fee_rate = self.market_config.fee;
      let (lamports, fee) = self.compressed_listing.calculate_fee(fee_rate)?;

      msg!("fee: {}", fee);
      msg!("lamports: {}", lamports);

      self.transfer_sol(self.seller.to_account_info(),lamports)?;

      if fee.gt(&0) {
        self.transfer_sol(self.market_fee_vault.to_account_info(),fee)?;
      }

      self.transfer_cnft_to_buyer(
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

    fn transfer_sol(&self, destination: AccountInfo<'info>, amount:u64) -> Result<()> {
      transfer(CpiContext::new(self.system_program.to_account_info(), Transfer {
        from: self.buyer.to_account_info(),
        to: destination,
      }), amount)?;
      Ok(())
    }


  

    fn transfer_cnft_to_buyer(
      &self,
      args: CnftArgs,
      remaining_accounts: Vec<(&AccountInfo<'info>, bool, bool)>,
    ) -> Result<()> {
      let asset_id = get_asset_id(self.merkle_tree.key, args.nonce);
      let seeds = &[
          COMPRESSED_LISTING_SEED,
          self.seller.key.as_ref(),
          self.merkle_tree.key.as_ref(),
          asset_id.as_ref(),
          &[self.compressed_listing.bump],
      ];

      let signer_seeds = &[&seeds[..]];

      let merkle_tree = self.merkle_tree.to_account_info();
      let compressed_listing = self.compressed_listing.to_account_info();

      TransferCpiBuilder::new(&self.mpl_bubblegum_program)
          .tree_config(&self.tree_config)
          .leaf_owner(&compressed_listing, true)
          .leaf_delegate(&compressed_listing, true)
          .new_leaf_owner(&self.buyer)
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
          .invoke_signed(signer_seeds)?;

      Ok(())
  }
}
